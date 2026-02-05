use crate::parser::grammar::statement::Stmt;

#[derive(Debug, PartialEq, Eq)]
pub enum DeclStmt {
    StructDeclStmt(StructDecl),
    FuncDeclStmt(FuncDecl),
}

#[derive(Debug, PartialEq, Eq)]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<Param>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FuncDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Param {
    pub name: String,
}
