use std::collections::HashMap;

use crate::{
    diagnostics::LoweringError,
    lowering::ir::{
        basic_block::Terminator,
        function::{FunctionBuilder, LocalKind},
        instruction::{InitInstruct, LValInstruct, RValInstruct, RValueKind},
        types::{FloatTy, MathicType, SintTy, UintTy, lower_inner_ast_type},
        value::{ConstExpr, NumericConst, Value, ValueModifier},
    },
    parser::{
        Span,
        ast::expression::{
            BinaryOp, ExprStmt, ExprStmtKind, InitExpr, LogicalOp, PrimaryExpr, UnaryOp,
        },
    },
};

pub fn lower_expr(
    func: &mut FunctionBuilder,
    expr: &ExprStmt,
    ty_hint: Option<MathicType>,
) -> Result<RValInstruct, LoweringError> {
    let rvalue = match &expr.kind {
        ExprStmtKind::Primary(val) => lower_primary_value(func, val, expr.span, ty_hint.clone())?,
        ExprStmtKind::Binary { lhs, op, rhs } => lower_binary_op(func, lhs, *op, rhs, expr.span)?,
        ExprStmtKind::Unary { op, rhs } => {
            lower_unary_op(func, *op, rhs, expr.span, ty_hint.clone())?
        }
        ExprStmtKind::Group(expr) => {
            return lower_expr(func, expr, ty_hint.clone());
        }
        ExprStmtKind::Call { callee, args } => lower_call(func, callee.clone(), args, expr.span)?,
        ExprStmtKind::Assign {
            name,
            expr: assign_expr,
        } => lower_assignment(func, name, assign_expr, expr.span)?,
        ExprStmtKind::Logical { lhs, op, rhs } => lower_logical_op(func, lhs, *op, rhs, expr.span)?,
        ExprStmtKind::Init(init_expr) => {
            lower_init_expr(func, init_expr, &ty_hint.clone().unwrap(), expr.span)?
        }
        ExprStmtKind::Index { .. } => todo!(),
        ExprStmtKind::StructGet {
            expr: struct_expr,
            field_name,
        } => lower_struct_get(func, struct_expr, field_name, expr.span, ty_hint.clone())?,
        ExprStmtKind::StructSet {
            lhs,
            field_name,
            rhs,
        } => lower_struct_set(func, lhs, field_name, rhs, expr.span)?,
    };

    Ok(rvalue)
}

