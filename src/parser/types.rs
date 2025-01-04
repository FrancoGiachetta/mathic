pub enum IdentType {
    Number(NumberType),
    String,
    Symbol,
    Struct(String)
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
