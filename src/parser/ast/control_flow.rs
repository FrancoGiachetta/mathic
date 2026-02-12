use crate::parser::ast::{expression::ExprStmt, statement::BlockStmt};

#[derive(Debug, PartialEq, Eq)]
pub struct IfStmt {
    pub condition: ExprStmt,
    pub then_block: BlockStmt,
    pub else_block: Option<BlockStmt>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WhileStmt {
    pub condition: ExprStmt,
    pub body: BlockStmt,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ForStmt {
    pub start: ExprStmt,
    pub end: ExprStmt,
    pub body: BlockStmt,
}
