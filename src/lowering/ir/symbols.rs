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

#[derive(Default, Debug, Clone, Copy)]
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

/// Global Symbol Table.
///
/// Stores symbols declared globally.
#[derive(Debug, Clone)]
pub struct GlobalSymbolTable {
    types: TypeTable,
    pub adts: Vec<Adt>,
    pub functions: HashMap<String, Function>,
}

/// Local Symbol Table.
///
/// Stores symbols declared within the function's context.
#[derive(Debug)]
pub struct LocalSymbolTable<'gbl> {
    types: TypeTable,
    pub locals: Vec<Local>,
    pub local_indexes: HashMap<String, usize>,
    pub functions: HashMap<String, Function>,
    pub adts: Vec<Adt>,
    global_sym_table: &'gbl mut GlobalSymbolTable,
}

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

impl GlobalSymbolTable {
    pub fn add_function(&mut self, func: Function) {
        self.functions.insert(func.name.clone(), func);
    }

    pub fn add_adt(&mut self, adt: Adt) -> usize {
        let index = self.adts.len();

        self.types.insert(MathicType::Adt {
            index,
            is_local: false,
        });
        self.adts.push(adt);

        index
    }

    pub fn get_type(&self, ty: MathicType) -> Option<TypeIndex> {
        self.types.get_index(ty).map(|idx| TypeIndex {
            idx,
            is_local: false,
        })
    }

    pub fn get_or_insert_type(&mut self, ty: MathicType) -> TypeIndex {
        self.get_type(ty).unwrap_or_else(|| TypeIndex {
            idx: self.types.insert(ty),
            is_local: false,
        })
    }
}

impl<'gbl> LocalSymbolTable<'gbl> {
    pub fn new(global: &'gbl mut GlobalSymbolTable) -> Self {
        Self {
            types: Default::default(),
            locals: Vec::new(),
            local_indexes: HashMap::new(),
            functions: HashMap::new(),
            adts: Vec::new(),
            global_sym_table: global,
        }
    }

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

    pub fn get_type(&self, ty: MathicType) -> Option<TypeIndex> {
        if let Some(idx) = self.types.get_index(ty) {
            return Some(TypeIndex {
                idx,
                is_local: true,
            });
        }

        self.global_sym_table.get_type(ty)
    }

    pub fn get_or_insert_type(&mut self, ty: MathicType) -> TypeIndex {
        self.get_type(ty).unwrap_or_else(|| TypeIndex {
            idx: self.types.insert(ty),
            is_local: true,
        })
    }

    pub fn add_function(&mut self, func: Function) {
        self.functions.insert(func.name.clone(), func);
    }

    pub fn add_adt(&mut self, adt: Adt) -> usize {
        let index = self.adts.len();

        self.types.insert(MathicType::Adt {
            index,
            is_local: true,
        });

        self.adts.push(adt);

        index
    }

    pub fn get_adt(&self, idx: usize) -> Option<&Adt> {
        self.adts.get(idx)
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
