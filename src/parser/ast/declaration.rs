use crate::parser::{Span, ast::expression::ExprStmt, ast::statement::Stmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopLevelItem {
    Struct(StructDecl),
    Func(FuncDecl),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DeclStmt {
    Var(VarDecl),
    Struct(StructDecl),
    Func(FuncDecl),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarDecl {
    pub name: String,
    pub expr: ExprStmt,
    pub ty: AstType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<StructField>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructField {
    pub name: String,
    pub ty: AstType,
    pub is_pub: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
    pub span: Span,
    pub return_ty: Option<AstType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub span: Span,
    pub ty: AstType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstType {
    Type(String),
    Array { inner: Box<Self>, length: u32 },
}
