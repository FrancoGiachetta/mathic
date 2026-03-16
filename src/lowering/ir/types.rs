use crate::{
    diagnostics::LoweringError,
    lowering::{
        ast_lowering::declaration::lower_inner_struct,
        ir::{
            Ir,
            function::{Function, FunctionBuilder},
        },
        lower_top_level_struct,
    },
    parser::{Span, ast::declaration::AstType},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UintTy {
    U8,
    U16,
    U32,
    U64,
    U128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SintTy {
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
pub enum MathicType {
    Adt { index: usize, is_local: bool },
    Bool,
    Char,
    Float(FloatTy),
    Str,
    Uint(UintTy),
    Sint(SintTy),
    Void,
}

pub fn lower_inner_ast_type(
    func_builder: &mut FunctionBuilder,
    ty: &AstType,
    span: Span,
) -> Result<MathicType, LoweringError> {
    Ok(match ty {
        AstType::Type(name) => match name.as_str() {
            "i8" => MathicType::Sint(SintTy::I8),
            "i16" => MathicType::Sint(SintTy::I16),
            "i32" => MathicType::Sint(SintTy::I32),
            "i64" => MathicType::Sint(SintTy::I64),
            "i128" => MathicType::Sint(SintTy::I128),
            "u8" => MathicType::Uint(UintTy::U8),
            "u16" => MathicType::Uint(UintTy::U16),
            "u32" => MathicType::Uint(UintTy::U32),
            "u64" => MathicType::Uint(UintTy::U64),
            "u128" => MathicType::Uint(UintTy::U128),
            "str" => MathicType::Str,
            "char" => MathicType::Char,
            "bool" => MathicType::Bool,
            other => {
                if let Ok(ty) = func_builder.get_user_def_type(other, span) {
                    return Ok(ty);
                }

                match func_builder
                    .ir_builder
                    .decl_table
                    .get_struct_decl(other)
                    .cloned()
                {
                    Some(d) => MathicType::Adt {
                        index: lower_top_level_struct(func_builder.ir_builder, &d)?,
                        is_local: false,
                    },
                    None => match func_builder.decl_table.get_struct_decl(other).cloned() {
                        Some(d) => MathicType::Adt {
                            index: lower_inner_struct(func_builder, &d)?,
                            is_local: true,
                        },
                        None => {
                            return Err(LoweringError::UndeclaredType { span });
                        }
                    },
                }
            }
        },
    })
}

impl MathicType {
    pub fn bit_width(&self) -> usize {
        match self {
            Self::Sint(ty) => match ty {
                SintTy::I8 => 8,
                SintTy::I16 => 16,
                SintTy::I32 => 32,
                SintTy::I64 => 64,
                SintTy::I128 => 128,
            },
            MathicType::Uint(ty) => match ty {
                UintTy::U8 => 8,
                UintTy::U16 => 16,
                UintTy::U32 => 32,
                UintTy::U64 => 64,
                UintTy::U128 => 128,
            },
            MathicType::Float(ty) => match ty {
                FloatTy::F32 => 32,
                FloatTy::F64 => 64,
            },
            Self::Bool => 1,
            Self::Char => 8,
            Self::Void => 0,
            Self::Str | Self::Adt { .. } => todo!(),
        }
    }

    pub fn align(&self, ir: &Ir, func: &Function) -> usize {
        match self {
            Self::Sint(ty) => match ty {
                SintTy::I8 => 8,
                SintTy::I16 => 16,
                SintTy::I32 => 32,
                SintTy::I64 => 64,
                SintTy::I128 => 128,
            },
            MathicType::Uint(ty) => match ty {
                UintTy::U8 => 8,
                UintTy::U16 => 16,
                UintTy::U32 => 32,
                UintTy::U64 => 64,
                UintTy::U128 => 128,
            },
            MathicType::Float(ty) => match ty {
                FloatTy::F32 => 32,
                FloatTy::F64 => 64,
            },
            Self::Bool => 1,
            Self::Str => 8,
            Self::Char => 8,
            Self::Void => 0,
            Self::Adt { index, is_local } => {
                let adt = if *is_local {
                    func.sym_table.get_adt(*index).unwrap()
                } else {
                    ir.adts.get(*index).unwrap()
                };
                let adt_fields_tys = adt.get_fields_tys();
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
        matches!(self, Self::Sint(_))
    }

    #[inline(always)]
    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Sint(_) | Self::Uint(_))
    }

    #[inline(always)]
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    #[inline(always)]
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool)
    }
}
