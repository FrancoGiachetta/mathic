use crate::parser::{ast::Span, token::Token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprStmt {
    pub kind: ExprStmtKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ExprStmtKind {
    Primary(PrimaryExpr),
    Binary {
        lhs: Box<ExprStmt>,
        op: BinaryOp,
        rhs: Box<ExprStmt>,
    },
    Call {
        callee: String,
        args: Vec<ExprStmt>,
    },
    Group(Box<ExprStmt>),
    Index {
        name: Token,
        pos: Token,
    },
    Logical {
        lhs: Box<ExprStmt>,
        op: LogicalOp,
        rhs: Box<ExprStmt>,
    },
    Unary {
        op: UnaryOp,
        rhs: Box<ExprStmt>,
    },
    Assign {
        name: String,
        expr: Box<ExprStmt>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Arithmetic(ArithOp),
    Compare(CmpOp),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PrimaryExpr {
    Ident(String),
    Num(String),
    Str(String),
    Bool(bool),
}
