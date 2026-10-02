use crate::{
    diagnostics::LoweringError,
    lowering::{
        ir::{
            Builder,
            symbols::TypeIndex,
            types::{MathicType, NumericTy, SintTy, UintTy},
        },
        utils::get_or_insert_struct_type,
    },
    parser::{Span, ast::declaration::AstType},
};

pub mod control_flow;
pub mod declaration;
pub mod expression;
pub mod statement;

pub fn lower_ast_type(
    builder: &mut dyn Builder,
    ty: &AstType,
    span: Span,
) -> Result<TypeIndex, LoweringError> {
    Ok(match ty {
        AstType::Type { ty, inner } => {
            match ty.as_str() {
                "isz" => builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Sint(SintTy::Isize))),
                "i8" => {
                    builder.get_or_insert_type_idx(MathicType::Numeric(NumericTy::Sint(SintTy::I8)))
                }
                "i16" => builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Sint(SintTy::I16))),
                "i32" => builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Sint(SintTy::I32))),
                "i64" => builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Sint(SintTy::I64))),
                "i128" => builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Sint(SintTy::I128))),
                "usz" => builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Uint(UintTy::Usize))),
                "u8" => {
                    builder.get_or_insert_type_idx(MathicType::Numeric(NumericTy::Uint(UintTy::U8)))
                }
                "u16" => builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Uint(UintTy::U16))),
                "u32" => builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Uint(UintTy::U32))),
                "u64" => builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Uint(UintTy::U64))),
                "u128" => builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Uint(UintTy::U128))),
                "str" => builder.get_or_insert_type_idx(MathicType::Str),
                "char" => builder.get_or_insert_type_idx(MathicType::Char),
                "bool" => builder.get_or_insert_type_idx(MathicType::Bool),
                "expr" => {
                    let Some(inner_ty) = inner else {
                        return Err(LoweringError::TypeRequiresTypeParameter {
                            name: ty.clone(),
                            span,
                        });
                    };
                    let inner_ty_idx = lower_ast_type(builder, inner_ty, span)?;
                    let inner_ty = builder.get_type(inner_ty_idx, span)?;

                    match inner_ty {
                        MathicType::Numeric(num_ty) => {
                            builder.get_or_insert_type_idx(MathicType::SymbolicExpr(num_ty))
                        }
                        other => {
                            return Err(LoweringError::MismatchedType {
                                expected: other,
                                found: other,
                                span,
                            });
                        }
                    }
                }
                other => {
                    if let Some(ty) = builder.get_user_def_type(other) {
                        return Ok(ty);
                    }

                    let (s, module_idx) = builder.get_struct_decl(other, span)?;

                    let builder = if module_idx.is_some() {
                        builder.get_ir_builder()
                    } else {
                        builder
                    };

                    get_or_insert_struct_type(builder, &s, module_idx, span)?
                }
            }
        }
        AstType::SelfType => builder
            .get_self_ty_idx()
            .ok_or(LoweringError::NoAssociatedSelfType { span })?,
    })
}
