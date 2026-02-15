use std::{io::Write, path::PathBuf};

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
    codegen::{MathicCodeGen, error::CodegenError},
    ffi,
    parser::MathicParser,
};

#[derive(Default)]
#[repr(u8)]
pub enum OptLvl {
    None,
    #[default]
    O1,
    O2,
    O3,
}

pub struct MathicCompiler {
    ctx: Context,
}

impl MathicCompiler {
    pub fn new() -> Result<Self, CodegenError> {
        Ok(Self {
            ctx: ffi::create_context()?,
        })
    }

    pub fn compile_path<'func>(
        &'func self,
        file_path: &Path,
        opt_lvl: OptLvl,
    ) -> MathicResult<Module<'func>> {
        // Read source file
        let source = fs::read_to_string(file_path)?;

        self.compile_source(&source, opt_lvl, Some(file_path))
    }

    pub fn compile_source<'func>(
        &'func self,
        source: &str,
        opt_lvl: OptLvl,
        file_path: Option<&Path>,
    ) -> MathicResult<Module<'func>> {
        let parser = MathicParser::new(source);
        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(e) => {
                if let Some(path) = file_path {
                    parser.format_error(path, &e);
                }

                std::process::exit(1);
            }
        };

        // Generate Module.
        let mut module = ffi::create_module(&self.ctx, opt_lvl)?;

        {
            let codegen = MathicCodeGen {
                ctx: &self.ctx,
                module: &module,
            };

            codegen.generate_module(ast)?;
        }

        if let Ok(v) = std::env::var("MATHIC_DBG_DUMP") {
            if v == "1" {
                let file_path = PathBuf::from("dump-prepass.mlir");
                let mut f = fs::File::create(file_path).unwrap();
                write!(f, "{}", module.as_operation()).unwrap();
            } else {
                tracing::warn!(
                    "Incorrect value for MATHIC_DBG_DUMP: \"{}\", igonring it",
                    v
                )
            }
        }

        debug_assert!(module.as_operation().verify());
        tracing::debug!("Module crated successfully");

        // Run Passes to the generated module.
        Self::run_passes(&self.ctx, &mut module)?;

        tracing::debug!("Passes ran successfully");

        if let Ok(v) = std::env::var("MATHIC_DBG_DUMP") {
            if v == "1" {
                let file_path = PathBuf::from("dump.mlir");
                let mut f = fs::File::create(file_path).unwrap();
                write!(f, "{}", module.as_operation()).unwrap();
            } else {
                tracing::warn!(
                    "Incorrect value for MATHIC_DBG_DUMP: \"{}\", igonring it",
                    v
                )
            }
        }

        Ok(module)
    }

    fn run_passes(ctx: &Context, module: &mut Module) -> Result<(), CodegenError> {
        let pass_manager = PassManager::new(ctx);

        pass_manager.enable_verifier(true);
        pass_manager.add_pass(create_canonicalizer());
        pass_manager.add_pass(create_scf_to_control_flow()); // needed because to_llvm doesn't include it.
        pass_manager.add_pass(create_to_llvm());

        pass_manager.run(module)?;

        Ok(())
    }
}
