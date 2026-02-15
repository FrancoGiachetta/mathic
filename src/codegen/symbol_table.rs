use std::{cell::RefCell, collections::HashMap};

use melior::ir::{Value, ValueLike};
use mlir_sys::MlirValue;

#[derive(Clone, Default)]
pub struct SymbolTable {
    symbols: HashMap<String, MlirValue>,
    parent: Box<Option<RefCell<SymbolTable>>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            parent: Box::new(None),
        }
    }

    pub fn with_parent(parent: RefCell<SymbolTable>) -> Self {
        Self {
            symbols: HashMap::new(),
            parent: Box::new(Some(parent)),
        }
    }

    pub fn insert(&mut self, name: String, value: Value) {
        self.symbols.insert(name, value.to_raw());
    }

    pub fn get(&self, name: &str) -> Option<MlirValue> {
        match self.symbols.get(name) {
            None => {
                if let Some(parent) = &*self.parent {
                    parent.borrow().get(name)
                } else {
                    None
                }
            }
            sym => sym.cloned(),
        }
    }

    pub fn update_value(&mut self, name: &str, value: Value) -> Option<MlirValue> {
        if self.symbols.contains_key(name) {
            let new_value = value.to_raw();
            self.symbols.insert(name.to_string(), new_value);
            Some(new_value)
        } else if let Some(parent) = &*self.parent {
            parent.borrow_mut().update_value(name, value)
        } else {
            None
        }
    }
}
