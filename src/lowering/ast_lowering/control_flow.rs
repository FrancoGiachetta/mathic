use crate::{
    lowering::{
        Lowerer,
        ir::{basic_block::Terminator, function::Function},
    },
    parser::ast::control_flow::{ForStmt, IfStmt, WhileStmt},
};

impl Lowerer {
    pub fn lower_if(&self, func: &mut Function, stmt: &IfStmt) {
        let IfStmt {
            condition,
            then_block,
            else_block,
        } = stmt;

        let condition = self.lower_expr(func, condition);

        let trigger_block_idx = func.last_block_idx();

        let (true_block, false_block) = if let Some(else_block) = else_block {
            // if there's an else_block we will need at least three more blocks:
            //     - The true block.
            //     - The false block.
            //     - The next block, which will be the target block of the
            //       branching operation of other two after the statements.
            let target_block_idx = func.last_block_idx() + 3;

            let then_block_idx = func.add_block(
                Terminator::Branch {
                    target: target_block_idx,
                    span: None,
                },
                Some(then_block.span.clone()),
            );

            for stmt in then_block.stmts.iter() {
                self.lower_stmt(stmt, func);
            }

            let else_block_idx = func.add_block(
                Terminator::Branch {
                    target: target_block_idx,
                    span: None,
                },
                Some(else_block.span.clone()),
            );

            for stmt in else_block.stmts.iter() {
                self.lower_stmt(stmt, func);
            }

            (then_block_idx, else_block_idx)
        } else {
            // if there's no else block we will need at least two more blocks:
            //     - The true block.
            //     - The next block, which will also be the target block of the
            //       branching operation of two.
            let target_block_idx = func.last_block_idx() + 2;

            let then_block_idx = func.add_block(
                Terminator::Branch {
                    target: target_block_idx,
                    span: None,
                },
                Some(then_block.span.clone()),
            );

            for stmt in then_block.stmts.iter() {
                self.lower_stmt(stmt, func);
            }

            // FUTURE: when adding the else if blocks, it will be similar as
            // having an else block but adding the amount of else if blocks
            // and decrement them with a for loop.

            (then_block_idx, target_block_idx)
        };

        func.get_basic_block_mut(trigger_block_idx).terminator = Terminator::CondBranch {
            condition,
            true_block,
            false_block,
            span: None,
        };
    }

    #[allow(dead_code)]
    pub fn lower_while(&self, _func: &mut Function, _stmt: &WhileStmt) {
        todo!()
    }

    #[allow(dead_code)]
    pub fn lower_for(&self, _func: &mut Function, _stmt: &ForStmt) {
        todo!()
    }
}
