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
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use std::{fs, path::Path};

use crate::{
    MathicError, MathicResult,
    codegen::{MathicCodeGen, compiler_helper::CompilerHelper, module::MathicModule},
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
            IrModule,
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

    /// Compiles a project.
    ///
    /// It has ondemand compilation, meaning the of a module is never
    /// referenced it will not be compiled.
    pub fn compile_project(
        &self,
        src_root: &Path,
        compiler_options: CompilerOpts,
    ) -> MathicResult<Vec<MathicModule<'_>>> {
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

        let modules = lowered
            .par_iter()
            .map(|(path, ir)| {
                (
                    path,
                    self.compile_module(ir, src_root, path, compiler_options),
                )
            })
            .collect::<Vec<_>>();

        let mut compiled_modules = Vec::with_capacity(modules.len());

        for (_, module) in modules {
            if let Ok(module) = module {
                compiled_modules.push(module);
            }
        }

        if self.diagnostics.has_errors()? {
            return Err(MathicError::CompilationFailed);
        }

        Ok(compiled_modules)
    }

    /// Handles the code generation of a Mathir.
    fn compile_module<'func>(
        &'func self,
        ir: &Ir,
        src_root: &Path,
        file_path: &Path,
        compiler_options: CompilerOpts,
    ) -> MathicResult<MathicModule<'func>> {
        let relative_path = file_path
            .strip_prefix(src_root)
            .unwrap_or(file_path)
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
        let mut module = match MathicModule::new(&self.ctx, compiler_options.opt_lvl) {
            Ok(module) => module,
            Err(e) => {
                return Err(self
                    .diagnostics
                    .report_and_fail(file_path.to_path_buf(), CompilationError::Codegen(e)));
            }
        };

        {
            let codegen = MathicCodeGen::new(
                &self.ctx,
                ir,
                module.as_inner(),
                Some(file_path.to_path_buf()),
            );
            let mut helper = CompilerHelper::new();

            if let Err(e) = codegen.generate_module(&mut helper) {
                return Err(self
                    .diagnostics
                    .report_and_fail(file_path.to_path_buf(), CompilationError::Codegen(e)));
            }
        }

        if compiler_options.dump_mlir {
            let mlir_path = PathBuf::from("mlir_dumps")
                .join(format!("{}-dump-prepass", relative_path.display()))
                .with_extension("mlir");

            fs::create_dir_all(mlir_path.parent().unwrap())?;

            let mut f_prepass_program = fs::File::create(mlir_path)?;

            write!(f_prepass_program, "{}", module.inner_to_operation())?;
        }

        debug_assert!(module.inner_to_operation().verify());
        tracing::debug!("Module crated successfully");

        // Run Passes to the generated module.
        if let Err(e) = Self::run_passes(&self.ctx, module.as_inner_mut()) {
            return Err(self
                .diagnostics
                .report_and_fail(file_path.to_path_buf(), CompilationError::Codegen(e)));
        }

        tracing::debug!("Passes ran successfully");

        if compiler_options.dump_mlir {
            let mlir_path = PathBuf::from("mlir_dumps")
                .join(format!("{}-dump-postpass", relative_path.display()))
                .with_extension("mlir");

            fs::create_dir_all(mlir_path.parent().unwrap())?;

            let mut f = fs::File::create(mlir_path).unwrap();
            write!(f, "{}", module.inner_to_operation())?;
        }

        Ok(module)
    }

    /// Compiles a single path.
    pub fn compile_path<'func>(
        &'func self,
        file_path: &Path,
        compiler_options: CompilerOpts,
    ) -> MathicResult<MathicModule<'func>> {
        let source = fs::read_to_string(file_path)?;

        self.compile_source(&source, Some(file_path.to_path_buf()), compiler_options)
    }

    /// Compiles a source program.
    pub fn compile_source<'func>(
        &'func self,
        source: &str,
        file_path: Option<PathBuf>,
        compiler_options: CompilerOpts,
    ) -> MathicResult<MathicModule<'func>> {
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
        let mut module = match MathicModule::new(&self.ctx, compiler_options.opt_lvl) {
            Ok(module) => module,
            Err(e) => {
                self.diagnostics
                    .report(path.clone(), CompilationError::Codegen(e))?;
                return Err(MathicError::CompilationFailed);
            }
        };

        {
            let codegen = MathicCodeGen::new(&self.ctx, &ir, module.as_inner(), file_path);
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

            write!(f_prepass_program, "{}", module.inner_to_operation())?;
        }

        debug_assert!(module.inner_to_operation().verify());
        tracing::debug!("Module crated successfully");

        // Run Passes to the generated module.
        if let Err(e) = Self::run_passes(&self.ctx, module.as_inner_mut()) {
            self.diagnostics
                .report(path, CompilationError::Codegen(e))?;
            return Err(MathicError::CompilationFailed);
        }

        tracing::debug!("Passes ran successfully");

        if compiler_options.dump_mlir {
            let file_path = PathBuf::from("dump.mlir");
            let mut f = fs::File::create(file_path).unwrap();
            write!(f, "{}", module.inner_to_operation()).unwrap();
        }

        Ok(module)
    }

    /// Parses a file.
    ///
    /// There may be recursive calls to this function for cases with the module
    /// imports another.
    ///
    /// Returns HashMap with the mapping the paths parsed to their AST.
    fn parse_file(
        &self,
        src_root: &Path,
        path: PathBuf,
        parsed_files: &mut HashSet<String>,
    ) -> MathicResult<HashMap<PathBuf, Arc<IrModule>>> {
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
            if let TopLevelItem::Import(MathicPath {
                idents,
                group_paths: _,
                span,
                import_all: _,
            }) = item
            {
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
