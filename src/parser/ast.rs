use std::sync::Arc;

use super::ast::declaration::TopLevelItem;

pub mod control_flow;
pub mod declaration;
pub mod expression;
pub mod statement;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrModule {
    pub module_name: String,
    pub modules: Vec<Arc<Self>>,
    pub items: Vec<TopLevelItem>,
}
