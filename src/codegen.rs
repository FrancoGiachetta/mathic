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

pub struct MathicCodeGen<'ctx> {
    pub ctx: &'ctx Context,
    pub module: &'ctx Module<'ctx>,
}

impl<'ctx> MathicCodeGen<'ctx> {
    pub fn generate_module(&self, program: Program) -> MathicResult<()> {
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
