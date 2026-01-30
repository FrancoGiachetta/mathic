use melior::{
    Context, DialectRegistry, Module, PassManager,
    dialect::DialectRegistry,
    ir::{Location, Module},
    pass::PassManager,
};

use crate::{
    codegen::error::CodegenError,
    error::{MathicError, Result},
    parser::grammar::Program,
};

pub mod error;
pub mod expression;
pub mod statement;

pub struct MathicCodeGen {
    context: Context,
    module: Module,
    pass_manager: PassManager,
}

impl MathicCodeGen {
    pub fn new() -> Result<Self> {
        let context = Context::new();
        let module = Module::new(Location::unknown(&context));
        let pass_manager = Self::create_pass_manager(&module)?;

        context.append_dialect_registry(&Self::create_dialect_registry());
        context.load_all_available_dialects();

        register_all_passes();
        register_all_llvm_translations(&context);

        Ok(Self {
            context,
            module,
            pass_manager,
        })
    }

    fn create_dialect_registry() -> DialectRegistry {
        let registry = DialectRegistry::new();

        register_all_dialects(&registry);

        registry
    }

    fn create_pass_manager(module: &Module) -> Result<PassManager> {
        let pass_manager = PassManager::new(module);

        pass_manager.enable_verifier(true);
        pass_manager.add_pass(pass::transform::create_canonicalizer());
        pass_manager.add_pass(pass::conversion::create_scf_to_control_flow()); // needed because to_llvm doesn't include it.
        pass_manager.add_pass(pass::conversion::create_to_llvm());

        Ok(pass_manager)
    }

    pub fn generate_module(&mut self, program: &Program) -> Result<Module> {
        // Check if main function is present
        if !program.funcs.iter().any(|f| f.name == "main") {
            return Err(MathicError::Codegen(CodegenError::MissingMainFunction));
        }

        // TODO: Compile structs in the future

        for func in &program.funcs {
            self.compile_function(func)?;
        }

        Ok(self.module.clone())
    }
}
