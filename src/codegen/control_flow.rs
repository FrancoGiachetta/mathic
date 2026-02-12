use melior::{
    Context,
    dialect::scf,
    helpers::ArithBlockExt,
    ir::{Block, BlockLike, Location, Region, RegionLike, ValueLike},
};

use crate::{
    codegen::{MathicCodeGen, error::CodegenError},
    parser::grammar::control_flow::{ForStmt, IfStmt, WhileStmt},
};

impl<'this, 'ctx> MathicCodeGen<'this, 'ctx>
where
    'this: 'ctx,
{
    pub fn compile_if(
        &self,
        ctx: &'ctx Context,
        block: &'this Block<'ctx>,
        stmt: &IfStmt,
    ) -> Result<(), CodegenError> {
        let location = Location::unknown(ctx);
        let IfStmt {
            condition,
            then_block,
            else_block,
        } = stmt;

        let condition_val = self.compile_expression(ctx, block, condition)?;

        block.append_operation(scf::r#if(
            condition_val,
            &[],
            {
                let region = Region::new();
                let true_block = region.append_block(Block::new(&[]));

                self.compile_block(ctx, &true_block, &then_block.stmts)?;
                true_block.append_operation(scf::r#yield(&[], location));

                region
            },
            {
                let region = Region::new();

                let false_block = region.append_block(Block::new(&[]));

                if let Some(else_block) = else_block {
                    self.compile_block(ctx, &false_block, &else_block.stmts)?;
                }

                false_block.append_operation(scf::r#yield(&[], location));

                region
            },
            location,
        ));

        Ok(())
    }

    pub fn compile_while(
        &self,
        ctx: &'ctx Context,
        block: &'this Block<'ctx>,
        stmt: &WhileStmt,
    ) -> Result<(), CodegenError> {
        let location = Location::unknown(ctx);
        let WhileStmt { condition, body } = stmt;

        block.append_operation(scf::r#while(
            &[],
            &[],
            {
                let region = Region::new();
                let before_block = region.append_block(Block::new(&[]));
                let condition_val = self.compile_expression(ctx, &before_block, condition)?;

                before_block.append_operation(scf::condition(condition_val, &[], location));

                region
            },
            {
                let region = Region::new();
                let after_block = region.append_block(Block::new(&[]));

                self.compile_block(ctx, &after_block, &body.stmts)?;
                after_block.append_operation(scf::r#yield(&[], location));

                region
            },
            location,
        ));

        Ok(())
    }

    pub fn compile_for(
        &self,
        ctx: &'ctx Context,
        block: &'this Block<'ctx>,
        stmt: &ForStmt,
    ) -> Result<(), CodegenError> {
        let location = Location::unknown(ctx);
        let ForStmt { start, end, body } = stmt;

        let start_val = self.compile_expression(ctx, block, start)?;
        let end_val = self.compile_expression(ctx, block, end)?;

        block.append_operation(scf::r#for(
            start_val,
            end_val,
            block.const_int_from_type(ctx, location, 1, start_val.r#type())?,
            {
                let region = Region::new();
                let for_block = region.append_block(Block::new(&[]));

                self.compile_block(ctx, &for_block, &body.stmts)?;

                for_block.append_operation(scf::r#yield(&[], location));

                region
            },
            location,
        ));

        Ok(())
    }
}
