use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UintTy {
    U8,
    U16,
    U32,
    U64,
    U128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SintTy {
    I8,
    I16,
    I32,
    I64,
    I128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatTy {
    F32,
    F64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathicType {
    Uint(UintTy),
    Sint(SintTy),
    Float(FloatTy),
    Bool,
    Void,
}

impl MathicType {
    #[inline(always)]
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
            Self::Void => 0,
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
            MathicType::Void => write!(f, "void"),
        }
    }
}
