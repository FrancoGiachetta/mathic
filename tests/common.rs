#![allow(dead_code)]

use std::{
    env,
    path::{Path, PathBuf},
    sync::OnceLock,
};

use mathic::{
    compiler::{CompilerOpts, MathicCompiler},
    executor::{MathicExecutor, jit::MathicJITExecutor},
};

static COMPILER: OnceLock<MathicCompiler> = OnceLock::new();

fn get_compiler() -> &'static MathicCompiler {
    COMPILER.get_or_init(|| MathicCompiler::new().expect("Failed to create the compiler"))
}

/// Resolves a path relative to the crate root (`CARGO_MANIFEST_DIR`).
fn absolute_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_owned()
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"))
            .join(path)
    }
}

pub fn compile_and_execute(path: &Path) -> i64 {
    let opts = CompilerOpts::default();

    get_compiler()
        .compile_path(path, opts)
        .and_then(|module| {
            let executor = MathicJITExecutor::new(&[module], opts)?;
            Ok(executor.call_function("program::main")?)
        })
        .unwrap()
}

/// Compiles and executes a whole project (a directory containing a `src/`
/// subdirectory with a `main.mth`). `main.mth`'s `main` function is executed
/// and its return value returned.
pub fn compile_and_execute_project(project_dir: &Path) -> i64 {
    let src_root = absolute_path(project_dir).join("src");

    let opts = CompilerOpts::default();

    get_compiler()
        .compile_project(&src_root, opts)
        .and_then(|modules| {
            let executor = MathicJITExecutor::new(&modules, opts)?;
            Ok(executor.call_function("main::main")?)
        })
        .unwrap()
}
