use crate::{
    diagnostics::LoweringError,
    lowering::ir::{
        basic_block::Terminator,
        function::{Function, LocalKind},
        instruction::{LValInstruct, RValInstruct, RValueKind},
        types::{FloatTy, MathicType, SintTy, UintTy},
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
        ExprStmtKind::Primary(val) => lower_primary_value(func, val, expr.span.clone(), ty_hint)?,
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

    Ok(RValInstruct {
        kind: RValueKind::Use {
            value: Value::Const(ConstExpr::Void),
            span: None,
        },
        ty: MathicType::Void,
    })
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

    Ok(RValInstruct {
        kind: RValueKind::Use {
            value: Value::InMemory(local_idx),
            span: None,
        },
        ty: MathicType::Sint(SintTy::I64),
    })
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

    Ok(RValInstruct {
        kind: RValueKind::Binary {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            span,
        },
        ty: lhs_ty,
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

    Ok(RValInstruct {
        kind: RValueKind::Logical {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            span,
        },
        ty: MathicType::Bool,
    })
}

fn lower_unary_op(
    func: &mut Function,
    op: UnaryOp,
    rhs: &ExprStmt,
    span: Span,
) -> Result<RValInstruct, LoweringError> {
    let (rhs, rhs_ty) = lower_expr(func, rhs, None)?;

    Ok(RValInstruct {
        kind: RValueKind::Unary {
            op,
            rhs: Box::new(rhs),
            span,
        },
        ty: rhs_ty,
    })
}

fn lower_primary_value(
    func: &mut Function,
    expr: &PrimaryExpr,
    span: Span,
    ty_hint: Option<MathicType>,
) -> Result<RValInstruct, LoweringError> {
    let (value, ty) = match expr {
        PrimaryExpr::Ident(name) => {
            let local = func.get_local_from_name(name, span.clone())?;
            (Value::InMemory(local.local_idx), local.ty)
        }
        PrimaryExpr::Num(n) => match ty_hint {
            Some(ty) => (
                Value::Const(match ty {
                    MathicType::Uint(uint_ty) => match uint_ty {
                        UintTy::U8 => {
                            ConstExpr::Numeric(NumericConst::U8(n.parse::<u8>().unwrap()))
                        }
                        UintTy::U16 => {
                            ConstExpr::Numeric(NumericConst::U16(n.parse::<u16>().unwrap()))
                        }
                        UintTy::U32 => {
                            ConstExpr::Numeric(NumericConst::U32(n.parse::<u32>().unwrap()))
                        }
                        UintTy::U64 => {
                            ConstExpr::Numeric(NumericConst::U64(n.parse::<u64>().unwrap()))
                        }
                        UintTy::U128 => {
                            ConstExpr::Numeric(NumericConst::U128(n.parse::<u128>().unwrap()))
                        }
                    },
                    MathicType::Sint(uint_ty) => match uint_ty {
                        SintTy::I8 => {
                            ConstExpr::Numeric(NumericConst::I8(n.parse::<i8>().unwrap()))
                        }
                        SintTy::I16 => {
                            ConstExpr::Numeric(NumericConst::I16(n.parse::<i16>().unwrap()))
                        }
                        SintTy::I32 => {
                            ConstExpr::Numeric(NumericConst::I32(n.parse::<i32>().unwrap()))
                        }
                        SintTy::I64 => {
                            ConstExpr::Numeric(NumericConst::I64(n.parse::<i64>().unwrap()))
                        }
                        SintTy::I128 => {
                            ConstExpr::Numeric(NumericConst::I128(n.parse::<i128>().unwrap()))
                        }
                    },
                    MathicType::Float(float_ty) => match float_ty {
                        FloatTy::F32 => {
                            ConstExpr::Numeric(NumericConst::F32(n.parse::<f32>().unwrap()))
                        }
                        FloatTy::F64 => {
                            ConstExpr::Numeric(NumericConst::F64(n.parse::<f64>().unwrap()))
                        }
                    },
                    MathicType::Bool => unreachable!(),
                    MathicType::Void => unreachable!(),
                }),
                ty,
            ),
            None => (
                Value::Const(ConstExpr::Numeric(NumericConst::I32(
                    n.parse::<i32>().unwrap(),
                ))),
                MathicType::Sint(SintTy::I32),
            ),
        },
        PrimaryExpr::Bool(b) => (Value::Const(ConstExpr::Bool(*b)), MathicType::Bool),
        PrimaryExpr::Str(_) => todo!(),
    };

    Ok(RValInstruct {
        kind: RValueKind::Use {
            value,
            span: Some(span),
        },
        ty,
    })
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
