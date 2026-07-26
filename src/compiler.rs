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
    codegen::{MathicCodeGen, compiler_helper::CompilerHelper},
    diagnostics::{self, CodegenError},
    ffi::{
        self,
        dialect_integration::symbolic_dialect::{
            create_symbolic_extract_eval, create_symbolic_to_arith,
        },
    },
    lowering,
    parser::MathicParser,
};

#[derive(Debug, Clone, Copy)]
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

    pub fn compile_path<'func>(
        &'func self,
        file_path: &Path,
        compiler_options: CompilerOpts,
    ) -> MathicResult<Module> {
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

    pub fn compile_source<'func>(
        &'func self,
        source: &str,
        file_path: Option<PathBuf>,
        compiler_options: CompilerOpts,
    ) -> MathicResult<Module> {
        // Source code parsing.
        let ast = {
            let parser = MathicParser::new(source);
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
