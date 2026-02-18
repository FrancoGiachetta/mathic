use crate::parser::ast::{
    control_flow::{ForStmt, IfStmt, WhileStmt},
    declaration::DeclStmt,
    expression::ExprStmt,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Stmt {
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
