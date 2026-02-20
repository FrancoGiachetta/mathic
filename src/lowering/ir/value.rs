use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // Holds the index to find the associated local
    InMemory(usize),
    // Holds the value as-is
    Const(ContExpr),
}

/// Constant expressions
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ContExpr {
    Int(String),
    Bool(bool),
    Void,
}

impl Display for ContExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(n) => write!(f, "{}", n),
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
