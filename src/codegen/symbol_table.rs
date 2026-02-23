use std::collections::HashMap;

use melior::ir::{Value, ValueLike};
use mlir_sys::MlirValue;

/// Symbol table for function name resolution
/// Note: In the new IR system, variables are tracked per-function in the IR
/// This symbol table maintains backward compatibility for the AST-based codegen
#[derive(Clone, Default)]
pub struct SymbolTable {
    /// Variable name -> MLIR value (for AST-based codegen backward compatibility)
    variables: HashMap<String, MlirValue>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    // Backward compatibility methods for AST-based codegen

    /// Insert a variable (for AST-based codegen)
    pub fn insert(&mut self, name: String, value: Value) {
        self.variables.insert(name, value.to_raw());
    }

    /// Look up a variable (for AST-based codegen)
    pub fn get(&self, name: &str) -> Option<MlirValue> {
        self.variables.get(name).copied()
    }
}