fn lower_assignment(
    func: &mut FunctionBuilder,
    name: &str,
    expr: &ExprStmt,
    span: Span,
) -> Result<RValInstruct, LoweringError> {
    let local = func.sym_table.get_local_from_name(name, span)?;
    let value = lower_expr(func, expr, Some(local.ty.clone()))?;

    // The new value should be of the same type as the local's.
    if local.ty != value.ty {
        return Err(LoweringError::MismatchedType {
            expected: local.ty,
            found: value.ty,
            span,
        });
    }

    func.get_basic_block_mut(func.last_block_idx())
        .instructions
        .push(LValInstruct::Assign {
            local_idx: local.local_idx,
            value,
            modifier: vec![],
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
        let param_ty: MathicType = lower_inner_ast_type(func, &param.ty, param.span)?;
        let arg_val = lower_expr(func, arg, Some(param_ty.clone()))?;

        if arg_val.ty != param_ty {
            return Err(LoweringError::MismatchedType {
                expected: param_ty,
                found: arg_val.ty,
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
        Some(ty) => lower_inner_ast_type(func, &ty, span)?,
        None => MathicType::Void,
    };
    let local_idx = func
        .sym_table
        .add_local(None, return_ty.clone(), None, LocalKind::Temp)?;

    let dest_block_idx = func.last_block_idx() + 1;

    func.get_basic_block_mut(func.last_block_idx()).terminator = Terminator::Call {
        callee,
        args: arg_values,
        span: Some(span),
        return_dest: Value::InMemory {
            local_idx,
            modifier: vec![],
        },
        return_ty: return_ty.clone(),
        dest_block: dest_block_idx,
    };

    func.add_block(Terminator::Return(None, None), None);

    Ok(RValInstruct {
        kind: RValueKind::Use {
            value: Value::InMemory {
                local_idx,
                modifier: vec![],
            },
            span: None,
        },
        ty: return_ty,
    })
}

fn lower_binary_op(
    func: &mut FunctionBuilder,
    lhs: &ExprStmt,
    op: BinaryOp,
    rhs: &ExprStmt,
    span: Span,
) -> Result<RValInstruct, LoweringError> {
    let lhs = lower_expr(func, lhs, None)?;
    let rhs = lower_expr(func, rhs, Some(lhs.ty.clone()))?;

    if lhs.ty != rhs.ty {
        return Err(LoweringError::MismatchedType {
            expected: lhs.ty.clone(),
            found: rhs.ty.clone(),
            span,
        });
    }

    let inst_ty = match op {
        BinaryOp::Compare(_) => MathicType::Bool,
        BinaryOp::Arithmetic(_) => lhs.ty.clone(),
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
    let lhs = lower_expr(func, lhs, None)?;
    let rhs = lower_expr(func, rhs, Some(lhs.ty.clone()))?;

    if !lhs.ty.is_bool() {
        return Err(LoweringError::MismatchedType {
            expected: MathicType::Bool,
            found: lhs.ty.clone(),
            span,
        });
    }
    if !rhs.ty.is_bool() {
        return Err(LoweringError::MismatchedType {
            expected: MathicType::Bool,
            found: rhs.ty.clone(),
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
    let rhs = lower_expr(func, rhs, ty_hint)?;
    let rhs_ty = rhs.ty.clone();

    Ok(RValInstruct {
        kind: RValueKind::Unary {
            op,
            rhs: Box::new(rhs),
            span,
        },
        ty: rhs_ty,
    })
}

fn lower_init_expr(
    func: &mut FunctionBuilder,
    init: &InitExpr,
    expr_ty: &MathicType,
    span: Span,
) -> Result<RValInstruct, LoweringError> {
    match init {
        InitExpr::StructInit { name, fields } => lower_adt_init(func, name, fields, span),
        InitExpr::ArrayInit { elements } => lower_array_init(func, elements, expr_ty, span),
    }
}

fn lower_array_init(
    func: &mut FunctionBuilder,
    elements: &[ExprStmt],
    ty: &MathicType,
    span: Span,
) -> Result<RValInstruct, LoweringError> {
    let MathicType::Array { inner_ty, .. } = ty else {
        panic!()
    };

    let mut lowered_elements = Vec::with_capacity(elements.len());

    for expr in elements.iter() {
        let rvalue = lower_expr(func, expr, Some(*inner_ty.clone()))?;

        if **inner_ty != rvalue.ty {
            return Err(LoweringError::MismatchedType {
                expected: *inner_ty.clone(),
                found: rvalue.ty,
                span: expr.span,
            });
        }
        lowered_elements.push(rvalue);
    }

    let inner_ty = if lowered_elements.is_empty() {
        MathicType::Void
    } else {
        lowered_elements[0].ty.clone()
    };

    let lowered_elements_len = lowered_elements.len() as u32;

    Ok(RValInstruct {
        kind: RValueKind::Init {
            init_inst: InitInstruct::ArrayInit {
                elements: lowered_elements,
            },
            span,
        },
        ty: MathicType::Array {
            inner_ty: Box::new(inner_ty),
            length: lowered_elements_len,
        },
    })
}

fn lower_adt_init(
    func: &mut FunctionBuilder,
    name: &str,
    fields: &HashMap<String, ExprStmt>,
    span: Span,
) -> Result<RValInstruct, LoweringError> {
    let adt_ty = func.get_user_def_type(name, span)?;
    let adt_body = func.get_adt_body(adt_ty.clone(), span)?.clone();
    let mut init_fields = vec![None; fields.len()];
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
        let field_ty = adt_body
            .get_field_ty(name)
            .ok_or(LoweringError::UndeclaredStructField {
                found: name.to_string(),
                span,
            })?;
        let rvalue = lower_expr(func, expr, Some(field_ty.clone()))?;

        if field_ty != rvalue.ty {
            return Err(LoweringError::MismatchedType {
                expected: field_ty,
                found: rvalue.ty,
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

        init_fields[field_idx] = Some(rvalue);
    }

    Ok(RValInstruct {
        kind: RValueKind::Init {
            init_inst: InitInstruct::StructInit {
                fields: init_fields.into_iter().map(Option::unwrap).collect(),
            },
            span,
        },
        ty: adt_ty,
    })
}

fn lower_struct_get(
    func: &mut FunctionBuilder,
    expr: &ExprStmt,
    field_name: &str,
    span: Span,
    ty_hint: Option<MathicType>,
) -> Result<RValInstruct, LoweringError> {
    let struct_expr = lower_expr(func, expr, ty_hint)?;
    let struct_ty = struct_expr.ty.clone();

    let RValueKind::Use { value, .. } = struct_expr.kind else {
        unreachable!()
    };
    let Value::InMemory {
        local_idx,
        mut modifier,
    } = value
    else {
        unreachable!()
    };

    let struct_adt = func.get_adt_body(struct_ty, expr.span)?;
    let field_index =
        struct_adt
            .get_field_index(field_name)
            .ok_or(LoweringError::UndeclaredStructField {
                found: field_name.to_string(),
                span: expr.span,
            })?;
    let field_ty =
        struct_adt
            .get_field_ty(field_name)
            .ok_or(LoweringError::UndeclaredStructField {
                found: field_name.to_string(),
                span: expr.span,
            })?;

    modifier.push(ValueModifier::Field(field_index));

    Ok(RValInstruct {
        kind: RValueKind::Use {
            value: Value::InMemory {
                local_idx,
                modifier,
            },
            span: Some(span),
        },
        ty: field_ty,
    })
}

fn lower_struct_set(
    func: &mut FunctionBuilder,
    lhs: &ExprStmt,
    field_name: &str,
    rhs: &ExprStmt,
    span: Span,
) -> Result<RValInstruct, LoweringError> {
    let struct_field_value = lower_struct_get(func, lhs, field_name, span, None)?;
    let field_ty = struct_field_value.ty.clone();
    let (local_idx, modifier) = {
        let RValueKind::Use { value, .. } = struct_field_value.kind else {
            unreachable!()
        };
        let Value::InMemory {
            local_idx,
            modifier,
        } = value
        else {
            unreachable!()
        };

        (local_idx, modifier)
    };
    let value = lower_expr(func, rhs, Some(field_ty.clone()))?;

    if value.ty != field_ty {
        return Err(LoweringError::MismatchedType {
            expected: field_ty,
            found: value.ty,
            span,
        });
    }

    func.get_basic_block_mut(func.last_block_idx())
        .instructions
        .push(LValInstruct::Assign {
            local_idx,
            value,
            modifier,
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

fn lower_primary_value(
    func: &mut FunctionBuilder,
    expr: &PrimaryExpr,
    span: Span,
    ty_hint: Option<MathicType>,
) -> Result<RValInstruct, LoweringError> {
    let (value, ty) = match expr {
        PrimaryExpr::Ident(name) => {
            let local = func.sym_table.get_local_from_name(name, span)?;
            (
                Value::InMemory {
                    local_idx: local.local_idx,
                    modifier: vec![],
                },
                local.ty,
            )
        }
        PrimaryExpr::Num(n) => match ty_hint {
            Some(ty) => (
                Value::Const(match ty {
                    MathicType::Uint(uint_ty) => match uint_ty {
                        UintTy::Usize => {
                            ConstExpr::Numeric(NumericConst::Usize(n.parse::<usize>().unwrap()))
                        }
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
                        SintTy::Isize => {
                            ConstExpr::Numeric(NumericConst::Isize(n.parse::<isize>().unwrap()))
                        }
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
                    | MathicType::Adt { .. }
                    | MathicType::Array { .. } => {
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
