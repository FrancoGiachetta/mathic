pub mod ast_lowering;
pub mod ir;

use crate::{
    diagnostics::LoweringError,
    lowering::{
        ast_lowering::statement,
        ir::{
            IrBuilder,
            adts::{Adt, StructAdt, StructField},
            function::FunctionBuilder,
            types::{MathicType, SintTy, UintTy, lower_inner_ast_type},
        },
    },
    parser::{
        Span,
        ast::{
            Program,
            declaration::{AstType, DeclStmt, FuncDecl, StructDecl, TopLevelItem},
            statement::StmtKind,
        },
    },
};
use ir::Ir;
use tracing::instrument;

/// Lowering entrypoint.
///
/// Given an AST, this function lowers it and returns a MATHIR. In the process,
/// semantic check are perfomed to verify the correctness of the program.
#[instrument(target = "lowering")]
pub fn lower_program(program: &Program) -> Result<Ir, LoweringError> {
    let start = std::time::Instant::now();
    tracing::info!("Starting lowering phase");
    let mut ir_builder = IrBuilder::new();

    // Save program's items' declarations. This is for on-demand lowering, allowing
    // to reference function no yet declared. For example, a function call
    // of a not yet declared function.
    for item in program.items.iter() {
        match item {
            TopLevelItem::Func(f) => ir_builder.decl_table.add_func_decl(f.clone()),
            TopLevelItem::Struct(s) => ir_builder.decl_table.add_struct_decl(s.clone()),
        }
    }

    for item in program.items.iter() {
        match item {
            TopLevelItem::Func(f) => lower_top_level_function(&mut ir_builder, f)?,
            TopLevelItem::Struct(s) => {
                let _ = lower_top_level_struct(&mut ir_builder, s)?;
            }
        }
    }

    tracing::info!("Lowering complete: {:?}", start.elapsed());
    Ok(ir_builder.build())
}

/// Lowers global functions.
#[instrument(target = "lowering", skip(ir_builder))]
fn lower_top_level_function(
    ir_builder: &mut IrBuilder,
    func_decl: &FuncDecl,
) -> Result<(), LoweringError> {
    tracing::debug!("Lowering function: {}", func_decl.name);
    let FuncDecl {
        name,
        params,
        body,
        span,
        return_ty,
    } = func_decl;

    let mut temporary_func_builder =
        FunctionBuilder::new(name.clone(), params, MathicType::Void, ir_builder, *span)?;

    let return_ty = match return_ty {
        Some(ty) => lower_inner_ast_type(&mut temporary_func_builder, ty, *span)?,
        None => MathicType::Void,
    };

    let mut func_builder =
        FunctionBuilder::new(name.clone(), params, return_ty, ir_builder, *span)?;

    // Save function's declaration. This for on-demand lowering, allowing
    // to reference function no yet declared. For example, a function call
    // of a not yet declared function.
    for stmt in body.iter() {
        if let StmtKind::Decl(DeclStmt::Func(f)) = &stmt.kind {
            func_builder.decl_table.add_func_decl(f.clone());
        }

        // FUTURE: do the same for structs, enums, etc
    }

    for stmt in body {
        statement::lower_stmt(&mut func_builder, stmt)?;
    }

    let func = func_builder.build();

    ir_builder.add_function(func);

    Ok(())
}

fn lower_top_level_struct(
    ir_builder: &mut IrBuilder,
    struct_decl: &StructDecl,
) -> Result<usize, LoweringError> {
    let StructDecl { name, fields, span } = struct_decl;

    let mut adt = StructAdt {
        name: name.clone(),
        fields: Vec::new(),
        _span: *span,
    };

    for field in fields {
        adt.fields.push(StructField {
            name: field.name.clone(),
            ty: lower_top_level_ast_type(ir_builder, &field.ty, field.span)?,
            _is_pub: field.is_pub,
        });
    }

    let idx = ir_builder.add_adt(name.clone(), Adt::Struct(adt));

    Ok(idx)
}

pub fn lower_top_level_ast_type(
    ir_builder: &mut IrBuilder,
    ty: &AstType,
    span: Span,
) -> Result<MathicType, LoweringError> {
    Ok(match ty {
        AstType::Type(name) => match name.as_str() {
            "i8" => MathicType::Sint(SintTy::I8),
            "i16" => MathicType::Sint(SintTy::I16),
            "i32" => MathicType::Sint(SintTy::I32),
            "i64" => MathicType::Sint(SintTy::I64),
            "i128" => MathicType::Sint(SintTy::I128),
            "u8" => MathicType::Uint(UintTy::U8),
            "u16" => MathicType::Uint(UintTy::U16),
            "u32" => MathicType::Uint(UintTy::U32),
            "u64" => MathicType::Uint(UintTy::U64),
            "u128" => MathicType::Uint(UintTy::U128),
            "str" => MathicType::Str,
            "char" => MathicType::Char,
            "bool" => MathicType::Bool,
            other => {
                if let Some(ty) = ir_builder.get_user_def_type(other) {
                    return Ok(ty);
                }

                match ir_builder.decl_table.get_struct_decl(other).cloned() {
                    Some(d) => MathicType::Adt {
                        index: lower_top_level_struct(ir_builder, &d)?,
                        is_local: false,
                    },
                    None => {
                        return Err(LoweringError::UndeclaredType { span });
                    }
                }
            }
        },
    })
}
