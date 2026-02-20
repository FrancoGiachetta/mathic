use crate::parser::ast::{Span, expression::ExprStmt, statement::Stmt};

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DeclStmt {
    Var(VarDecl),
    Struct(StructDecl),
    Func(FuncDecl),
}

#[derive(Debug, PartialEq, Eq)]
pub struct VarDecl {
    pub name: String,
    pub expr: ExprStmt,
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
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub span: Span,
}
