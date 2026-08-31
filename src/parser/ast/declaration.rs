use crate::parser::{Span, ast::expression::ExprStmt, ast::statement::Stmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopLevelItem {
    Func(FuncDecl),
    Import(Path),
    Struct(StructDecl),
    ExpandBlock(ExpandDecl),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpandDecl {
    pub adt_name: Path,
    pub methods: Vec<FuncDecl>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclStmt {
    Var(VarDecl),
    Sym(SymDecl),
    Struct(StructDecl),
    ExpandDecl(ExpandDecl),
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
    pub ty: AstType,
    pub span: Span,
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
    SelfType,
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
            _ => panic!(),
        }
    }
}

impl Path {
    pub fn join(&self, sep: &str) -> String {
        self.idents.join(sep)
    }

    #[allow(dead_code)]
    pub fn concat(&mut self, idents: &[String]) {
        self.idents = [&self.idents, idents].concat()
    }

    pub fn concat_back(&mut self, idents: &[String]) {
        self.idents = [idents, &self.idents].concat()
    }
}
