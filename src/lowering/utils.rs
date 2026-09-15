use crate::{
    diagnostics::LoweringError,
    lowering::{
        ast_lowering::{lower_ast_type, statement::lower_struct},
        ir::{
            Builder, IrBuilder, function::FunctionBuilder, symbols::TypeIndex, types::MathicType,
        },
    },
    parser::{
        Span,
        ast::declaration::{FuncDecl, Path, StructDecl, TopLevelItem},
    },
};

/// Adds an external function declaration to the IR.
pub fn add_extern_function(
    ir_builder: &mut IrBuilder,
    module_path: &str,
    func: &FuncDecl,
    span: Span,
) -> Result<(), LoweringError> {
    let return_ty = match &func.return_ty {
        Some(ty) => lower_ast_type(ir_builder, ty, span)?,
        None => ir_builder.get_or_insert_type_idx(MathicType::Void),
    };
    let mangled_function_name = ir_builder.get_mangled_name(module_path, &func.name);
    let extern_func = FunctionBuilder::new(
        mangled_function_name,
        &func.params,
        return_ty,
        ir_builder,
        span,
        true,
    )?
    .build();

    ir_builder.add_function(extern_func);

    Ok(())
}

/// Resolves a [`Path`].
///
/// Returns the [`TopLevelItem`] together with the index of the module the item
/// lives in. Otherwise returns a [`LoweringError`].
pub fn resolve_path(
    ir_builder: &IrBuilder,
    path: &Path,
) -> Result<(TopLevelItem, usize), LoweringError> {
    let module_path = path
        .idents
        .get(..path.idents.len() - 1)
        .ok_or(LoweringError::UnResolvedPath {
            path: path.idents.join("::"),
            span: path.span,
        })?
        .join("::");
    let item_name = &path.idents[path.idents.len() - 1];

    find_module_item(ir_builder, &module_path, item_name, path.span)
}

/// Resolves an imported function reference (e.g. `util::add`) to the
/// referenced function's declaration and module index.
///
/// This also declares the function as external in the IR if not already.
pub fn resolve_external_func(
    ir_builder: &mut IrBuilder,
    path: &Path,
) -> Result<(FuncDecl, usize), LoweringError> {
    let module_path = path.idents[..path.idents.len() - 1].join("::");
    let item_name = &path.idents[path.idents.len() - 1];

    let (item, module_idx) = find_module_item(ir_builder, &module_path, item_name, path.span)?;

    match item {
        TopLevelItem::Func(func) => {
            let mangled_name = ir_builder.get_mangled_name(&module_path, &func.name);

            // The function may already be declared by a path call (mangled
            // name) or by an import (non-mangled name).
            let declared_by_path = ir_builder.sym_table.functions.contains_key(&mangled_name);
            let declared_by_import = ir_builder
                .decl_table
                .get_function_decl(&func.name)
                .is_some_and(|(_, module)| *module == Some(module_idx));

            if !(declared_by_path || declared_by_import) {
                add_extern_function(ir_builder, &module_path, &func, path.span)?;
            }

            Ok((func, module_idx))
        }
        _ => Err(LoweringError::UnResolvedPath {
            path: path.join("::"),
            span: path.span,
        }),
    }
}

/// Resolves a [`Path`] to an external struct, registering the struct's ADT type
/// under its module-qualified name if it is not already present.
///
/// Returns the struct's [`TypeIndex`] together with the index of the module the
/// struct lives in.
pub fn resolve_external_struct(
    ir_builder: &mut IrBuilder,
    path: &Path,
) -> Result<(TypeIndex, usize), LoweringError> {
    let module_path = path.idents[..path.idents.len() - 1].join("::");
    let item_name = &path.idents[path.idents.len() - 1];

    let (item, module_idx) = find_module_item(ir_builder, &module_path, item_name, path.span)?;

    match item {
        TopLevelItem::Struct(strct) => {
            let adt_ty =
                get_or_insert_struct_type(ir_builder, &strct, Some(module_idx), path.span)?;
            Ok((adt_ty, module_idx))
        }
        _ => Err(LoweringError::UnResolvedPath {
            path: path.join("::"),
            span: path.span,
        }),
    }
}

/// Resolves a struct type.
///
/// Order of resolution:
/// 1. Types already registered in the function's symbol table, i.e. function
///    local ADTs and top-level structs previously seen in the same module.
/// 2. External structs keyed by their mangled name (`module::Struct`).
/// 3. Otherwise the struct is registered on demand.
pub fn resolve_struct_type(
    builder: &mut impl Builder,
    name: &str,
    span: Span,
) -> Result<TypeIndex, LoweringError> {
    if let Some(ty) = builder.get_user_def_type(name) {
        return Ok(ty);
    }

    let Some((strct, module_idx)) = builder.get_struct_decl(name).cloned() else {
        return Err(LoweringError::UndeclaredType { span });
    };

    let key = match module_idx {
        None => strct.name.clone(),
        Some(idx) => {
            let module = builder
                .get_module(idx)
                .unwrap_or_else(|| panic!("module index {} should be valid", idx));

            builder.get_mangled_name(&module.module_name, &strct.name)
        }
    };

    // The struct may already be resolved (e.g. by a prior struct init or a
    // type annotation), keyed by its mangled name.
    if let Some(ty) = builder.get_user_def_type(&key) {
        return Ok(ty);
    }

    get_or_insert_struct_type(builder, &strct, module_idx, span)
}

/// Registers a [`StructDecl`]'s ADT type, deduplicating by name.
///
/// A struct local to the current module is registered under its plain name
/// (e.g. `Point`), while a struct from another module is registered under its
/// module-qualified name (e.g. `util::Point`), mirroring how function symbols
/// are mangled in the IR. Returns the struct's [`TypeIndex`].
pub fn get_or_insert_struct_type(
    builder: &mut dyn Builder,
    strct_decl: &StructDecl,
    module_idx: Option<usize>,
    span: Span,
) -> Result<TypeIndex, LoweringError> {
    let key = match module_idx {
        None => strct_decl.name.clone(),
        Some(idx) => {
            let module = builder
                .get_module(idx)
                .unwrap_or_else(|| panic!("module index {} should be valid", idx));

            builder.get_mangled_name(&module.module_name, &strct_decl.name)
        }
    };

    if let Some(ty) = builder.get_user_def_type(&key) {
        return Ok(ty);
    }

    let mut strct = strct_decl.clone();

    strct.name = key.clone();
    lower_struct(builder, &strct)?;

    builder
        .get_user_def_type(&key)
        .ok_or(LoweringError::UndeclaredType { span })
}

/// Finds a top level item within a module.
fn find_module_item(
    ir_builder: &IrBuilder,
    module_path: &str,
    item_name: &str,
    span: Span,
) -> Result<(TopLevelItem, usize), LoweringError> {
    let Some(module_idx) = ir_builder.decl_table.get_module_idx(module_path) else {
        return Err(LoweringError::UnResolvedPath {
            path: module_path.to_string(),
            span,
        });
    };

    let module = ir_builder
        .decl_table
        .get_module(module_idx)
        .unwrap_or_else(|| panic!("module index {} should be valid", module_idx));

    let item = module
        .items
        .iter()
        .find(|i| i.get_name() == item_name)
        .cloned()
        .ok_or(LoweringError::UnResolvedPath {
            path: module_path.to_string(),
            span,
        })?;

    Ok((item, module_idx))
}
