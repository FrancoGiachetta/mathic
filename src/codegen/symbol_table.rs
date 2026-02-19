use std::collections::HashMap;

use melior::ir::{Value, ValueLike};
use mlir_sys::MlirValue;

/// Symbol table for function name resolution
/// Note: In the new IR system, variables are tracked per-function in the IR
/// This symbol table maintains backward compatibility for the AST-based codegen
#[derive(Clone, Default)]
pub struct SymbolTable {
    /// Function name -> mangled name for calling
    functions: HashMap<String, String>,
    /// Variable name -> MLIR value (for AST-based codegen backward compatibility)
    variables: HashMap<String, MlirValue>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    /// Insert a function into the symbol table
    pub fn insert_func(&mut self, name: String, mangled_name: String) {
        self.functions.insert(name, mangled_name);
    }

    /// Look up a function by name
    pub fn get_func(&self, name: &str) -> Option<&String> {
        self.functions.get(name)
    }

    /// Check if a function exists
    pub fn contains_func(&self, name: &str) -> bool {
        self.functions.contains_key(name)
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
