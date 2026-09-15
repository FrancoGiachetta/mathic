use crate::{
    diagnostics::LoweringError,
    lowering::{
        ir::{Builder, Ir, function::Function, symbols::TypeIndex},
        utils::get_or_insert_struct_type,
    },
    parser::{Span, ast::declaration::AstType},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UintTy {
    Usize,
    U8,
    U16,
    U32,
    U64,
    U128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SintTy {
    Isize,
    I8,
    I16,
    I32,
    I64,
    I128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatTy {
    F32,
    F64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumericTy {
    Sint(SintTy),
    Uint(UintTy),
    Float(FloatTy),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MathicType {
    Numeric(NumericTy),
    Adt { index: usize, is_local: bool },
    Bool,
    Char,
    Str,
    SymbolicExpr(NumericTy),
    Void,
}

pub fn lower_ast_type(
    builder: &mut impl Builder,
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

                    match builder.get_struct_decl(other).cloned() {
                        Some((s, module_idx)) => get_or_insert_struct_type(
                            builder.get_ir_builder(),
                            &s,
                            module_idx,
                            span,
                        )?,
                        None => {
                            return Err(LoweringError::UndeclaredType { span });
                        }
                    }
                }
            }
        }
    })
}

// pub fn lower_ast_type(
//     builder: &mut impl Builder,
//     ty: &AstType,
//     span: Span,
// ) -> Result<TypeIndex, LoweringError> {
//     Ok(match ty {
//         AstType::Type { ty, inner } => {
//             match ty.as_str() {
//                 "isz" => builder
//                     .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Sint(SintTy::Isize))),
//                 "i8" => {
//                     builder.get_or_insert_type_idx(MathicType::Numeric(NumericTy::Sint(SintTy::I8)))
//                 }
//                 "i16" => builder
//                     .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Sint(SintTy::I16))),
//                 "i32" => builder
//                     .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Sint(SintTy::I32))),
//                 "i64" => builder
//                     .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Sint(SintTy::I64))),
//                 "i128" => builder
//                     .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Sint(SintTy::I128))),
//                 "usz" => builder
//                     .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Uint(UintTy::Usize))),
//                 "u8" => {
//                     builder.get_or_insert_type_idx(MathicType::Numeric(NumericTy::Uint(UintTy::U8)))
//                 }
//                 "u16" => builder
//                     .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Uint(UintTy::U16))),
//                 "u32" => builder
//                     .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Uint(UintTy::U32))),
//                 "u64" => builder
//                     .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Uint(UintTy::U64))),
//                 "u128" => builder
//                     .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Uint(UintTy::U128))),
//                 "str" => builder.get_or_insert_type_idx(MathicType::Str),
//                 "char" => builder.get_or_insert_type_idx(MathicType::Char),
//                 "bool" => builder.get_or_insert_type_idx(MathicType::Bool),
//                 "expr" => {
//                     let Some(inner_ty) = inner else { panic!() };
//                     let inner_ty_idx = lower_ast_type(builder, inner_ty, span)?;
//                     let inner_ty = builder.get_type(inner_ty_idx, span)?;

//                     match inner_ty {
//                         MathicType::Numeric(num_ty) => {
//                             builder.get_or_insert_type_idx(MathicType::SymbolicExpr(num_ty))
//                         }
//                         other => {
//                             return Err(LoweringError::MismatchedType {
//                                 expected: other,
//                                 found: other,
//                                 span,
//                             });
//                         }
//                     }
//                 }
//                 other => {
//                     if let Some(ty) = builder.get_user_def_type(other, span) {
//                         return Ok(ty);
//                     }

//                     match builder.get_struct_decl(other).cloned() {
//                         Some((s, module_idx)) => {
//                             utils::get_or_insert_struct_type(builder, &s, module_idx, span)?
//                         }
//                         None => {
//                             return Err(LoweringError::UndeclaredType { span });
//                         }
//                     }
//                 }
//             }
//         }
//     })
// }

impl NumericTy {
    pub fn is_signed(&self) -> bool {
        matches!(self, Self::Sint(_) | Self::Float(_))
    }

