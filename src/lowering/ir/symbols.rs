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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeIndex {
    pub idx: usize,
    pub is_local: bool,
}

#[derive(Default, Debug, Clone)]
pub struct TypeTable {
    types: Vec<MathicType>,
    cache: HashMap<MathicType, usize>, // private
}

impl TypeTable {
    pub fn get(&self, idx: usize) -> Option<MathicType> {
        self.types.get(idx).copied()
    }

    pub fn get_index(&self, ty: MathicType) -> Option<usize> {
        self.cache.get(&ty).copied()
    }

    pub fn insert(&mut self, ty: MathicType) -> usize {
        let idx = self.types.len();

        self.types.push(ty);
        self.cache.insert(ty, idx);

        idx
    }
}

/// Local Symbol Table.
///
/// Stores symbols declared within the function's context.
#[derive(Clone, Debug, Default)]
pub struct SymbolTable {
    pub types: Vec<MathicType>,
    pub locals: Vec<Local>,
    pub functions: Vec<Function>,
    pub adts: Vec<Adt>,
}

#[derive(Clone, Debug, Default)]
pub struct SymbolTableBuilder {
    types: TypeTable,
    pub locals: Vec<Local>,
    pub local_indexes: HashMap<String, usize>,
    pub functions: HashMap<String, Function>,
    pub user_def_types: HashMap<String, TypeIndex>,
    pub adts: Vec<Adt>,
}

impl SymbolTableBuilder {
    pub fn build(self) -> SymbolTable {
        SymbolTable {
            types: self.types.types,
            locals: self.locals,
            functions: self.functions.into_values().collect(),
            adts: self.adts,
        }
    }

    pub fn get_type_index(&self, ty: MathicType, is_local: bool) -> Option<TypeIndex> {
        self.types
            .get_index(ty)
            .map(|idx| TypeIndex { idx, is_local })
    }

    pub fn get_type(&self, idx: usize) -> Option<MathicType> {
        self.types.get(idx)
    }

    pub fn get_or_insert_type(&mut self, ty: MathicType, is_local: bool) -> TypeIndex {
        self.get_type_index(ty, is_local)
            .unwrap_or_else(|| TypeIndex {
                idx: self.types.insert(ty),
                is_local,
            })
    }

    pub fn add_function(&mut self, func: Function) {
        self.functions.insert(func.name.clone(), func);
    }

    pub fn get_user_def_type(&self, name: &str) -> Option<TypeIndex> {
        self.user_def_types.get(name).copied()
    }

    pub fn add_adt(&mut self, name: String, adt: Adt, is_local: bool) -> usize {
        let index = self.adts.len();

        let adt_type_idx = self.types.insert(MathicType::Adt { index, is_local });

        self.user_def_types.insert(
            name,
            TypeIndex {
                idx: adt_type_idx,
                is_local,
            },
        );
        self.adts.push(adt);

        index
    }

    pub fn get_adt(&self, adt_ty: MathicType) -> Option<&Adt> {
        let MathicType::Adt { index, .. } = adt_ty else {
            panic!("internal error: expected ADT type in get_adt")
        };

        self.adts.get(index)
    }

    pub fn add_local(
        &mut self,
        debug_name: Option<String>,
        ty: TypeIndex,
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
