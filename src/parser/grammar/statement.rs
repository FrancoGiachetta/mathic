use crate::parser::grammar::{declaration::DeclStmt, expression::ExprStmt};

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Stmt {
    Decl(DeclStmt),
    Block(BlockStmt),
    Return(ReturnStmt),
    Expr(ExprStmt),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ReturnStmt {
    pub value: ExprStmt,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}
