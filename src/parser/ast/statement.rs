use crate::parser::{
    Span,
    ast::control_flow::{ForStmt, IfStmt, WhileStmt},
    ast::declaration::DeclStmt,
    ast::expression::ExprStmt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StmtKind {
    Decl(DeclStmt),
    Block(BlockStmt),
    If(IfStmt),
    While(WhileStmt),
    For(ForStmt),
    Return(ExprStmt),
    Expr(ExprStmt),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}
