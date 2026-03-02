use crate::{
    diagnostics::LoweringError,
    lowering::ir::{
        basic_block::Terminator,
        function::{Function, LocalKind},
        instruction::{LValInstruct, RValInstruct},
        types::{MathicType, SintTy},
        value::{ConstExpr, NumericConst, Value},
    },
    parser::ast::{
        Span,
        expression::{BinaryOp, ExprStmt, ExprStmtKind, LogicalOp, PrimaryExpr, UnaryOp},
    },
};

pub fn lower_expr(
    func: &mut Function,
    expr: &ExprStmt,
    ty_hint: Option<MathicType>,
) -> Result<(RValInstruct, MathicType), LoweringError> {
    let rvalue = match &expr.kind {
        ExprStmtKind::Primary(val) => lower_primary_value(func, val, expr.span.clone())?,
        ExprStmtKind::Binary { lhs, op, rhs } => {
            lower_binary_op(func, lhs, *op, rhs, expr.span.clone())?
        }
        ExprStmtKind::Unary { op, rhs } => lower_unary_op(func, *op, rhs, expr.span.clone())?,
        ExprStmtKind::Group(expr) => {
            return lower_expr(func, expr, ty_hint);
        }
        ExprStmtKind::Call { callee, args } => {
            lower_call(func, callee.clone(), args, expr.span.clone())?
        }
        ExprStmtKind::Assign {
            name,
            expr: assign_expr,
        } => lower_assignment(func, name, assign_expr, expr.span.clone())?,
        ExprStmtKind::Logical { lhs, op, rhs } => {
            lower_logical_op(func, lhs, *op, rhs, expr.span.clone())?
        }
        ExprStmtKind::Index { .. } => todo!(),
    };

    Ok((
        rvalue,
        lower_expression_type(func, &expr.kind, ty_hint, expr.span.clone())?,
    ))
}

fn lower_assignment(
    func: &mut Function,
    name: &str,
    expr: &ExprStmt,
    span: Span,
) -> Result<RValInstruct, LoweringError> {
    let local = func.get_local_from_name(name, span.clone())?;
    let (value, ty) = lower_expr(func, expr, Some(local.ty))?;

    // The new value should be of the same type as the local's.
    if local.ty != ty {
        return Err(LoweringError::MismatchedType {
            expected: local.ty,
            found: ty,
            span,
        });
    }

    func.get_basic_block_mut(func.last_block_idx())
        .instructions
        .push(LValInstruct::Assign {
            local_idx: local.local_idx,
            value,
            span: Some(span),
        });

    Ok(RValInstruct::Use(Value::Const(ConstExpr::Void), None))
}

fn lower_call(
    func: &mut Function,
    callee: String,
    func_args: &[ExprStmt],
    span: Span,
) -> Result<RValInstruct, LoweringError> {
    let mut arg_values: Vec<RValInstruct> = Vec::new();

    for (_, arg) in func_args.iter().enumerate() {
        let (arg_val, _) = lower_expr(func, arg, None)?;

        arg_values.push(arg_val);
    }

    // FUTURE: check that the amount of args matches the expected and that
    // every type matches the expected type.

    let local_idx = func.add_local(None, MathicType::Sint(SintTy::I64), None, LocalKind::Temp)?;

    let dest_block_idx = func.last_block_idx() + 1;

    func.get_basic_block_mut(func.last_block_idx()).terminator = Terminator::Call {
        callee,
        args: arg_values,
        span: Some(span),
        return_dest: Value::InMemory(local_idx),
        dest_block: dest_block_idx,
    };

    func.add_block(Terminator::Return(None, None), None);

    Ok(RValInstruct::Use(Value::InMemory(local_idx), None))
}

fn lower_binary_op(
    func: &mut Function,
    lhs: &ExprStmt,
    op: BinaryOp,
    rhs: &ExprStmt,
    span: Span,
) -> Result<RValInstruct, LoweringError> {
    let (lhs, lhs_ty) = lower_expr(func, lhs, None)?;
    let (rhs, rhs_ty) = lower_expr(func, rhs, None)?;

    // Operands' types must match.
    if lhs_ty != rhs_ty {
        return Err(LoweringError::MismatchedType {
            expected: lhs_ty,
            found: rhs_ty,
            span,
        });
    }

    Ok(RValInstruct::Binary {
        op,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
        span,
    })
}

fn lower_logical_op(
    func: &mut Function,
    lhs: &ExprStmt,
    op: LogicalOp,
    rhs: &ExprStmt,
    span: Span,
) -> Result<RValInstruct, LoweringError> {
    let (lhs, lhs_ty) = lower_expr(func, lhs, None)?;
    let (rhs, rhs_ty) = lower_expr(func, rhs, None)?;

    // Operands' types must be boolean.
    if lhs_ty.is_bool() {
        return Err(LoweringError::MismatchedType {
            expected: lhs_ty,
            found: MathicType::Bool,
            span,
        });
    }
    if rhs_ty.is_bool() {
        return Err(LoweringError::MismatchedType {
            expected: rhs_ty,
            found: MathicType::Bool,
            span,
        });
    }

    Ok(RValInstruct::Logical {
        op,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
        span,
    })
}

fn lower_unary_op(
    func: &mut Function,
    op: UnaryOp,
    rhs: &ExprStmt,
    span: Span,
) -> Result<RValInstruct, LoweringError> {
    let (rhs, _) = lower_expr(func, rhs, None)?;

    Ok(RValInstruct::Unary {
        op,
        rhs: Box::new(rhs),
        span,
    })
}

fn lower_primary_value(
    func: &mut Function,
    expr: &PrimaryExpr,
    span: Span,
) -> Result<RValInstruct, LoweringError> {
    let value = match expr {
        PrimaryExpr::Ident(name) => {
            Value::InMemory(func.get_local_idx_from_name(name, span.clone())?)
        }
        PrimaryExpr::Num(n) => Value::Const(ConstExpr::Numeric(NumericConst::I64(
            n.parse::<i64>().unwrap(),
        ))),
        PrimaryExpr::Bool(b) => Value::Const(ConstExpr::Bool(*b)),
        PrimaryExpr::Str(_) => todo!(),
    };

    Ok(RValInstruct::Use(value, Some(span)))
}

fn lower_expression_type(
    func: &Function,
    expr: &ExprStmtKind,
    ty_hint: Option<MathicType>,
    span: Span,
) -> Result<MathicType, LoweringError> {
    Ok(match expr {
        ExprStmtKind::Primary(primary_expr) => match primary_expr {
            PrimaryExpr::Ident(name) => func.get_local_from_name(name, span.clone())?.ty,
            PrimaryExpr::Num(_) => match ty_hint {
                Some(ty) => ty,
                None => MathicType::Sint(SintTy::I32),
            },
            PrimaryExpr::Str(_) => todo!(),
            PrimaryExpr::Bool(_) => MathicType::Bool,
        },
        ExprStmtKind::Binary { lhs, .. } => {
            lower_expression_type(func, &lhs.kind, None, span.clone())?
        }
        ExprStmtKind::Call { callee: _, .. } => todo!(),
        ExprStmtKind::Group(expr_stmt) => lower_expression_type(func, &expr_stmt.kind, None, span)?,
        ExprStmtKind::Index { .. } => todo!(),
        ExprStmtKind::Logical { .. } => MathicType::Bool,
        ExprStmtKind::Unary { rhs, .. } => lower_expression_type(func, &rhs.kind, None, span)?,
        ExprStmtKind::Assign { expr, .. } => lower_expression_type(func, &expr.kind, None, span)?,
    })
}
