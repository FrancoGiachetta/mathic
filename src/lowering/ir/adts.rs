use crate::{lowering::ir::types::MathicType, parser::Span};

#[derive(Debug, Clone)]
pub enum Adt {
    Struct(StructAdt),
}

#[derive(Debug, Clone)]
pub struct StructAdt {
    pub name: String,
    pub fields: Vec<StructField>,
    pub _span: Span,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub ty: MathicType,
    pub _is_pub: bool,
}

impl Adt {
    pub fn fields_len(&self) -> usize {
        match self {
            Adt::Struct(s) => s.fields.len(),
        }
    }

    pub fn get_field_names(&self) -> Vec<String> {
        match self {
            Adt::Struct(s) => s.fields.iter().map(|f| f.name.clone()).collect(),
        }
    }

    pub fn get_field_index(&self, name: &str) -> Option<usize> {
        match self {
            Adt::Struct(s) => s.fields.iter().position(|f| f.name == name),
        }
    }

    pub fn get_field_ty(&self, name: &str) -> Option<MathicType> {
        match self {
            Adt::Struct(s) => s
                .fields
                .iter()
                .find(|f| f.name == name)
                .map(|f| f.ty.clone()),
        }
    }

    pub fn get_fields_tys(&self) -> Vec<MathicType> {
        match self {
            Adt::Struct(s) => s.fields.iter().map(|f| f.ty.clone()).collect(),
        }
    }
}
