use std::{path::Path, sync::OnceLock};

use mathic::{
    compiler::{MathicCompiler, OptLvl},
    executor::MathicExecutor,
};

static COMPILER: OnceLock<MathicCompiler> = OnceLock::new();

fn get_compiler() -> &'static MathicCompiler {
    &COMPILER.get_or_init(|| MathicCompiler::new().expect("Failed to create the compiler"))
}

pub fn compile_and_execute(path: &Path) -> i64 {
    let compiler = get_compiler();

    let module = compiler
        .compile_path(path, OptLvl::O2)
        .expect("Failed to compile source");

    let executor =
        MathicExecutor::new(&module, OptLvl::default()).expect("Failed to create executor");

    executor
        .call_function("main")
        .expect("Failed to execute main function")
}
