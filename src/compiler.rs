use std::{collections::HashSet, env, io::Write, path::PathBuf};

use melior::{
    Context,
    ir::{Module, operation::OperationLike},
    pass::{
        PassManager,
        conversion::{create_scf_to_control_flow, create_to_llvm},
        transform::create_canonicalizer,
    },
};

use std::{fs, path::Path};

use crate::{
    MathicResult,
    codegen::{MathicCodeGen, compiler_helper::CompilerHelper},
    diagnostics::{self, CodegenError, LoweringError, MathicError},
    ffi::{
        self,
        dialect_integration::symbolic_dialect::{
            create_symbolic_extract_eval, create_symbolic_to_arith,
        },
    },
    lowering,
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
}

unsafe impl Send for MathicCompiler {}
unsafe impl Sync for MathicCompiler {}

impl MathicCompiler {
    pub fn new() -> Result<Self, CodegenError> {
        Ok(Self {
            ctx: ffi::create_context()?,
        })
    }

    pub fn compile_project(&self, _compiler_options: CompilerOpts) -> MathicResult<()> {
        let main_file_path = env::current_dir()?.join("src/main.mth");

        let compilation_unit = {
            let mut parsed_files = HashSet::new();
            self.parse_file(main_file_path, &mut parsed_files)?
        };

        dbg!(compilation_unit);

        Ok(())
    }

    pub fn compile_path<'func>(
        &'func self,
        file_path: &Path,
        compiler_options: CompilerOpts,
    ) -> MathicResult<Module<'func>> {
        // Read source file
        let source = fs::read_to_string(file_path)?;

        match self.compile_source(&source, Some(file_path.to_path_buf()), compiler_options) {
            Err(e) => {
                diagnostics::format_error(file_path, &e);
                std::process::exit(1);
            }
            module => module,
        }
    }

    pub fn compile_source(
        &self,
        source: &str,
        file_path: Option<PathBuf>,
        compiler_options: CompilerOpts,
    ) -> MathicResult<Module<'_>> {
        // Source code parsing.
        let ast = {
            let parser = MathicParser::new(source, None);
            parser.parse()?
        };

        // AST lowering and semantic checks.
        let ir = lowering::lower_program(&ast)?;

        if compiler_options.dump_mathir {
            let mathir_path = PathBuf::from("program.mathir");

            let mut f_mathir = fs::File::create(mathir_path)?;

            write!(f_mathir, "{}", ir)?;
        }

        // Generate Module.
        let mut module = ffi::create_module(&self.ctx, compiler_options.opt_lvl)?;

        {
            let codegen = MathicCodeGen::new(&self.ctx, &ir, &module, file_path);
            let mut helper = CompilerHelper::new();

            codegen.generate_module(&mut helper)?;
        }

        if compiler_options.dump_mlir {
            let file_path = PathBuf::from("dump-prepass.mlir");

            let mut f_prepass_program = fs::File::create(file_path)?;

            write!(f_prepass_program, "{}", module.as_operation())?;
        }

        debug_assert!(module.as_operation().verify());
        tracing::debug!("Module crated successfully");

        // Run Passes to the generated module.
        Self::run_passes(&self.ctx, &mut module)?;

        tracing::debug!("Passes ran successfully");

        if compiler_options.dump_mlir {
            let file_path = PathBuf::from("dump.mlir");
            let mut f = fs::File::create(file_path).unwrap();
            write!(f, "{}", module.as_operation()).unwrap();
        }

        Ok(module)
    }

    pub fn parse_file(
        &self,
        path: PathBuf,
        parsed_files: &mut HashSet<String>,
    ) -> MathicResult<Vec<MathicModule>> {
        let base_path = path.parent().unwrap().to_owned();

        dbg!("BASE PATH: {}", &base_path);

        let source = fs::read_to_string(&path)?;

        if !parsed_files.insert(path.to_string_lossy().into_owned()) {
            return Ok(Vec::with_capacity(0));
        }

        let mut compilation_unit = Vec::new();

        let program = {
            let parser = MathicParser::new(&source, Some(path.clone()));
            match parser.parse() {
                Err(e) => {
                    diagnostics::format_error(&path, &e.into());
                    std::process::exit(1);
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
                let path = if full_path.is_file() {
                    full_path
                } else {
                    let idents_without_last = idents.get(..idents.len() - 1).ok_or(
                        MathicError::Lowering(LoweringError::UnResolvedPath {
                            path: full_path,
                            span: *span,
                        }),
                    )?;
                    base_path
                        .join(idents_without_last.join("/"))
                        .with_added_extension("mth")
                };

                if !path.exists() {
                    return Err(MathicError::Lowering(LoweringError::UnResolvedPath {
                        path,
                        span: *span,
                    }));
                }

                compilation_unit.extend(self.parse_file(path, parsed_files)?);
            }
        }

        compilation_unit.insert(0, program);

        Ok(compilation_unit)
    }

    fn run_passes(ctx: &Context, module: &mut Module) -> MathicResult<()> {
        let pass_manager = PassManager::new(ctx);

        pass_manager.enable_verifier(true);
        pass_manager.add_pass(create_canonicalizer());
        pass_manager.add_pass(create_scf_to_control_flow()); // needed because to_llvm doesn't include it.
        pass_manager.add_pass(create_symbolic_extract_eval());
        pass_manager.add_pass(create_symbolic_to_arith());
        pass_manager.add_pass(create_to_llvm());

        pass_manager.run(module).map_err(CodegenError::from)?;

        Ok(())
    }
}
