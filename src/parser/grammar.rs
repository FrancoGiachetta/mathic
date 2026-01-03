use super::grammar::declaration::{FuncDecl, StructDecl};

pub mod control_flow;
pub mod declaration;
pub mod expression;
pub mod statement;

#[derive(Debug)]
pub struct Program {
    pub structs: Vec<StructDecl>,
    pub funcs: Vec<FuncDecl>,
}
