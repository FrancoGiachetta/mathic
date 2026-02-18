use crate::parser::token::Token;

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ExprStmt {
    Primary(PrimaryExpr),
    BinOp {
        lhs: Box<Self>,
        op: Token,
        rhs: Box<Self>,
    },
    Call {
        calle: String,
        args: Vec<Self>,
    },
    Group(Box<Self>),
    Index {
        name: Token,
        pos: Token,
    },
    Logical {
        lhs: Box<Self>,
        op: Token,
        rhs: Box<Self>,
    },
    Unary {
        op: Token,
        rhs: Box<Self>,
    },
    Assign {
        name: String,
        expr: Box<Self>,
    },
}

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PrimaryExpr {
    Ident(String),
    Num(String),
    Str(String),
    Bool(bool),
}
