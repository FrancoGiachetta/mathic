use std::collections::HashMap;

use crate::{
    diagnostics::LoweringError,
    lowering::ir::{
        adts::Adt,
        function::{Function, Local, LocalKind},
        types::MathicType,
    },
    parser::{
        Span,
        ast::declaration::{FuncDecl, StructDecl},
    },
};

/// Declaration Table
///
/// Use to store function, struct and enum declarations to allow for
/// forward referencing.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct DeclTable {
    functions: HashMap<String, FuncDecl>,
    structs: HashMap<String, StructDecl>,
}

/// Symbol Table.
///
/// Stores locals and function declared within the function's context.
#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    pub locals: Vec<Local>,
    pub local_indexes: HashMap<String, usize>,
    pub functions: HashMap<String, Function>,
    pub adts: Vec<Adt>,
    pub user_def_types: HashMap<String, MathicType>,
}

impl DeclTable {
    pub fn add_func_decl(&mut self, func: FuncDecl) {
        self.functions.insert(func.name.clone(), func);
    }

    pub fn add_struct_decl(&mut self, strct: StructDecl) {
        self.structs.insert(strct.name.clone(), strct);
    }

    pub fn get_function_decl(&self, name: &str) -> Option<&FuncDecl> {
        self.functions.get(name)
    }

    pub fn get_struct_decl(&self, name: &str) -> Option<&StructDecl> {
        self.structs.get(name)
    }
}

impl SymbolTable {
    /// Adds a user-defined local.
    pub fn add_local(
        &mut self,
        debug_name: Option<String>,
        ty: MathicType,
        span: Option<Span>,
        kind: LocalKind,
    ) -> Result<usize, LoweringError> {
        if let Some(name) = &debug_name
            && self.local_indexes.contains_key(name)
        {
            return Err(LoweringError::DuplicateDeclaration {
                name: name.clone(),
                span: span.unwrap(),
            });
        }

        let idx = self.locals.len();

        self.locals.push(Local {
            local_idx: idx,
            kind,
            ty,
            debug_name: debug_name.clone(),
        });

        if let Some(name) = debug_name {
            self.local_indexes.insert(name, idx);
        }

        Ok(idx)
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
                is_local: true,
            },
        );

        self.adts.push(adt);

        index
    }

    pub fn get_adt(&self, idx: usize) -> Option<&Adt> {
        self.adts.get(idx)
    }

    pub fn get_user_def_type(&self, name: &str) -> Option<MathicType> {
        self.user_def_types.get(name).copied()
    }

    pub fn get_local_from_name(&self, name: &str, span: Span) -> Result<Local, LoweringError> {
        let local_idx =
            self.local_indexes
                .get(name)
                .copied()
                .ok_or(LoweringError::UndeclaredVariable {
                    name: name.to_string(),
                    span,
                })?;

        Ok(self.locals[local_idx].clone())
    }
}
