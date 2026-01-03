use crate::parser::grammar::statement::BlockStmt;

#[derive(Debug)]
pub enum DeclStmt {
    StructDeclStmt(StructDecl),
    FuncDeclStmt(FuncDecl),
}

#[derive(Debug)]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<Param>,
}

#[derive(Debug)]
pub struct FuncDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub body: BlockStmt
}

#[derive(Debug)]
pub struct Param {
    pub name: String,
}
