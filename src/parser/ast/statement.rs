use crate::parser::ast::{
    Span,
    control_flow::{ForStmt, IfStmt, WhileStmt},
    declaration::DeclStmt,
    expression::ExprStmt,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StmtKind {
    Decl(DeclStmt),
    Block(BlockStmt),
    If(IfStmt),
    While(WhileStmt),
    For(ForStmt),
    Return(ExprStmt),
    Expr(ExprStmt),
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}
