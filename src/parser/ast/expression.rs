use crate::parser::{ast::Span, token::Token};

#[derive(Debug, PartialEq, Eq)]
pub struct ExprStmt {
    pub kind: ExprStmtKind,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ExprStmtKind {
    Primary(PrimaryExpr),
    BinOp {
        lhs: Box<ExprStmt>,
        op: Token,
        rhs: Box<ExprStmt>,
    },
    Call {
        calle: String,
        args: Vec<ExprStmt>,
    },
    Group(Box<ExprStmt>),
    Index {
        name: Token,
        pos: Token,
    },
    Logical {
        lhs: Box<ExprStmt>,
        op: Token,
        rhs: Box<ExprStmt>,
    },
    Unary {
        op: Token,
        rhs: Box<ExprStmt>,
    },
    Assign {
        name: String,
        expr: Box<ExprStmt>,
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
