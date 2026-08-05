use std::{path::Path, sync::OnceLock};

use mathic::{
    compiler::{CompilerOpts, MathicCompiler},
    executor::{MathicExecutor, jit::MathicJITExecutor},
};

static COMPILER: OnceLock<MathicCompiler> = OnceLock::new();

fn get_compiler() -> &'static MathicCompiler {
    COMPILER.get_or_init(|| MathicCompiler::new().expect("Failed to create the compiler"))
}

pub fn compile_and_execute(path: &Path) -> i64 {
    let compiler = get_compiler();

    let opts = CompilerOpts::default();

    let module = compiler
        .compile_path(path, opts)
        .expect("Failed to compile source");

    let executor = MathicJITExecutor::new(&[module], opts).expect("Failed to create executor");

    executor
        .call_function("program::main")
        .expect("Failed to execute main function")
}
