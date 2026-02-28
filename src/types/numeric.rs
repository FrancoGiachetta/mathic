#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericType {
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
}

impl NumericType {
    pub fn bit_width(&self) -> usize {
        match self {
            Self::I8 | Self::U8 => size_of::<u8>(),
            Self::I16 | Self::U16 => size_of::<u16>(),
            Self::I32 | Self::U32 => size_of::<u32>(),
            Self::I64 | Self::U64 => size_of::<u64>(),
            Self::I128 | Self::U128 => size_of::<u128>(),
            NumericType::F32 => size_of::<f32>(),
            NumericType::F64 => size_of::<f64>(),
        }
    }

    pub fn is_signed(&self) -> bool {
        matches!(
            self,
            Self::I8 | Self::I16 | Self::I32 | Self::I64 | Self::I128 | Self::F32 | Self::F64
        )
    }

    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            Self::I8
                | Self::I16
                | Self::I32
                | Self::I64
                | Self::I128
                | Self::U8
                | Self::U16
                | Self::U32
                | Self::U64
                | Self::U128
        )
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Self::F32 | Self::F64)
    }
}
