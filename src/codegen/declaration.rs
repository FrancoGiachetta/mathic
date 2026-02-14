use melior::{
    Context,
    dialect::func,
    ir::{
        Block, BlockLike, Location, Region, RegionLike,
        attribute::{StringAttribute, TypeAttribute},
        r#type::{FunctionType, IntegerType},
    },
};

use crate::{
    codegen::{MathicCodeGen, error::CodegenError},
    parser::ast::declaration::{DeclStmt, FuncDecl},
};

impl<'ctx> MathicCodeGen<'_, 'ctx> {
    pub fn compile_declaration<'func>(
        &self,
        ctx: &'ctx Context,
        block: &'func Block<'ctx>,
        stmt: &DeclStmt,
    ) -> Result<(), CodegenError> {
        match stmt {
            DeclStmt::Var(var_decl) => todo!(),
            DeclStmt::Struct(_struct_decl) => todo!("Implement struct"),
            DeclStmt::Func(func_decl) => self.compile_function(ctx, block, func_decl),
        }
    }

    pub fn compile_function<'func>(
        &self,
        ctx: &'ctx Context,
        block: &'func Block<'ctx>,
        func: &FuncDecl,
    ) -> Result<(), CodegenError> {
        // let params = vec![];

        let region = Region::new();
        let inner_block = region.append_block(Block::new(&[]));

        for stmt in func.body.iter() {
            self.compile_statement(ctx, &inner_block, stmt)?;
        }

        let location = Location::unknown(ctx);
        let i64_type = IntegerType::new(ctx, 64).into();

        block.append_operation(func::func(
            ctx,
            StringAttribute::new(ctx, &format!("mathic__{}", func.name)),
            TypeAttribute::new(FunctionType::new(ctx, &[], &[i64_type]).into()),
            region,
            &[],
            location,
        ));

        Ok(())
    }
}
