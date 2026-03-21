use std::collections::HashMap;

use crate::lowering::ir::{
    adts::Adt,
    function::Function,
    symbols::{DeclTable, GlobalSymbolTable, TypeIndex},
    types::MathicType,
};

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
    pub sym_table: GlobalSymbolTable,
    pub functions: HashMap<String, Function>,
    pub adts: Vec<Adt>,
}

impl IrBuilder {
    pub fn new() -> Self {
        Self {
            decl_table: DeclTable::default(),
            sym_table: Default::default(),
            functions: HashMap::new(),
            adts: Vec::new(),
        }
    }

    pub fn add_function(&mut self, func: Function) {
        self.functions.insert(func.name.clone(), func);
    }

    pub fn add_adt(&mut self, name: String, adt: Adt) -> usize {
        let index = self.adts.len();

        let adt_type_idx = self.sym_table.get_or_insert_type(MathicType::Adt {
            index,
            is_local: false,
        });

        self.sym_table.add_user_def_type(name, adt_type_idx);
        self.adts.push(adt);

        index
    }

    pub fn get_user_def_type(&self, name: &str) -> Option<TypeIndex> {
        self.sym_table.user_def_types.get(name).copied()
    }

    pub fn build(self) -> Ir {
        Ir {
            functions: self.functions.into_values().collect(),
            adts: self.adts,
        }
    }
}
