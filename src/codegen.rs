use crate::parser::grammar::Program;
use melior::{
    Context, DialectRegistry, Module, PassManager,
    dialect::DialectRegistry,
    ir::{Location, Module},
    pass::PassManager,
};

pub struct MathicCodeGen {
    context: Context,
    module: Module,
    pass_manager: PassManager,
}

impl MathicCodeGen {
    pub fn new() -> Self {
        let context = Context::new();
        let module = Module::new(Location::unknown(context));
        let pass_manager = Self::create_pass_manager();

        context.append_dialect_registry(&Self::create_dialect_registry());
        context.load_all_available_dialects();

        register_all_passes();
        register_all_llvm_translations(&context);

        Self {
            context,
            module,
            pass_manager,
        }
    }

    fn create_dialect_registry() -> DialectRegistry {
        let registry = DialectRegistry::new();

        register_all_dialects(&registry);

        registry
    }

    fn create_pass_manager() -> PassManager {
        let pass_manager = PassManager::new(&module);

        pass_manager.enable_verifier(true);
        pass_manager.add_pass(pass::transform::create_canonicalizer());
        pass_manager.add_pass(pass::conversion::create_scf_to_control_flow()); // needed because to_llvm doesn't include it.
        pass_manager.add_pass(pass::conversion::create_to_llvm());

        pass_manager
    }

    pub fn generate_module(&self, ast: &Program) -> Module {
        todo!("Generate MLIR module from AST")
    }
}
