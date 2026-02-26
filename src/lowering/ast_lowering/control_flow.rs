use crate::{
    lowering::{
        Lowerer,
        error::LoweringError,
        ir::{
            basic_block::Terminator,
            function::{Function, LocalKind},
            instruction::{LValInstruct, RValInstruct},
            value::Value,
        },
    },
    parser::ast::{
        Span,
        control_flow::{ForStmt, IfStmt, WhileStmt},
        expression::{ArithOp, BinaryOp, CmpOp},
        statement::BlockStmt,
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

            func.add_block(Terminator::Return(None, None), None);

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
        func: &mut Function,
        stmt: &WhileStmt,
        _span: Span,
    ) -> Result<(), LoweringError> {
        let WhileStmt { condition, body } = stmt;

        let loop_breaker_condition = self.lower_expr(func, condition)?;

        self.lower_loop(func, body, loop_breaker_condition, Vec::with_capacity(0))
    }

    pub fn lower_for(
        &self,
        func: &mut Function,
        stmt: &ForStmt,
        _span: Span,
    ) -> Result<(), LoweringError> {
        let ForStmt { start, end, body } = stmt;

        // FUTURE: add proper support for variable tracking and custom steps.

        let loop_tracker_idx = func.add_local(None, None, LocalKind::Temp)?;
        let loop_breaker_condition = RValInstruct::Binary {
            op: BinaryOp::Compare(CmpOp::Ge),
            lhs: Box::new(RValInstruct::Use(Value::InMemory(loop_tracker_idx), None)),
            rhs: Box::new(self.lower_expr(func, end)?),
            span: start.span.start..end.span.end,
        };
        let before_instructions = vec![
            LValInstruct::Assign {
                local_idx: loop_tracker_idx,
                value: RValInstruct::Binary {
                    op: BinaryOp::Arithmetic(ArithOp::Sub),
                    lhs: Box::new(RValInstruct::Use(Value::InMemory(loop_tracker_idx), None)),
                    rhs: Box::new(RValInstruct::Use(1i64.into(), None)),
                    span: start.span.start..end.span.end,
                },
                span: None,
            };
            1
        ];

        self.lower_loop(func, body, loop_breaker_condition, before_instructions)
    }

    fn lower_loop(
        &self,
        func: &mut Function,
        loop_body: &BlockStmt,
        condition: RValInstruct,
        before_instructions: Vec<LValInstruct>,
    ) -> Result<(), LoweringError> {
        // Loops will take three basic blocks:
        //  1. start: it is in charge of checking if we should continue
        //     looping or not.
        //  2. loop: it is in charge of executing the loop's statements.
        //  3. exit: it next block after the loop finished.

        let start_block_idx = func.last_block_idx() + 1;
        let loop_block_idx = start_block_idx + 1;
        let end_block_idx = start_block_idx + 2;

        // Jump to the start block.
        func.get_basic_block_mut(func.last_block_idx()).terminator = Terminator::Branch {
            target: start_block_idx,
            span: None,
        };

        // The first block to add will be the start block. It will jump to the
        // loop block (the block to it) until the condition is not matched.
        // Once this happens, it will jump to exit block (which is two blocks
        // after it).
        let start_block_idx = func.add_block(
            Terminator::CondBranch {
                condition,
                true_block: loop_block_idx,
                false_block: end_block_idx,
                span: None,
            },
            None,
        );

        // Instructions to execute before verifying the condition.
        for i in before_instructions {
            func.push_instruction(i);
        }

        // The loop block executes the loop's statements and jumps back to the
        // start block.
        let _ = self.lower_block(
            func,
            loop_body,
            Terminator::Branch {
                target: start_block_idx,
                span: None,
            },
        );

        // Prepare the exit block.
        func.add_block(Terminator::Return(None, None), None);

        Ok(())
    }
}
