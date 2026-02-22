use crate::{
    lowering::{
        Lowerer,
        error::LoweringError,
        ir::{basic_block::Terminator, function::Function},
    },
    parser::ast::{
        Span,
        control_flow::{ForStmt, IfStmt, WhileStmt},
    },
};

impl Lowerer {
    pub fn lower_if(&self, func: &mut Function, stmt: &IfStmt) -> Result<(), LoweringError> {
        let IfStmt {
            condition,
            then_block,
            else_block,
        } = stmt;

        let condition = self.lower_expr(func, condition)?;

        // FUTURE: check if the condition is of type boolean.

        // Hold the index of the current block to create the condition branch later.
        let trigger_block_idx = func.last_block_idx();

        let (true_block, false_block) = if let Some(else_block) = else_block {
            let target_block_idx = func.last_block_idx() + 3;

            let then_block_idx = self.lower_block(
                func,
                then_block,
                Terminator::Branch {
                    target: target_block_idx,
                    span: None,
                },
            )?;

            let else_block_idx = self.lower_block(
                func,
                else_block,
                Terminator::Branch {
                    target: target_block_idx,
                    span: None,
                },
            )?;

            (then_block_idx, else_block_idx)
        } else {
            let target_block_idx = func.last_block_idx() + 2;

            let then_block_idx = self.lower_block(
                func,
                then_block,
                Terminator::Branch {
                    target: target_block_idx,
                    span: None,
                },
            )?;

            (then_block_idx, target_block_idx)
        };

        func.get_basic_block_mut(trigger_block_idx).terminator = Terminator::CondBranch {
            condition,
            true_block,
            false_block,
            span: None,
        };

        Ok(())
    }

    pub fn lower_while(
        &self,
        _func: &mut Function,
        _stmt: &WhileStmt,
        span: Span,
    ) -> Result<(), LoweringError> {
        Err(LoweringError::UnsupportedFeature {
            feature: "while loops".to_string(),
            span,
        })
    }

    pub fn lower_for(
        &self,
        _func: &mut Function,
        _stmt: &ForStmt,
        span: Span,
    ) -> Result<(), LoweringError> {
        Err(LoweringError::UnsupportedFeature {
            feature: "for loops".to_string(),
            span,
        })
    }
}
