use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    // Holds the index to find the associated local
    InMemory(usize),
    // Holds the value as-is
    Const(ConstExpr),
}

/// Constant expressions
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum ConstExpr {
    Numeric(NumericConst),
    Bool(bool),
    Void,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumericConst {
    // Signed Ints
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    // Unsigned Ints
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    // Floats
    F32(f32),
    F64(f64),
}

impl Display for NumericConst {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::I8(n) => write!(f, "{}", n),
            NumericConst::I16(n) => write!(f, "{}", n),
            NumericConst::I32(n) => write!(f, "{}", n),
            NumericConst::I64(n) => write!(f, "{}", n),
            NumericConst::I128(n) => write!(f, "{}", n),
            NumericConst::U8(n) => write!(f, "{}", n),
            NumericConst::U16(n) => write!(f, "{}", n),
            NumericConst::U32(n) => write!(f, "{}", n),
            NumericConst::U64(n) => write!(f, "{}", n),
            NumericConst::U128(n) => write!(f, "{}", n),
            NumericConst::F32(n) => write!(f, "{}", n),
            NumericConst::F64(n) => write!(f, "{}", n),
        }
    }
}

macro_rules! numeric_const_value_from_int {
    ($variant:tt, $int_ty:ty) => {
        impl From<$int_ty> for Value {
            fn from(value: $int_ty) -> Self {
                Self::Const(ConstExpr::Numeric(NumericConst::$variant(value)))
            }
        }
    };
}

numeric_const_value_from_int!(I8, i8);
numeric_const_value_from_int!(I16, i16);
numeric_const_value_from_int!(I32, i32);
numeric_const_value_from_int!(I64, i64);
numeric_const_value_from_int!(I128, i128);
numeric_const_value_from_int!(U8, u8);
numeric_const_value_from_int!(U16, u16);
numeric_const_value_from_int!(U32, u32);
numeric_const_value_from_int!(U64, u64);
numeric_const_value_from_int!(U128, u128);
numeric_const_value_from_int!(F32, f32);
numeric_const_value_from_int!(F64, f64);

impl Display for ConstExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Numeric(n) => write!(f, "{}", n),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Void => write!(f, "void"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InMemory(idx) => write!(f, "%{}", idx),
            Self::Const(c) => write!(f, "{}", c),
        }
    }
}
