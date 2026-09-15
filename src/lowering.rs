mod ast_lowering;
pub mod ir;
mod utils;

use crate::{
    diagnostics::LoweringError,
    lowering::{
        ast_lowering::{declaration::lower_function, statement::lower_struct},
        ir::{Builder, Ir, IrBuilder},
    },
    parser::ast::{
        IrModule,
        declaration::{Path, TopLevelItem},
    },
};
use tracing::instrument;

/// Lowering entrypoint.
///
/// Given an AST, this function lowers it and returns a MATHIR. In the process,
/// semantic check are perfomed to verify the correctness of the program.
#[instrument(target = "lowering")]
pub fn lower_program(program: &IrModule) -> Result<Ir, LoweringError> {
    let start = std::time::Instant::now();
    tracing::info!("Starting lowering phase");
    let mut ir_builder = IrBuilder::new(program.module_name.clone(), program.modules.clone());

    // Save program's items' declarations. This is for on-demand lowering, allowing
    // to reference function no yet declared. For example, a function call
    // of a not yet declared function.
    for item in program.items.iter() {
        match item {
            TopLevelItem::ExpandBlock(_) => todo!(),
            TopLevelItem::Func(f) => ir_builder.add_function_decl(f.clone(), None, None)?,
            TopLevelItem::Import(imp) => lower_import(&mut ir_builder, imp)?,
            TopLevelItem::Struct(s) => ir_builder.add_struct_decl(s.clone(), None)?,
        }
    }

    for item in program.items.iter() {
        match item {
            TopLevelItem::Func(f) => lower_function(&mut ir_builder, f)?,
            TopLevelItem::Struct(s) => {
                let _ = lower_struct(&mut ir_builder, s)?;
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
    let Path {
        idents,
        group_paths,
        import_all,
        ..
    } = import_path;

    if idents.len() == 1 && group_paths.is_empty() && !*import_all {
        return Ok(());
    }

    if group_paths.is_empty() || *import_all {
        let (items, module_idx, module) = if *import_all {
            let path = import_path.join("::");
            let module_idx = ir_builder.decl_table.get_module_idx(&path).ok_or(
                LoweringError::UnResolvedPath {
                    path,
                    span: import_path.span,
                },
            )?;
            let module = ir_builder
                .get_module(module_idx)
                .cloned()
                .unwrap_or_else(|| panic!("module index {} should be valid", module_idx));

            (module.items.clone(), module_idx, module)
        } else {
            let (item, module_idx) = utils::resolve_path(ir_builder, import_path)?;
            let module = ir_builder
                .get_module(module_idx)
                .cloned()
                .unwrap_or_else(|| panic!("module index {} should be valid", module_idx));

            (vec![item], module_idx, module)
        };

        for item in items {
            match item {
                TopLevelItem::Func(func) => {
                    ir_builder.add_function_decl(func.clone(), None, Some(module_idx))?;
                    utils::add_extern_function(
                        ir_builder,
                        &module.module_name,
                        &func,
                        import_path.span,
                    )?;
                }
                TopLevelItem::Struct(strct) => {
                    ir_builder.add_struct_decl(strct.clone(), Some(module_idx))?
                }
                _ => {}
            }
        }
    } else {
        for path in group_paths {
            lower_import(ir_builder, path)?;
        }
    }

    Ok(())
}
