mod ast_lowering;
pub mod ir;
mod utils;

use crate::{
    diagnostics::LoweringError,
    lowering::{
        ast_lowering::statement,
        ir::{
            IrBuilder,
            adts::{Adt, StructAdt, StructField},
            function::FunctionBuilder,
            symbols::TypeIndex,
            types::{MathicType, NumericTy, SintTy, UintTy},
        },
    },
    parser::{
        Span,
        ast::{
            MathicModule,
            declaration::{AstType, DeclStmt, FuncDecl, Path, StructDecl, TopLevelItem},
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
pub fn lower_program(program: &MathicModule) -> Result<Ir, LoweringError> {
    let start = std::time::Instant::now();
    tracing::info!("Starting lowering phase");
    let mut ir_builder = IrBuilder::new(program.module_name.clone(), program.modules.clone());

    // Save program's items' declarations. This is for on-demand lowering, allowing
    // to reference function no yet declared. For example, a function call
    // of a not yet declared function.
    for item in program.items.iter() {
        match item {
            TopLevelItem::Func(f) => ir_builder.decl_table.add_func_decl(f.clone(), None)?,
            TopLevelItem::Import(imp) => lower_import(&mut ir_builder, imp)?,
            TopLevelItem::Struct(s) => ir_builder.decl_table.add_struct_decl(s.clone()),
        }
    }

    for item in program.items.iter() {
        match item {
            TopLevelItem::Func(f) => lower_top_level_function(&mut ir_builder, f)?,
            TopLevelItem::Struct(s) => {
                let _ = lower_top_level_struct(&mut ir_builder, s)?;
            }
            _ => {}
        }
    }

    tracing::info!("Lowering complete: {:?}", start.elapsed());

    Ok(ir_builder.build())
}

/// Lowering an import statement.
///
/// It only cares about import that references items (like functions) and adds
/// them to the declaration table of the current ir being built.
fn lower_import(ir_builder: &mut IrBuilder, import_path: &Path) -> Result<(), LoweringError> {
    // Only lower imports which do referen items.
    if import_path.idents.len() == 1 {
        return Ok(());
    }

    let module_path = import_path.idents[..import_path.idents.len() - 1].join("::");
    let item_name = &import_path.idents[import_path.idents.len() - 1];

    let (item, module_idx) =
        utils::find_module_item(ir_builder, &module_path, item_name, import_path.span)?;

    match item {
        TopLevelItem::Func(func) => {
            ir_builder
                .decl_table
                .add_func_decl(func.clone(), Some(module_idx))?;
            utils::add_extern_function(ir_builder, &module_path, &func, import_path.span)?;
        }
        TopLevelItem::Struct(strct) => ir_builder.decl_table.add_struct_decl(strct.clone()),
        _ => {}
    }

    Ok(())
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

    let return_ty = match return_ty {
        Some(ty) => lower_top_level_ast_type(ir_builder, ty, *span)?,
        None => ir_builder.get_or_insert_type_idx(MathicType::Void),
    };

    let mangled_function_name = ir_builder.get_mangled_name(&ir_builder.module_name, name);
    let mut func_builder = FunctionBuilder::new(
        mangled_function_name,
        params,
        return_ty,
        ir_builder,
        *span,
        false,
    )?;

    // Save function's declaration. This for on-demand lowering, allowing
    // to reference function no yet declared. For example, a function call
    // of a not yet declared function.
    for stmt in body.iter() {
        if let StmtKind::Decl(DeclStmt::Func(f)) = &stmt.kind {
            func_builder.decl_table.add_func_decl(f.clone(), None)?;
        }
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

    let idx = ir_builder.add_adt(adt.name.clone(), Adt::Struct(adt));

    Ok(idx)
}

pub fn lower_top_level_ast_type(
    ir_builder: &mut IrBuilder,
    ty: &AstType,
    span: Span,
) -> Result<TypeIndex, LoweringError> {
    Ok(match ty {
        AstType::Type { ty, inner } => {
            match ty.as_str() {
                "isz" => ir_builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Sint(SintTy::Isize))),
                "i8" => ir_builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Sint(SintTy::I8))),
                "i16" => ir_builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Sint(SintTy::I16))),
                "i32" => ir_builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Sint(SintTy::I32))),
                "i64" => ir_builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Sint(SintTy::I64))),
                "i128" => ir_builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Sint(SintTy::I128))),
                "usz" => ir_builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Uint(UintTy::Usize))),
                "u8" => ir_builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Uint(UintTy::U8))),
                "u16" => ir_builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Uint(UintTy::U16))),
                "u32" => ir_builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Uint(UintTy::U32))),
                "u64" => ir_builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Uint(UintTy::U64))),
                "u128" => ir_builder
                    .get_or_insert_type_idx(MathicType::Numeric(NumericTy::Uint(UintTy::U128))),
                "str" => ir_builder.get_or_insert_type_idx(MathicType::Str),
                "char" => ir_builder.get_or_insert_type_idx(MathicType::Char),
                "bool" => ir_builder.get_or_insert_type_idx(MathicType::Bool),
                "expr" => {
                    let Some(inner_ty) = inner else { panic!() };
                    let inner_ty_idx = lower_top_level_ast_type(ir_builder, inner_ty, span)?;
                    let inner_ty = ir_builder.get_type(inner_ty_idx, span)?;

                    match inner_ty {
                        MathicType::Numeric(num_ty) => {
                            ir_builder.get_or_insert_type_idx(MathicType::SymbolicExpr(num_ty))
                        }
                        other => {
                            return Err(LoweringError::MismatchedType {
                                expected: other,
                                found: other,
                                span,
                            });
                        }
                    }
                }
                other => {
                    if let Some(ty) = ir_builder.get_user_def_type(other) {
                        return Ok(ty);
                    }

                    match ir_builder.decl_table.get_struct_decl(other).cloned() {
                        Some(d) => {
                            let adt_index = lower_top_level_struct(ir_builder, &d)?;
                            ir_builder.get_or_insert_type_idx(MathicType::Adt {
                                index: adt_index,
                                is_local: false,
                            })
                        }
                        None => {
                            return Err(LoweringError::UndeclaredType { span });
                        }
                    }
                }
            }
        }
    })
}
