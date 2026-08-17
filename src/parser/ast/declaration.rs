use crate::parser::{Span, ast::expression::ExprStmt, ast::statement::Stmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopLevelItem {
    Func(FuncDecl),
    Import(Path),
    Struct(StructDecl),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclStmt {
    Var(VarDecl),
    Sym(SymDecl),
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
pub struct SymDecl {
    pub name: String,
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
pub struct Path {
    pub idents: Vec<String>,
    pub group_paths: Vec<Path>,
    pub import_all: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstType {
    Type {
        ty: String,
        inner: Option<Box<AstType>>,
    },
}

impl TopLevelItem {
    #[inline(always)]
    pub fn get_name(&self) -> String {
        match self {
            TopLevelItem::Func(item) => item.name.clone(),
            TopLevelItem::Struct(item) => item.name.clone(),
            TopLevelItem::Import(item) => item.join("_"),
        }
    }
}

impl Path {
    pub fn join(&self, sep: &str) -> String {
        self.idents.join(sep)
    }
}
