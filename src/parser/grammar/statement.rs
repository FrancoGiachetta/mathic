use crate::parser::grammar::declaration::DeclStmt;

#[derive(Debug)]
pub enum Stmt {
    Decl(DeclStmt),
    Block(BlockStmt),
}

#[derive(Debug)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}
