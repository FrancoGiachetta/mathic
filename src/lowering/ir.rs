use std::sync::Arc;

use crate::{
    diagnostics::LoweringError,
    lowering::ir::{
        adts::Adt,
        function::{FuncId, Function},
        symbols::{DeclTable, SymbolTableBuilder, TypeIndex},
        types::MathicType,
    },
    parser::{
        Span,
        ast::{
            IrModule,
            declaration::{FuncDecl, StructDecl},
        },
    },
};

pub mod adts;
pub mod basic_block;
pub mod function;
pub mod instruction;
pub mod ir_walk;
pub mod symbols;
pub mod types;
pub mod value;

pub trait Builder {
    fn get_module(&self, idx: usize) -> Option<&Arc<IrModule>>;

    fn get_function_decl(
        &self,
        name: &str,
        method_of: Option<TypeIndex>,
        span: Span,
    ) -> Result<(FuncDecl, Option<usize>), LoweringError>;
    fn add_function_decl(
        &mut self,
        func: FuncDecl,
        method_of: Option<TypeIndex>,
        module_idx: Option<usize>,
    ) -> Result<(), LoweringError>;

    fn get_struct_decl(
        &self,
        name: &str,
        span: Span,
    ) -> Result<(StructDecl, Option<usize>), LoweringError>;
    fn add_struct_decl(
        &mut self,
        strct: StructDecl,
        module_idx: Option<usize>,
    ) -> Result<(), LoweringError>;

    fn add_function(&mut self, func: Function, method_of: Option<TypeIndex>);

    fn get_type(&self, idx: TypeIndex, span: Span) -> Result<MathicType, LoweringError>;

    fn get_self_ty_idx(&self) -> Option<TypeIndex>;
    fn set_self_ty_idx(&mut self, ty: Option<TypeIndex>);

    fn add_adt(&mut self, name: String, adt: Adt) -> usize;
    fn get_adt(&self, adt_type_idx: TypeIndex, span: Span) -> Result<&Adt, LoweringError>;

    fn get_or_insert_type_idx(&mut self, ty: MathicType) -> TypeIndex;
    fn get_user_def_type(&self, name: &str) -> Option<TypeIndex>;

    fn get_mangled_name(&self, module: &str, name: &str) -> String;

    fn get_ir_builder(&mut self) -> &mut IrBuilder;
}

/// Helper struct to build the IR.
#[derive(Debug, Default)]
pub struct IrBuilder {
    pub module_name: String,
    pub decl_table: DeclTable,
    pub sym_table: SymbolTableBuilder,
}

impl IrBuilder {
    pub fn new(module_name: String, modules: Vec<Arc<IrModule>>) -> Self {
        Self {
            module_name,
            sym_table: SymbolTableBuilder::default(),
            decl_table: DeclTable::new(modules),
        }
    }

    pub fn build(self) -> Ir {
        let sym_table = self.sym_table.build();

        Ir {
            types: sym_table.types,
            functions: sym_table.functions,
            adts: sym_table.adts,
        }
    }
}

impl Builder for IrBuilder {
    fn get_module(&self, idx: usize) -> Option<&Arc<IrModule>> {
        self.decl_table.get_module(idx)
    }

    fn get_function_decl(
        &self,
        name: &str,
        method_of: Option<TypeIndex>,
        span: Span,
    ) -> Result<(FuncDecl, Option<usize>), LoweringError> {
        self.decl_table
            .get_function_decl(name, method_of)
            .cloned()
            .ok_or(LoweringError::UndeclaredFunction {
                name: name.to_string(),
                span,
            })
    }

    fn add_function_decl(
        &mut self,
        func: FuncDecl,
        method_of: Option<TypeIndex>,
        module_idx: Option<usize>,
    ) -> Result<(), LoweringError> {
        self.decl_table.add_func_decl(func, method_of, module_idx)
    }

    fn get_struct_decl(
        &self,
        name: &str,
        span: Span,
    ) -> Result<(StructDecl, Option<usize>), LoweringError> {
        self.decl_table
            .get_struct_decl(name)
            .cloned()
            .ok_or(LoweringError::UndeclaredType { span })
    }

    fn add_struct_decl(
        &mut self,
        strct: StructDecl,
        module_idx: Option<usize>,
    ) -> Result<(), LoweringError> {
        self.decl_table.add_struct_decl(strct, module_idx)
    }

    fn add_function(&mut self, func: Function, method_of: Option<TypeIndex>) {
        let func_id = FuncId {
            name: func.name.clone(),
            method_of,
        };
        self.sym_table.functions.insert(func_id, func);
    }

    fn get_type(&self, idx: TypeIndex, span: Span) -> Result<MathicType, LoweringError> {
        self.sym_table
            .get_type(idx.idx)
            .ok_or(LoweringError::UndeclaredType { span })
    }

    fn get_self_ty_idx(&self) -> Option<TypeIndex> {
        self.sym_table.self_ty
    }

    fn set_self_ty_idx(&mut self, ty: Option<TypeIndex>) {
        self.sym_table.self_ty = ty;
    }

    fn add_adt(&mut self, name: String, adt: Adt) -> usize {
        self.sym_table.add_adt(name, adt, false)
    }

    fn get_adt(&self, adt_type_idx: TypeIndex, span: Span) -> Result<&Adt, LoweringError> {
        let adt_ty = self.get_type(adt_type_idx, span)?;

        self.sym_table
            .get_adt(adt_ty)
            .ok_or(LoweringError::UndeclaredType { span })
    }

    fn get_or_insert_type_idx(&mut self, ty: MathicType) -> TypeIndex {
        self.sym_table.get_or_insert_type_idx(ty, false)
    }

    fn get_user_def_type(&self, name: &str) -> Option<TypeIndex> {
        self.sym_table.user_def_types.get(name).copied()
    }

    fn get_mangled_name(&self, module: &str, name: &str) -> String {
        format!("{}::{}", module, name)
    }

    fn get_ir_builder(&mut self) -> &mut IrBuilder {
        self
    }
}

/// Mathic's IR (MATHIR).
#[derive(Debug, Default)]
pub struct Ir {
    pub types: Vec<MathicType>,
    functions: Vec<Function>,
    adts: Vec<Adt>,
}

impl Ir {
    pub fn get_types(&self) -> &[MathicType] {
        &self.types
    }

    pub fn get_type(&self, idx: usize) -> Option<MathicType> {
        self.types.get(idx).copied()
    }

    pub fn get_adt(&self, idx: usize) -> Option<&Adt> {
        self.adts.get(idx)
    }

    pub fn get_functions(&self) -> &[Function] {
        &self.functions
    }

    pub fn get_functions_mut(&mut self) -> &mut [Function] {
        &mut self.functions
    }
}
