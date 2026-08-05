use std::{
    collections::{HashMap, HashSet},
    io::Write,
    path::PathBuf,
    sync::Arc,
};

use melior::{
    Context,
    ir::{Module, operation::OperationLike},
    pass::{
        PassManager,
        conversion::{create_scf_to_control_flow, create_to_llvm},
        transform::create_canonicalizer,
    },
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use std::{fs, path::Path};

use crate::{
    MathicError, MathicResult,
    codegen::{MathicCodeGen, compiler_helper::CompilerHelper},
    diagnostics::{CodegenError, CompilationError, DiagnosticsManager, LoweringError},
    ffi::{
        self,
        dialect_integration::symbolic_dialect::{
            create_symbolic_extract_eval, create_symbolic_to_arith,
        },
    },
    lowering::{self, ir::Ir, lower_program},
    parser::{
        MathicParser,
        ast::{
            MathicModule,
            declaration::{Path as MathicPath, TopLevelItem},
        },
    },
};

#[derive(Debug, Clone, Copy, Default)]
pub struct CompilerOpts {
    pub opt_lvl: OptLvl,
    pub dump_mathir: bool,
    pub dump_mlir: bool,
    pub dump_llvmir: bool,
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(u8)]
pub enum OptLvl {
    None,
    O1,
    #[default]
    O2,
    O3,
}

impl From<usize> for OptLvl {
    fn from(value: usize) -> Self {
        match value {
            0 => OptLvl::None,
            1 => OptLvl::O1,
            2 => OptLvl::O2,
            _ => OptLvl::O3,
        }
    }
}

impl From<OptLvl> for usize {
    fn from(val: OptLvl) -> Self {
        match val {
            OptLvl::None => 0,
            OptLvl::O1 => 1,
            OptLvl::O2 => 2,
            OptLvl::O3 => 3,
        }
    }
}

impl From<u8> for OptLvl {
    fn from(value: u8) -> Self {
        match value {
            0 => OptLvl::None,
            1 => OptLvl::O1,
            2 => OptLvl::O2,
            _ => OptLvl::O3,
        }
    }
}

pub struct MathicCompiler {
    ctx: Context,
    diagnostics: DiagnosticsManager,
}

unsafe impl Send for MathicCompiler {}
unsafe impl Sync for MathicCompiler {}

impl MathicCompiler {
    pub fn new() -> Result<Self, CodegenError> {
        Ok(Self {
            ctx: ffi::create_context()?,
            diagnostics: DiagnosticsManager::new(),
        })
    }

    pub fn diagnostics(&self) -> &DiagnosticsManager {
        &self.diagnostics
    }

    pub fn compile_project(
        &self,
        src_root: &Path,
        compiler_options: CompilerOpts,
    ) -> MathicResult<Vec<Module<'_>>> {
        self.diagnostics.clear()?;

        let main_file_path = PathBuf::from("main.mth");

        let compilation_unit = {
            let mut parsed_files = HashSet::new();
            self.parse_file(src_root, main_file_path, &mut parsed_files)?
        };

        if self.diagnostics.has_errors()? {
            return Err(MathicError::CompilationFailed);
        }

        let irs = compilation_unit
            .into_par_iter()
            .map(|(path, p)| (path, lower_program(&p)))
            .collect::<Vec<_>>();

        let mut lowered = Vec::with_capacity(irs.len());

        for (path, ir) in irs {
            match ir {
                Ok(ir) => lowered.push((path, ir)),
                Err(e) => self
                    .diagnostics
                    .report(path, CompilationError::Lowering(e))?,
            }
        }

        if self.diagnostics.has_errors()? {
            return Err(MathicError::CompilationFailed);
        }

        let mut modules = Vec::with_capacity(lowered.len());

        for (path, ir) in lowered {
            modules.push(self.compile_module(&ir, src_root, path, compiler_options)?);
        }

        if self.diagnostics.has_errors()? {
            return Err(MathicError::CompilationFailed);
        }

        Ok(modules)
    }

    pub fn compile_module<'func>(
        &'func self,
        ir: &Ir,
        src_root: &Path,
        file_path: PathBuf,
        compiler_options: CompilerOpts,
    ) -> MathicResult<Module<'func>> {
        let relative_path = file_path
            .strip_prefix(src_root)
            .unwrap_or(&file_path)
            .to_owned();

        if compiler_options.dump_mathir {
            let mathir_path = PathBuf::from("mathir_dumps")
                .join(&relative_path)
                .with_extension("mathir");

            fs::create_dir_all(mathir_path.parent().unwrap())?;

            let mut f_mathir = fs::File::create(mathir_path)?;

            write!(f_mathir, "{}", ir)?;
        }

        // Generate Module.
        let mut module = match ffi::create_module(&self.ctx, compiler_options.opt_lvl) {
            Ok(module) => module,
            Err(e) => {
                return Err(self
                    .diagnostics
                    .report_and_fail(file_path, CompilationError::Codegen(e)));
            }
        };

        {
            let codegen = MathicCodeGen::new(&self.ctx, ir, &module, Some(file_path.clone()));
            let mut helper = CompilerHelper::new();

            if let Err(e) = codegen.generate_module(&mut helper) {
                return Err(self
                    .diagnostics
                    .report_and_fail(file_path, CompilationError::Codegen(e)));
            }
        }

        if compiler_options.dump_mlir {
            let mlir_path = PathBuf::from("mlir_dumps")
                .join(format!("{}-dump-prepass", relative_path.display()))
                .with_extension("mlir");

            fs::create_dir_all(mlir_path.parent().unwrap())?;

            let mut f_prepass_program = fs::File::create(mlir_path)?;

            write!(f_prepass_program, "{}", module.as_operation())?;
        }

        debug_assert!(module.as_operation().verify());
        tracing::debug!("Module crated successfully");

        // Run Passes to the generated module.
        if let Err(e) = Self::run_passes(&self.ctx, &mut module) {
            return Err(self
                .diagnostics
                .report_and_fail(file_path, CompilationError::Codegen(e)));
        }

        tracing::debug!("Passes ran successfully");

        if compiler_options.dump_mlir {
            let mlir_path = PathBuf::from("mlir_dumps")
                .join(format!("{}-dump-postpass", relative_path.display()))
                .with_extension("mlir");

            fs::create_dir_all(mlir_path.parent().unwrap())?;

            let mut f = fs::File::create(mlir_path).unwrap();
            write!(f, "{}", module.as_operation()).unwrap();
        }

        Ok(module)
    }

    pub fn compile_path<'func>(
        &'func self,
        file_path: &Path,
        compiler_options: CompilerOpts,
    ) -> MathicResult<Module<'func>> {
        let source = fs::read_to_string(file_path)?;

        self.compile_source(&source, Some(file_path.to_path_buf()), compiler_options)
    }

    pub fn compile_source<'func>(
        &'func self,
        source: &str,
        file_path: Option<PathBuf>,
        compiler_options: CompilerOpts,
    ) -> MathicResult<Module<'func>> {
        self.diagnostics.clear()?;

        let path = file_path
            .clone()
            .unwrap_or_else(|| PathBuf::from("program"));

        // Source code parsing.
        let ast = {
            let parser = MathicParser::new(source);
            match parser.parse("program".to_string()) {
                Ok(ast) => ast,
                Err(e) => {
                    self.diagnostics.report(path, CompilationError::Parse(e))?;
                    return Err(MathicError::CompilationFailed);
                }
            }
        };

        // AST lowering and semantic checks.
        let ir = match lowering::lower_program(&ast) {
            Ok(ir) => ir,
            Err(e) => {
                self.diagnostics
                    .report(path, CompilationError::Lowering(e))?;
                return Err(MathicError::CompilationFailed);
            }
        };

        if compiler_options.dump_mathir {
            let mathir_path = PathBuf::from("program.mathir");

            let mut f_mathir = fs::File::create(mathir_path)?;

            write!(f_mathir, "{}", ir)?;
        }

        // Generate Module.
        let mut module = match ffi::create_module(&self.ctx, compiler_options.opt_lvl) {
            Ok(module) => module,
            Err(e) => {
                self.diagnostics
                    .report(path.clone(), CompilationError::Codegen(e))?;
                return Err(MathicError::CompilationFailed);
            }
        };

        {
            let codegen = MathicCodeGen::new(&self.ctx, &ir, &module, file_path);
            let mut helper = CompilerHelper::new();

            if let Err(e) = codegen.generate_module(&mut helper) {
                self.diagnostics
                    .report(path, CompilationError::Codegen(e))?;
                return Err(MathicError::CompilationFailed);
            }
        }

        if compiler_options.dump_mlir {
            let file_path = PathBuf::from("dump-prepass.mlir");

            let mut f_prepass_program = fs::File::create(file_path)?;

            write!(f_prepass_program, "{}", module.as_operation())?;
        }

        debug_assert!(module.as_operation().verify());
        tracing::debug!("Module crated successfully");

        // Run Passes to the generated module.
        if let Err(e) = Self::run_passes(&self.ctx, &mut module) {
            self.diagnostics
                .report(path, CompilationError::Codegen(e))?;
            return Err(MathicError::CompilationFailed);
        }

        tracing::debug!("Passes ran successfully");

        if compiler_options.dump_mlir {
            let file_path = PathBuf::from("dump.mlir");
            let mut f = fs::File::create(file_path).unwrap();
            write!(f, "{}", module.as_operation()).unwrap();
        }

        Ok(module)
    }

    fn parse_file(
        &self,
        src_root: &Path,
        path: PathBuf,
        parsed_files: &mut HashSet<String>,
    ) -> MathicResult<HashMap<PathBuf, Arc<MathicModule>>> {
        let abs_path = src_root.join(&path);
        let base_path = abs_path.parent().unwrap();

        let source = fs::read_to_string(&abs_path)?;

        if !parsed_files.insert(abs_path.to_string_lossy().into_owned()) {
            return Ok(HashMap::with_capacity(0));
        }

        let mut compilation_unit = HashMap::new();

        let mut program = {
            let parser = MathicParser::new(&source);
            let module_name = path
                .with_extension("")
                .to_string_lossy()
                .replace("/", "::")
                .to_string();

            match parser.parse(module_name) {
                Err(e) => {
                    self.diagnostics
                        .report(abs_path.clone(), CompilationError::Parse(e))?;
                    return Ok(compilation_unit);
                }
                Ok(module) => module,
            }
        };

        for item in &program.items {
            if let TopLevelItem::Import(MathicPath { idents, span }) = item {
                let full_path = base_path.join(idents.join("/")).with_added_extension("mth");

                // * if the full path of the import is a file, then we
                // parse that file as normal.
                //
                // * if the full is not a file, it could only be
                // that the import references a top level item, so we
                // try to parse the path formed by all them idents but
                // the last one (top leve item).
                let import_path = if full_path.is_file() {
                    full_path
                } else {
                    let Some(idents_without_last) = idents.get(..idents.len() - 1) else {
                        let path = full_path
                            .strip_prefix(src_root)
                            .unwrap()
                            .to_string_lossy()
                            .replace("/", "::");

                        self.diagnostics.report(
                            abs_path.clone(),
                            CompilationError::Lowering(LoweringError::UnResolvedPath {
                                path,
                                span: *span,
                            }),
                        )?;
                        continue;
                    };

                    base_path
                        .join(idents_without_last.join("/"))
                        .with_added_extension("mth")
                };

                if !import_path.exists() {
                    let path = import_path
                        .strip_prefix(src_root)
                        .unwrap()
                        .to_string_lossy()
                        .replace("/", "::");

                    self.diagnostics.report(
                        abs_path.clone(),
                        CompilationError::Lowering(LoweringError::UnResolvedPath {
                            path,
                            span: *span,
                        }),
                    )?;
                    continue;
                }

                compilation_unit.extend(self.parse_file(
                    src_root,
                    import_path.strip_prefix(src_root).unwrap().to_owned(),
                    parsed_files,
                )?);
            }
        }

        program.modules = compilation_unit.values().cloned().collect();

        compilation_unit.insert(src_root.join(&path), Arc::new(program));

        Ok(compilation_unit)
    }

    fn run_passes(ctx: &Context, module: &mut Module) -> Result<(), CodegenError> {
        let pass_manager = PassManager::new(ctx);

        pass_manager.enable_verifier(true);
        pass_manager.add_pass(create_canonicalizer());
        pass_manager.add_pass(create_scf_to_control_flow()); // needed because to_llvm doesn't include it.
        pass_manager.add_pass(create_symbolic_extract_eval());
        pass_manager.add_pass(create_symbolic_to_arith());
        pass_manager.add_pass(create_to_llvm());

        pass_manager.run(module)?;

        Ok(())
    }
}
