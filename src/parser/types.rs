pub enum IdentType {
    Number(NumberType),
    String,
    Symbol,
    Struct(String),
    Bool,
    Null,
    Array(Box<Self>)
}

pub enum NumberType {
    Int(IntType),
    Uint(UintType),
    Float(FloaType),
}

pub enum IntType {
    I8,
    I16,
    I32,
    I64,
    I128,
}

pub enum UintType {
    U8,
    U16,
    U32,
    U64,
    U128,
}

pub enum FloaType {
    F32,
    F64,
}
