use melior::{
    Context,
    dialect::func,
    ir::{
        Block, BlockLike, Identifier, Location, Module, Region, RegionLike,
        attribute::{Attribute, StringAttribute, TypeAttribute},
        r#type::{FunctionType, IntegerType},
    },
};

use crate::{
    MathicResult,
    codegen::error::CodegenError,
    error::MathicError,
    parser::ast::{Program, declaration::FuncDecl},
};

pub mod control_flow;
pub mod declaration;
pub mod error;
pub mod expression;
pub mod statement;
pub mod symbol_table;

pub use symbol_table::SymbolTable;

pub struct MathicCodeGen<'this, 'ctx> {
    module: &'this Module<'ctx>,
    // Tracks variables for the current function being compiled.
    // Reset when compiling a new global function.
    symbols: std::cell::RefCell<SymbolTable<'ctx, 'this>>,
}

impl<'this, 'ctx> MathicCodeGen<'this, 'ctx> {
    pub fn new(module: &'this Module<'ctx>) -> Self {
        Self {
            module,
            symbols: Default::default(),
        }
    }

    pub fn generate_module(&mut self, ctx: &'ctx Context, program: Program) -> MathicResult<()> {
        // Check if main function is present
        if !program.funcs.iter().any(|f| f.name == "main") {
            return Err(MathicError::Codegen(CodegenError::MissingMainFunction));
        }

        // TODO: Compile structs in the future

        for func in program.funcs {
            self.compile_entry_point(ctx, func)?;

            self.symbols.replace(SymbolTable::new());
        }

        Ok(())
    }

    pub fn compile_entry_point(
        &self,
        ctx: &'ctx Context,
        func: FuncDecl,
    ) -> Result<(), CodegenError> {
        // let params = vec![];

        let region = Region::new();
        let block = region.append_block(Block::new(&[]));

        for stmt in func.body.iter() {
            self.compile_statement(ctx, &block, stmt)?;
        }

        let location = Location::unknown(ctx);
        let i64_type = IntegerType::new(ctx, 64).into();

        self.module.body().append_operation(func::func(
            ctx,
            StringAttribute::new(ctx, &format!("mathic__{}", func.name)),
            TypeAttribute::new(FunctionType::new(ctx, &[], &[i64_type]).into()),
            region,
            // This is necessary for the ExecutorEngine to execute a function.
            &[(
                Identifier::new(ctx, "llvm.emit_c_interface"),
                Attribute::unit(ctx),
            )],
            location,
        ));

        Ok(())
    }
}
