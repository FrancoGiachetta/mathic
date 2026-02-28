#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathicType {
    Uint(UintTy),
    Sint(SintTy),
    Float(FloatTy),
    Bool,
    Void,
}

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
            Self::Void => 0,
        }
    }

    pub fn is_signed(&self) -> bool {
        matches!(self, Self::Sint(_))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Sint(_) | Self::Uint(_))
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }
}
