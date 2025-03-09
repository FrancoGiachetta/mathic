pub enum MathicType {
    Number(NumberType),
    String,
    Symbol,
    Struct(String),
    Bool,
    Void,
}

pub enum NumberType {
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

impl From<String> for MathicType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "u8" => MathicType::Number(NumberType::U8),
            "u16" => MathicType::Number(NumberType::U16),
            "u32" => MathicType::Number(NumberType::U32),
            "u64" => MathicType::Number(NumberType::U64),
            "i8" => MathicType::Number(NumberType::I8),
            "i16" => MathicType::Number(NumberType::I16),
            "i32" => MathicType::Number(NumberType::I32),
            "64" => MathicType::Number(NumberType::I64),
            "str" => MathicType::String,
            "bool" => MathicType::Bool,
            name => MathicType::Struct(format!("{name}")),
        }
    }
}
