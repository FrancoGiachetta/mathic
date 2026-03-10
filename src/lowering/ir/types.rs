use std::fmt;

use melior::{
    Context,
    dialect::llvm,
    ir::{Type, r#type::IntegerType},
};

use crate::{
    diagnostics::LoweringError,
    lowering::{ast_lowering::declaration::lower_inner_struct, ir::function::FunctionBuilder},
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
    Adt { index: usize },
    Bool,
    Char,
    Float(FloatTy),
    Str,
    Uint(UintTy),
    Sint(SintTy),
    Void,
}

pub fn lower_ast_type(
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
                if let Some(ty) = func_builder.ir_builder.get_user_def_type(other) {
                    return Ok(ty);
                }

                match func_builder
                    .ir_builder
                    .decl_table
                    .get_struct_decl(other)
                    .cloned()
                {
                    Some(d) => MathicType::Adt {
                        index: lower_inner_struct(func_builder, &d)?,
                    },
                    None => match func_builder.decl_table.get_struct_decl(other).cloned() {
                        Some(d) => MathicType::Adt {
                            index: lower_inner_struct(func_builder, &d)?,
                        },
                        None => {
                            return Err(LoweringError::UndeclaredType {
                                name: other.to_string(),
                                span,
                            });
                        }
                    },
                }
            }
        },
    })
}

impl MathicType {
    pub fn get_compiled_type<'func>(&'func self, ctx: &'func Context) -> Type<'func> {
        match self {
            Self::Uint(_) | Self::Sint(_) => IntegerType::new(ctx, self.bit_width() as u32).into(),
            MathicType::Float(float_ty) => match float_ty {
                FloatTy::F32 => Type::float32(ctx),
                FloatTy::F64 => Type::float64(ctx),
            },
            MathicType::Bool => IntegerType::new(ctx, 1).into(),
            MathicType::Char => IntegerType::new(ctx, 8).into(),
            MathicType::Str => llvm::r#type::pointer(ctx, 0),
            MathicType::Void => Type::none(ctx),
            MathicType::Adt { .. } => {
                todo!()
            }
        }
    }

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

    pub fn align(&self) -> usize {
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
            Self::Adt { .. } => {
                todo!()
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

impl fmt::Display for UintTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UintTy::U8 => write!(f, "u8"),
            UintTy::U16 => write!(f, "u16"),
            UintTy::U32 => write!(f, "u32"),
            UintTy::U64 => write!(f, "u64"),
            UintTy::U128 => write!(f, "u128"),
        }
    }
}

impl fmt::Display for SintTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SintTy::I8 => write!(f, "i8"),
            SintTy::I16 => write!(f, "i16"),
            SintTy::I32 => write!(f, "i32"),
            SintTy::I64 => write!(f, "i64"),
            SintTy::I128 => write!(f, "i128"),
        }
    }
}

impl fmt::Display for FloatTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FloatTy::F32 => write!(f, "f32"),
            FloatTy::F64 => write!(f, "f64"),
        }
    }
}

impl fmt::Display for MathicType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathicType::Uint(ty) => write!(f, "{}", ty),
            MathicType::Sint(ty) => write!(f, "{}", ty),
            MathicType::Float(ty) => write!(f, "{}", ty),
            MathicType::Bool => write!(f, "bool"),
            MathicType::Str => write!(f, "str"),
            MathicType::Char => write!(f, "char"),
            MathicType::Void => write!(f, "void"),
            MathicType::Adt { index } => write!(f, "Adt({index})"),
        }
    }
}
