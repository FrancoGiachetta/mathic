use crate::{
    diagnostics::LoweringError,
    lowering::ir::{
        adts::Adt,
        function::Function,
        symbols::{DeclTable, SymbolTableBuilder, TypeIndex},
        types::MathicType,
    },
    parser::Span,
};

pub mod adts;
pub mod basic_block;
pub mod function;
pub mod instruction;
pub mod ir_walk;
pub mod symbols;
pub mod types;
pub mod value;

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

/// Helper struct to build the IR.
#[derive(Debug, Default)]
pub struct IrBuilder {
    pub decl_table: DeclTable,
    pub sym_table: SymbolTableBuilder,
}

impl IrBuilder {
    pub fn new() -> Self {
        Self {
            decl_table: DeclTable::default(),
            sym_table: SymbolTableBuilder::default(),
        }
    }

    pub fn add_function(&mut self, func: Function) {
        self.sym_table.functions.insert(func.name.clone(), func);
    }

    pub fn get_type(&self, idx: TypeIndex, span: Span) -> Result<MathicType, LoweringError> {
        self.sym_table
            .get_type(idx.idx)
            .ok_or(LoweringError::UndeclaredType { span })
    }

    pub fn add_adt(&mut self, name: String, adt: Adt) -> usize {
        self.sym_table.add_adt(name, adt, false)
    }

    pub fn get_adt(&self, adt_type_idx: TypeIndex, span: Span) -> Result<&Adt, LoweringError> {
        let adt_ty = self.get_type(adt_type_idx, span)?;

        self.sym_table
            .get_adt(adt_ty)
            .ok_or(LoweringError::UndeclaredType { span })
    }

    pub fn get_or_insert_type_idx(&mut self, ty: MathicType) -> TypeIndex {
        self.sym_table.get_or_insert_type(ty, false)
    }

    pub fn get_user_def_type(&self, name: &str) -> Option<TypeIndex> {
        self.sym_table.user_def_types.get(name).copied()
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
