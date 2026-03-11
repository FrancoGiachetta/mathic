use std::collections::HashMap;

use crate::{
    diagnostics::LoweringError,
    lowering::ir::{
        basic_block::Terminator,
        function::{FunctionBuilder, LocalKind},
        instruction::{InitInstruc, LValInstruct, RValInstruct, RValueKind},
        types::{FloatTy, MathicType, SintTy, UintTy, lower_ast_type},
        value::{ConstExpr, NumericConst, Value},
    },
    parser::{
        Span,
        ast::expression::{BinaryOp, ExprStmt, ExprStmtKind, LogicalOp, PrimaryExpr, UnaryOp},
    },
};

pub fn lower_expr(
    func: &mut FunctionBuilder,
    expr: &ExprStmt,
    ty_hint: Option<MathicType>,
) -> Result<(RValInstruct, MathicType), LoweringError> {
    let rvalue = match &expr.kind {
        ExprStmtKind::Primary(val) => lower_primary_value(func, val, expr.span, ty_hint)?,
        ExprStmtKind::Binary { lhs, op, rhs } => lower_binary_op(func, lhs, *op, rhs, expr.span)?,
        ExprStmtKind::Unary { op, rhs } => lower_unary_op(func, *op, rhs, expr.span, ty_hint)?,
        ExprStmtKind::Group(expr) => {
            return lower_expr(func, expr, ty_hint);
        }
        ExprStmtKind::Call { callee, args } => lower_call(func, callee.clone(), args, expr.span)?,
        ExprStmtKind::Assign {
            name,
            expr: assign_expr,
        } => lower_assignment(func, name, assign_expr, expr.span)?,
        ExprStmtKind::Logical { lhs, op, rhs } => lower_logical_op(func, lhs, *op, rhs, expr.span)?,
        ExprStmtKind::StructInit { name, fields } => lower_adt_init(func, name, fields, expr.span)?,
        ExprStmtKind::Index { .. } => todo!(),
    };

    Ok((
        rvalue,
        lower_expression_type(func, &expr.kind, ty_hint, expr.span)?,
    ))
}

