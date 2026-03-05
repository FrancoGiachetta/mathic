use std::{collections::HashMap, fmt};

use function::{Function, write_function_ir};

use crate::parser::ast::declaration::FuncDecl;

pub mod basic_block;
pub mod function;
pub mod instruction;
pub mod types;
pub mod value;

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct DeclTable {
    pub functions: HashMap<String, FuncDecl>,
}

#[derive(Debug, Default)]
pub struct Ir {
    pub functions: Vec<Function>,
}

#[derive(Debug, Default)]
pub struct IrBuilder {
    decl_table: DeclTable,
    pub functions: HashMap<String, Function>,
}

impl IrBuilder {
    pub fn new() -> Self {
        Self {
            decl_table: DeclTable::default(),
            functions: HashMap::new(),
        }
    }

    pub fn add_func_decl(&mut self, func: FuncDecl) {
        self.decl_table.functions.insert(func.name.clone(), func);
    }

    pub fn add_function(&mut self, func: Function) {
        self.functions.insert(func.name.clone(), func);
    }

    pub fn get_function_decl(&self, name: &str) -> Option<&FuncDecl> {
        self.decl_table.functions.get(name)
    }

    pub fn build(self) -> Ir {
        Ir {
            functions: self.functions.into_values().collect(),
        }
    }
}

impl fmt::Display for Ir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for func in &self.functions {
            write_function_ir(func, f, 0)?;
        }

        Ok(())
    }
}
