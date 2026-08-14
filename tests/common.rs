#![allow(dead_code)]

use std::{
    env,
    path::{Path, PathBuf},
};

use mathic::{
    compiler::{CompilerOpts, MathicCompiler},
    executor::{MathicExecutor, jit::MathicJITExecutor},
};

/// Resolves a path relative to the crate root (`CARGO_MANIFEST_DIR`).
fn absolute_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_owned()
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"))
            .join(path)
    }
}

/// Resolves the `src/` root of a project directory relative to the crate root.
pub fn project_src_root(project_dir: &Path) -> PathBuf {
    absolute_path(project_dir).join("src")
}

pub fn compile_and_execute(path: &Path) -> i64 {
    let opts = CompilerOpts::default();
    let compiler = MathicCompiler::new().expect("Failed to create the compiler");

    let module = compiler
        .compile_path(path, opts)
        .expect("compilation failed");

    let executor =
        MathicJITExecutor::new(vec![module], opts).expect("Failed to create the executor");

    executor
        .call_function("program::main")
        .expect("execution failed")
}

/// Compiles and executes a whole project (a directory containing a `src/`
/// subdirectory with a `main.mth`). `main.mth`'s `main` function is executed
/// and its return value returned.
pub fn compile_and_execute_project(project_dir: &Path) -> i64 {
    let src_root = absolute_path(project_dir).join("src");

    let opts = CompilerOpts::default();
    let compiler = MathicCompiler::new().expect("Failed to create the compiler");

    let modules = compiler
        .compile_project(&src_root, opts)
        .expect("compilation failed");

    let executor = MathicJITExecutor::new(modules, opts).expect("Failed to create the executor");

    executor
        .call_function("main::main")
        .expect("execution failed")
}
