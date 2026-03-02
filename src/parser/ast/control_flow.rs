use crate::{
    parser::ast::{Span, expression::ExprStmt, statement::BlockStmt},
    types::MathicType,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfStmt {
    pub condition: ExprStmt,
    pub then_block: BlockStmt,
    pub else_block: Option<BlockStmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhileStmt {
    pub condition: ExprStmt,
    pub body: BlockStmt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForStmt {
    pub index_tracker: IndexTracker,
    pub start: ExprStmt,
    pub end: ExprStmt,
    pub body: BlockStmt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexTracker {
    pub name: String,
    pub ty: MathicType,
    pub span: Span,
}
