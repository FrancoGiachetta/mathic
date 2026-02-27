use crate::{
    diagnostics::LoweringError,
    lowering::{
        ast_lowering::{expression, statement},
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

pub fn lower_if(func: &mut Function, stmt: &IfStmt) -> Result<(), LoweringError> {
    let IfStmt {
        condition,
        then_block,
        else_block,
    } = stmt;

    let condition = expression::lower_expr(func, condition)?;

    let trigger_block_idx = func.last_block_idx();

    let (true_block, false_block) = if let Some(else_block) = else_block {
        let target_block_idx = func.last_block_idx() + 3;

        let then_block_idx = statement::lower_block(
            func,
            then_block,
            Terminator::Branch {
                target: target_block_idx,
                span: None,
            },
        )?;

        let else_block_idx = statement::lower_block(
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

        let then_block_idx = statement::lower_block(
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
    func: &mut Function,
    stmt: &WhileStmt,
    _span: Span,
) -> Result<(), LoweringError> {
    let WhileStmt { condition, body } = stmt;

    let loop_breaker_condition = expression::lower_expr(func, condition)?;

    lower_loop(func, body, loop_breaker_condition, Vec::with_capacity(0))
}

pub fn lower_for(func: &mut Function, stmt: &ForStmt, span: Span) -> Result<(), LoweringError> {
    let ForStmt {
        variable,
        start,
        end,
        body,
    } = stmt;

    let loop_tracker_idx =
        func.add_local(Some(variable.clone()), Some(span.clone()), LocalKind::Temp)?;
    let loop_breaker_condition = RValInstruct::Binary {
        op: BinaryOp::Compare(CmpOp::Lt),
        lhs: Box::new(RValInstruct::Use(Value::InMemory(loop_tracker_idx), None)),
        rhs: Box::new(expression::lower_expr(func, end)?),
        span: start.span.start..end.span.end,
    };

    let before_instructions = vec![LValInstruct::Assign {
        local_idx: loop_tracker_idx,
        value: RValInstruct::Binary {
            op: BinaryOp::Arithmetic(ArithOp::Add),
            lhs: Box::new(RValInstruct::Use(Value::InMemory(loop_tracker_idx), None)),
            rhs: Box::new(RValInstruct::Use(1i64.into(), None)),
            span: start.span.start..end.span.end,
        },
        span: None,
    }];

    let init = expression::lower_expr(func, start)?;
    func.push_instruction(LValInstruct::Let {
        local_idx: loop_tracker_idx,
        init,
        span: None,
    });

    lower_loop(func, body, loop_breaker_condition, before_instructions)
}

fn lower_loop(
    func: &mut Function,
    loop_body: &BlockStmt,
    condition: RValInstruct,
    before_instructions: Vec<LValInstruct>,
) -> Result<(), LoweringError> {
    let start_block_idx = func.last_block_idx() + 1;
    let loop_block_idx = start_block_idx + 1;
    let end_block_idx = start_block_idx + 2;

    func.get_basic_block_mut(func.last_block_idx()).terminator = Terminator::Branch {
        target: start_block_idx,
        span: None,
    };

    let start_block_idx = func.add_block(
        Terminator::CondBranch {
            condition,
            true_block: loop_block_idx,
            false_block: end_block_idx,
            span: None,
        },
        None,
    );

    for i in before_instructions {
        func.push_instruction(i);
    }

    let _ = statement::lower_block(
        func,
        loop_body,
        Terminator::Branch {
            target: start_block_idx,
            span: None,
        },
    );

    func.add_block(Terminator::Return(None, None), None);

    Ok(())
}