    pub fn bit_width(&self) -> u32 {
        match self {
            Self::Sint(ty) => match ty {
                SintTy::Isize => isize::BITS,
                SintTy::I8 => 8,
                SintTy::I16 => 16,
                SintTy::I32 => 32,
                SintTy::I64 => 64,
                SintTy::I128 => 128,
            },
            Self::Uint(ty) => match ty {
                UintTy::Usize => usize::BITS,
                UintTy::U8 => 8,
                UintTy::U16 => 16,
                UintTy::U32 => 32,
                UintTy::U64 => 64,
                UintTy::U128 => 128,
            },
            Self::Float(ty) => match ty {
                FloatTy::F32 => 32,
                FloatTy::F64 => 64,
            },
        }
    }
}

impl MathicType {
    pub fn bit_width(&self) -> u32 {
        match self {
            Self::Numeric(inner) => inner.bit_width(),
            Self::Bool => 1,
            Self::Char => 8,
            Self::Void => 0,
            Self::Str | Self::SymbolicExpr(_) | Self::Adt { .. } => todo!(),
        }
    }

    /// Returns the align of a type expressed in bits.
    pub fn align(&self, ir: &Ir, func: &Function) -> usize {
        match self {
            Self::Numeric(inner) => inner.bit_width() as usize,
            Self::Bool => 1,
            Self::Str => 8,
            Self::Char => 8,
            Self::SymbolicExpr(_) => 0,
            Self::Void => 0,
            Self::Adt { index, is_local } => {
                let adt_fields_tys: Vec<MathicType> = {
                    if *is_local {
                        let adt = func
                            .get_adt(*index)
                            .expect("internal error: invalid local ADT index in type alignment");
                        adt.get_fields_tys()
                            .iter()
                            .map(|t| {
                                if t.is_local {
                                    func.get_type(t.idx)
                                } else {
                                    ir.get_type(t.idx)
                                }
                                .expect(
                                    "internal error: invalid local type index in type alignment",
                                )
                            })
                            .collect()
                    } else {
                        let adt = ir
                            .get_adt(*index)
                            .expect("internal error: invalid global ADT index in type alignment");
                        adt.get_fields_tys()
                            .iter()
                            .map(|t| {
                                ir.get_type(t.idx).expect(
                                    "internal error: invalid global type index in type alignment",
                                )
                            })
                            .collect()
                    }
                };

                let mut align = 0;

                for ty in adt_fields_tys.iter() {
                    align = align.max(ty.align(ir, func));
                }

                align
            }
        }
    }

    #[inline(always)]
    pub fn is_signed(&self) -> bool {
        matches!(
            self,
            Self::Numeric(NumericTy::Sint(_) | NumericTy::Float(_))
        )
    }

    #[inline(always)]
    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Numeric(NumericTy::Sint(_) | NumericTy::Uint(_)))
    }

    #[inline(always)]
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Numeric(NumericTy::Float(_)))
    }

    #[inline(always)]
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool)
    }

    #[inline(always)]
    pub fn is_symbolic(&self) -> bool {
        matches!(self, Self::SymbolicExpr(_))
    }
}

/// Resolves a struct type.
///
/// Order of resolution:
/// 1. Types already registered in the function's symbol table, i.e. function
///    local ADTs and top-level structs previously seen in the same module.
/// 2. External structs keyed by their mangled name (`module::Struct`).
/// 3. Otherwise the struct is registered on demand.
pub fn resolve_struct_type(
    builder: &mut impl Builder,
    name: &str,
    span: Span,
) -> Result<TypeIndex, LoweringError> {
    if let Some(ty) = builder.get_user_def_type(name) {
        return Ok(ty);
    }

    let Some((strct, module_idx)) = builder.get_struct_decl(name).cloned() else {
        return Err(LoweringError::UndeclaredType { span });
    };

    let key = match module_idx {
        None => strct.name.clone(),
        Some(idx) => {
            let module = builder
                .get_module(idx)
                .unwrap_or_else(|| panic!("module index {} should be valid", idx));

            builder.get_mangled_name(&module.module_name, &strct.name)
        }
    };

    // The struct may already be resolved (e.g. by a prior struct init or a
    // type annotation), keyed by its mangled name.
    if let Some(ty) = builder.get_user_def_type(&key) {
        return Ok(ty);
    }

    get_or_insert_struct_type(builder, &strct, module_idx, span)
}
