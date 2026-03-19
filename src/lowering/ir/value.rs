#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // Holds the index to find the associated local
    InMemory {
        local_idx: usize,
        modifier: Vec<ValueModifier>,
    },

    // Holds the value as-is
    Const(ConstExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueModifier {
    Field(usize),
}

/// Constant expressions
#[derive(Debug, Clone, PartialEq)]
pub enum ConstExpr {
    // Fixed size strings
    Str(String),
    Char(char),
    Numeric(NumericConst),
    Bool(bool),
    Void,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumericConst {
    // Signed Ints
    Isize(isize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    // Unsigned Ints
    Usize(usize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    // Floats
    F32(f32),
    F64(f64),
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
