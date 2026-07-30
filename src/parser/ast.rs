use super::ast::declaration::TopLevelItem;

pub mod control_flow;
pub mod declaration;
pub mod expression;
pub mod statement;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MathicModule {
    pub module_name: String,
    pub items: Vec<TopLevelItem>,
}
