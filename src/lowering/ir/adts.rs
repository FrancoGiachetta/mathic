use crate::{lowering::ir::types::MathicType, parser::Span};

#[derive(Debug, Clone)]
pub enum Adt {
    Struct(StructAdt),
}

#[derive(Debug, Clone)]
pub struct StructAdt {
    pub name: String,
    pub fields: Vec<StructField>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub ty: MathicType,
    pub is_pub: bool,
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
