use std::collections::HashMap;

use melior::ir::Value;

#[derive(Clone, Default)]
pub struct SymbolTable<'ctx, 'this> {
    symbols: HashMap<String, Value<'ctx, 'this>>,
    parent: Box<Option<SymbolTable<'ctx, 'this>>>,
}

impl<'ctx, 'this> SymbolTable<'ctx, 'this> {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            parent: Box::new(None),
        }
    }

    pub fn with_parent(parent: SymbolTable<'ctx, 'this>) -> Self {
        Self {
            symbols: HashMap::new(),
            parent: Box::new(Some(parent)),
        }
    }

    pub fn insert(&mut self, name: String, value: Value<'ctx, 'this>) {
        self.symbols.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value<'ctx, 'this>> {
        match self.symbols.get(name) {
            None => {
                if let Some(parent) = &*self.parent {
                    parent.get(name)
                } else {
                    None
                }
            }
            sym => sym.cloned(),
        }
    }
}
