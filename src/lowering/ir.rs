use std::collections::HashMap;

use crate::lowering::ir::{adts::Adt, function::Function, symbols::DeclTable, types::MathicType};

pub mod adts;
pub mod basic_block;
pub mod function;
pub mod instruction;
pub mod ir_walk;
pub mod symbols;
pub mod types;
pub mod value;

/// Mathic's IR (MATHIR).
#[derive(Debug, Default)]
pub struct Ir {
    pub functions: Vec<Function>,
    pub adts: Vec<Adt>,
}

/// Helper struct to build the IR.
#[derive(Debug, Default)]
pub struct IrBuilder {
    pub decl_table: DeclTable,
    pub functions: HashMap<String, Function>,
    pub adts: Vec<Adt>,
    pub user_def_types: HashMap<String, MathicType>,
}

impl IrBuilder {
    pub fn new() -> Self {
        Self {
            decl_table: DeclTable::default(),
            functions: HashMap::new(),
            adts: Vec::new(),
            user_def_types: HashMap::new(),
        }
    }

    pub fn add_function(&mut self, func: Function) {
        self.functions.insert(func.name.clone(), func);
    }

    pub fn add_adt(&mut self, name: String, adt: Adt) -> usize {
        let index = self.adts.len();

        self.user_def_types.insert(
            name,
            MathicType::Adt {
                index,
                is_local: false,
            },
        );
        self.adts.push(adt);

        index
    }

    pub fn get_user_def_type(&self, name: &str) -> Option<MathicType> {
        self.user_def_types.get(name).cloned()
    }

    pub fn build(self) -> Ir {
        Ir {
            functions: self.functions.into_values().collect(),
            adts: self.adts,
        }
    }
}
