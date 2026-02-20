use super::ast::declaration::{FuncDecl, StructDecl};

pub mod control_flow;
pub mod declaration;
pub mod expression;
pub mod statement;

// Re-export Span type for convenience
pub use crate::parser::lexer::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub structs: Vec<StructDecl>,
    pub funcs: Vec<FuncDecl>,
}
