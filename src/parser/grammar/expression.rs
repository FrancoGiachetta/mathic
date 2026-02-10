use crate::parser::token::Token;

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ExprStmt {
    Primary(PrimaryExpr),
    Assign {
        name: Token,
        value: Box<Self>,
    },
    BinOp {
        lhs: Box<Self>,
        op: Token,
        rhs: Box<Self>,
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
    Call {
        calle: Token,
        args: Vec<Self>,
    },
    Index {
        name: Token,
        pos: Token,
    },
}

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PrimaryExpr {
    Ident(Token),
    Num(String),
    Str(String),
    Bool(bool),
}
