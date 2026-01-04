use crate::parser::grammar::{declaration::DeclStmt, expression::ExprStmt};

#[derive(Debug, PartialEq, Eq)]
pub enum Stmt {
    Decl(DeclStmt),
    Block(BlockStmt),
    Expr(ExprStmt),
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}
