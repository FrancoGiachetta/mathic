use crate::types::numeric::NumericType;

pub mod numeric;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathicType {
    Number(NumericType),
    Bool,
    Void,
}
