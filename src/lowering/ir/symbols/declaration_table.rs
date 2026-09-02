use std::{collections::HashMap, sync::Arc};

use crate::{
    diagnostics::LoweringError,
    lowering::ir::symbols::SymbolTableBuilder,
    parser::ast::{
        IrModule,
        declaration::{FuncDecl, StructDecl},
    },
};

/// Declaration Table
///
/// Use to store function, struct and enum declarations to allow for
/// forward referencing.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct DeclTable {
    pub name_to_module: HashMap<String, usize>,
    pub modules: Vec<Arc<IrModule>>,
    functions: HashMap<String, (FuncDecl, Option<usize>)>,
    structs: HashMap<String, (StructDecl, Option<usize>)>,
}

impl DeclTable {
    pub fn new(modules: Vec<Arc<IrModule>>) -> Self {
        let name_to_module = modules
            .iter()
            .enumerate()
            .map(|(idx, m)| (m.module_name.clone(), idx))
            .collect();

        Self {
            name_to_module,
            modules,
            ..Default::default()
        }
    }

    pub fn add_func_decl(
        &mut self,
        func: FuncDecl,
        module_idx: Option<usize>,
    ) -> Result<(), LoweringError> {
        let name = func.name.clone();

        if self.functions.contains_key(&name) {
            return Err(LoweringError::DuplicateDeclaration {
                name,
                span: func.span,
            });
        }

        self.functions.insert(name, (func, module_idx));

        Ok(())
    }

    pub fn add_struct_decl(
        &mut self,
        strct: StructDecl,
        module_idx: Option<usize>,
    ) -> Result<(), LoweringError> {
        let name = strct.name.clone();

        if self.structs.contains_key(&name) {
            return Err(LoweringError::DuplicateDeclaration {
                name,
                span: strct.span,
            });
        }
        self.structs.insert(strct.name.clone(), (strct, module_idx));

        Ok(())
    }
}

impl SymbolTableBuilder {
    pub fn add_func_decl(
        &mut self,
        func: FuncDecl,
        module_idx: Option<usize>,
    ) -> Result<(), LoweringError> {
        self.decl_table.add_func_decl(func, module_idx)
    }

    pub fn add_struct_decl(
        &mut self,
        strct: StructDecl,
        module_idx: Option<usize>,
    ) -> Result<(), LoweringError> {
        self.decl_table.add_struct_decl(strct, module_idx)
    }

    pub fn get_function_decl(&self, name: &str) -> Option<&(FuncDecl, Option<usize>)> {
        self.decl_table.functions.get(name)
    }

    pub fn get_struct_decl(&self, name: &str) -> Option<&(StructDecl, Option<usize>)> {
        self.decl_table.structs.get(name)
    }

    pub fn get_module_idx(&self, module_name: &str) -> Option<usize> {
        self.decl_table.name_to_module.get(module_name).copied()
    }

    pub fn get_module(&self, module_idx: usize) -> Arc<IrModule> {
        self.decl_table
            .modules
            .get(module_idx)
            .cloned()
            .unwrap_or_else(|| panic!("module index {} should be valid", module_idx))
    }
}