fn lower_assignment(
    func: &mut FunctionBuilder,
    name: &str,
    expr: &ExprStmt,
    span: Span,
) -> Result<RValInstruct, LoweringError> {
    let local = func.sym_table.get_local_from_name(name, span)?;
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
    func: &mut FunctionBuilder,
    callee: String,
    func_args: &[ExprStmt],
    span: Span,
) -> Result<RValInstruct, LoweringError> {
    let mut arg_values: Vec<RValInstruct> = Vec::new();
    let func_prototype = func.get_function_decl(&callee, span)?;

    if func_prototype.params.len() != func_args.len() {
        return Err(LoweringError::WrongArgumentCount {
            name: callee.to_string(),
            expected: func_prototype.params.len(),
            got: func_args.len(),
            span,
        });
    }

    for (arg, param) in func_args.iter().zip(func_prototype.params.iter()) {
        let param_ty: MathicType = lower_ast_type(func, &param.ty, param.span)?;
        let (arg_val, arg_ty) = lower_expr(func, arg, Some(param_ty))?;

        if arg_ty != param_ty {
            return Err(LoweringError::MismatchedType {
                expected: param_ty,
                found: arg_ty,
                span: arg.span,
            });
        }

        arg_values.push(arg_val);
    }

    // Since we represent function calls as expressions, we need to return a
    // value. Due to the fact that function calls are actually terminators and
    // not RValue instructions, we need to create a temporary local to store
    // the return value and then create the RValue instruction pointing to that
    // new local.
    let return_ty = match func_prototype.return_ty {
        Some(ty) => lower_ast_type(func, &ty, span)?,
        None => MathicType::Void,
    };
    let local_idx = func
        .sym_table
        .add_local(None, return_ty, None, LocalKind::Temp)?;

    let dest_block_idx = func.last_block_idx() + 1;

    func.get_basic_block_mut(func.last_block_idx()).terminator = Terminator::Call {
        callee,
        args: arg_values,
        span: Some(span),
        return_dest: Value::InMemory(local_idx),
        return_ty,
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
    func: &mut FunctionBuilder,
    lhs: &ExprStmt,
    op: BinaryOp,
    rhs: &ExprStmt,
    span: Span,
) -> Result<RValInstruct, LoweringError> {
    let (lhs, lhs_ty) = lower_expr(func, lhs, None)?;
    let (rhs, rhs_ty) = lower_expr(func, rhs, Some(lhs_ty))?;

    // Operands' types must match.
    if lhs_ty != rhs_ty {
        return Err(LoweringError::MismatchedType {
            expected: lhs_ty,
            found: rhs_ty,
            span,
        });
    }
    let inst_ty = match op {
        BinaryOp::Compare(_) => MathicType::Bool,
        BinaryOp::Arithmetic(_) => lhs_ty,
    };

    Ok(RValInstruct {
        kind: RValueKind::Binary {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            span,
        },
        ty: inst_ty,
    })
}

fn lower_logical_op(
    func: &mut FunctionBuilder,
    lhs: &ExprStmt,
    op: LogicalOp,
    rhs: &ExprStmt,
    span: Span,
) -> Result<RValInstruct, LoweringError> {
    let (lhs, lhs_ty) = lower_expr(func, lhs, None)?;
    let (rhs, rhs_ty) = lower_expr(func, rhs, Some(lhs_ty))?;

    // Operands' types must be boolean.
    if !lhs_ty.is_bool() {
        return Err(LoweringError::MismatchedType {
            expected: MathicType::Bool,
            found: lhs_ty,
            span,
        });
    }
    if !rhs_ty.is_bool() {
        return Err(LoweringError::MismatchedType {
            expected: MathicType::Bool,
            found: rhs_ty,
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
    func: &mut FunctionBuilder,
    op: UnaryOp,
    rhs: &ExprStmt,
    span: Span,
    ty_hint: Option<MathicType>,
) -> Result<RValInstruct, LoweringError> {
    let (rhs, rhs_ty) = lower_expr(func, rhs, ty_hint)?;

    Ok(RValInstruct {
        kind: RValueKind::Unary {
            op,
            rhs: Box::new(rhs),
            span,
        },
        ty: rhs_ty,
    })
}

fn lower_adt_init(
    func: &mut FunctionBuilder,
    name: &str,
    fields: &HashMap<String, ExprStmt>,
    span: Span,
) -> Result<RValInstruct, LoweringError> {
    let adt_ty = func.get_user_def_type(name, span)?;
    let adt_body = func.get_adt_body(name, span)?.clone();
    let mut init_fields = Vec::with_capacity(fields.len());

    if fields.len() != adt_body.fields_len() {
        let adt_fields_names = adt_body.get_field_names();
        let missing = adt_fields_names
            .into_iter()
            .filter(|n| fields.contains_key(n))
            .collect::<Vec<_>>()
            .join(", ");

        return Err(LoweringError::MissingStructFields { missing, span });
    }

    for (name, expr) in fields {
        let (rvalue, rvalue_ty) = lower_expr(func, expr, adt_body.get_field_ty(name))?;
        let field_ty = adt_body
            .get_field_ty(name)
            .ok_or(LoweringError::UndeclaredStructField {
                found: name.to_string(),
                span,
            })?;

        if field_ty != rvalue_ty {
            return Err(LoweringError::MismatchedType {
                expected: field_ty,
                found: rvalue_ty,
                span,
            });
        }

        let field_idx =
            adt_body
                .get_field_index(name)
                .ok_or(LoweringError::UndeclaredStructField {
                    found: name.to_string(),
                    span,
                })?;

        init_fields.insert(field_idx, rvalue);
    }

    Ok(RValInstruct {
        kind: RValueKind::Init(InitInstruc::StructInit {
            fields: init_fields,
        }),
        ty: adt_ty,
    })
}

fn lower_primary_value(
    func: &mut FunctionBuilder,
    expr: &PrimaryExpr,
    span: Span,
    ty_hint: Option<MathicType>,
) -> Result<RValInstruct, LoweringError> {
    let (value, ty) = match expr {
        PrimaryExpr::Ident(name) => {
            let local = func.sym_table.get_local_from_name(name, span)?;
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
                    MathicType::Bool
                    | MathicType::Void
                    | MathicType::Char
                    | MathicType::Str
                    | MathicType::Adt { .. } => {
                        unreachable!()
                    }
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
        PrimaryExpr::Str(s) => (Value::Const(ConstExpr::Str(s.clone())), MathicType::Str),
        PrimaryExpr::Char(c) => (Value::Const(ConstExpr::Char(*c)), MathicType::Char),
    };

    Ok(RValInstruct {
        kind: RValueKind::Use {
            value,
            span: Some(span),
        },
        ty,
    })
}

/// Tries to infer the type of an expression.
///
/// A **ty_hint** may be provided to help guessing the type of expressions such
/// as numeric constants, whose type depend on the bit width declared. In such
/// cases, if no **ty_hint** was provided, the default type will be returned.
fn lower_expression_type(
    func: &FunctionBuilder,
    expr: &ExprStmtKind,
    ty_hint: Option<MathicType>,
    span: Span,
) -> Result<MathicType, LoweringError> {
    Ok(match expr {
        ExprStmtKind::Primary(primary_expr) => match primary_expr {
            PrimaryExpr::Ident(name) => func.sym_table.get_local_from_name(name, span)?.ty,
            PrimaryExpr::Num(_) => match ty_hint {
                Some(ty) => ty,
                None => MathicType::Sint(SintTy::I32),
            },
            PrimaryExpr::Str(_) => MathicType::Str,
            PrimaryExpr::Char(_) => MathicType::Char,
            PrimaryExpr::Bool(_) => MathicType::Bool,
        },
        ExprStmtKind::Binary { lhs, op, .. } => match op {
            BinaryOp::Compare(_) => MathicType::Bool,
            BinaryOp::Arithmetic(_) => lower_expression_type(func, &lhs.kind, None, span)?,
        },
        ExprStmtKind::Call { callee: _, .. } => MathicType::Sint(SintTy::I64),
        ExprStmtKind::Group(expr_stmt) => lower_expression_type(func, &expr_stmt.kind, None, span)?,
        ExprStmtKind::Index { .. } => todo!(),
        ExprStmtKind::Logical { .. } => MathicType::Bool,
        ExprStmtKind::Unary { rhs, .. } => lower_expression_type(func, &rhs.kind, None, span)?,
        ExprStmtKind::Assign { expr, .. } => lower_expression_type(func, &expr.kind, None, span)?,
        ExprStmtKind::StructInit { name, .. } => func.get_user_def_type(name, span)?,
    })
}
