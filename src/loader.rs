use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    MathicResult,
    diagnostics::{CompilationError, DiagnosticsManager, LoweringError},
    parser::{
        MathicParser,
        ast::{
            IrModule,
            declaration::{Path as MathicPath, TopLevelItem},
        },
    },
};

/// Loads a module and all the ones it imports.
pub struct ModuleLoader<'a> {
    diagnostics: &'a DiagnosticsManager,
    parsed: HashMap<PathBuf, Arc<IrModule>>,
}

impl<'a> ModuleLoader<'a> {
    pub fn new(diagnostics: &'a DiagnosticsManager) -> Self {
        Self {
            diagnostics,
            parsed: HashMap::new(),
        }
    }

    /// Parses the module at `path` and, recursively, every module it imports.
    ///
    /// Returns a mapping of every parsed module path to its AST.
    pub fn load(
        &mut self,
        src_root: &Path,
        path: PathBuf,
    ) -> MathicResult<HashMap<PathBuf, Arc<IrModule>>> {
        let abs_path = src_root.join(&path);
        let base_path = abs_path.parent().unwrap();

        if let Some(module) = self.parsed.get(&abs_path) {
            return Ok(HashMap::from([(abs_path, module.clone())]));
        }

        let source = fs::read_to_string(&abs_path)?;

        let mut compilation_unit = HashMap::new();

        let mut program = {
            let parser = MathicParser::new(&source);
            let module_name = path
                .with_extension("")
                .to_string_lossy()
                .replace("/", "::")
                .to_string();

            match parser.parse(module_name) {
                Err(e) => {
                    self.diagnostics
                        .report(abs_path.clone(), CompilationError::Parse(e))?;
                    return Ok(compilation_unit);
                }
                Ok(module) => module,
            }
        };

        for item in &program.items {
            if let TopLevelItem::Import(import_path) = item {
                if import_path.group_paths.is_empty() {
                    self.resolve_import(
                        &abs_path,
                        src_root,
                        base_path,
                        import_path,
                        &mut compilation_unit,
                    )?;
                } else {
                    for member in &import_path.group_paths {
                        self.resolve_import(
                            &abs_path,
                            src_root,
                            base_path,
                            member,
                            &mut compilation_unit,
                        )?;
                    }
                }
            }
        }

        program.modules = compilation_unit.values().cloned().collect();

        let module = Arc::new(program);
        self.parsed.insert(abs_path.clone(), module.clone());

        compilation_unit.insert(abs_path, module);

        Ok(compilation_unit)
    }

    /// Resolves an import (a module or an item import) to a module path and
    /// parses it.
    fn resolve_import(
        &mut self,
        abs_path: &Path,
        src_root: &Path,
        base_path: &Path,
        import_path: &MathicPath,
        compilation_unit: &mut HashMap<PathBuf, Arc<IrModule>>,
    ) -> MathicResult<()> {
        let new_base_path = match get_module_path(src_root, base_path, import_path) {
            Ok(res) => res,
            Err(err) => {
                self.diagnostics.report(abs_path.to_path_buf(), err)?;
                return Ok(());
            }
        };

        if !new_base_path.exists() {
            let path = new_base_path
                .strip_prefix(src_root)
                .unwrap()
                .to_string_lossy()
                .replace("/", "::");

            self.diagnostics.report(
                abs_path.to_path_buf(),
                CompilationError::Lowering(LoweringError::UnResolvedPath {
                    path,
                    span: import_path.span,
                }),
            )?;
            return Ok(());
        }

        compilation_unit.extend(self.load(
            src_root,
            new_base_path.strip_prefix(src_root).unwrap().to_owned(),
        )?);

        Ok(())
    }
}

/// Gets the path to a module based on an import path relative to a base path.
///
/// * if the full path of the import is a file, or not because is a group
///   import, we take the path as is.
///
/// * if the full is not a file, it could only be that the import references a
///   top level item, so we try to take the path formed by all them idents but
///   the last one (top level item).
fn get_module_path(
    src_root: &Path,
    base_path: &Path,
    path: &MathicPath,
) -> Result<PathBuf, CompilationError> {
    let MathicPath {
        idents,
        group_paths,
        span,
        import_all: _,
    } = path;

    let full_path = base_path.join(idents.join("/")).with_added_extension("mth");

    if full_path.is_file() || !group_paths.is_empty() {
        Ok(full_path)
    } else {
        let Some(idents_without_last) = idents.get(..idents.len() - 1) else {
            let path = full_path
                .strip_prefix(src_root)
                .unwrap()
                .to_string_lossy()
                .replace("/", "::");

            return Err(CompilationError::Lowering(LoweringError::UnResolvedPath {
                path,
                span: *span,
            }));
        };

        Ok(base_path
            .join(idents_without_last.join("/"))
            .with_added_extension("mth"))
    }
}
