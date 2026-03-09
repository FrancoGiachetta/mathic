use crate::{
    lowering::ir::types::MathicType,
    parser::{ast::expression::ExprStmt, ast::statement::Stmt, Span},
};

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
    pub ty: MathicType,
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
    pub return_ty: AstType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub span: Span,
    pub ty: AstType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstType {
    Str,
    Char,
    Bool,
    // Numeric.
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    // Abstract Data Type.
    Adt(String),
    Void,
}
