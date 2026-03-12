use std::{collections::HashMap, fmt};

use crate::lowering::ir::{
    adts::{Adt, write_adt_ir},
    function::{Function, write_function_ir},
    symbols::DeclTable,
    types::MathicType,
};

pub mod adts;
pub mod basic_block;
pub mod function;
pub mod instruction;
pub mod symbols;
pub mod types;
pub mod value;

/// Mathic's IR (MATHIR).
#[derive(Debug, Default)]
pub struct Ir {
    pub functions: Vec<Function>,
    pub adts: Vec<Adt>,
    pub types: Vec<MathicType>,
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
        self.user_def_types.get(name).copied()
    }

    pub fn build(self) -> Ir {
        Ir {
            functions: self.functions.into_values().collect(),
            adts: self.adts,
            types: Vec::new(),
        }
    }
}

impl fmt::Display for Ir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for adt in &self.adts {
            write_adt_ir(adt, f, 0)?;
        }

        for func in &self.functions {
            write_function_ir(func, f, 0)?;
        }

        Ok(())
    }
}
