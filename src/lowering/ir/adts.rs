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
            Adt::Struct(s) => s.fields.iter().find(|f| f.name == name).map(|f| f.ty),
        }
    }

    pub fn get_fields_tys(&self) -> Vec<MathicType> {
        match self {
            Adt::Struct(s) => s.fields.iter().map(|f| f.ty).collect(),
        }
    }
}

pub fn write_adt_ir<W: std::fmt::Write>(adt: &Adt, f: &mut W, indent: usize) -> std::fmt::Result {
    let indent_str = " ".repeat(indent);

    match adt {
        Adt::Struct(s) => {
            writeln!(f, "{}struct {} {{", indent_str, s.name)?;

            for field in &s.fields {
                writeln!(f, "{}    {}: {},", indent_str, field.name, field.ty)?;
            }

            writeln!(f, "{}}}\n", indent_str)
        }
    }
}
