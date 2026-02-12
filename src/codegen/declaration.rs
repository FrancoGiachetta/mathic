use melior::{
    Context,
    dialect::func,
    ir::{
        Attribute, Block, BlockLike, Identifier, Location, Region, RegionLike,
        attribute::{StringAttribute, TypeAttribute},
        r#type::{FunctionType, IntegerType},
    },
};

use crate::{
    codegen::{MathicCodeGen, error::CodegenError},
    parser::ast::declaration::FuncDecl,
};

impl<'this, 'ctx> MathicCodeGen<'this, 'ctx>
where
    'this: 'ctx,
{
    pub fn compile_function(&self, ctx: &'ctx Context, func: FuncDecl) -> Result<(), CodegenError> {
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
