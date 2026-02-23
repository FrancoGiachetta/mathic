use crate::{
    compiler::{MathicCompiler, OptLvl},
    executor::MathicExecutor,
};

pub fn compile_and_execute(source: &str) -> i64 {
    let compiler = MathicCompiler::new().expect("Could not create compiler.");

    let module = compiler
        .compile_source(source, OptLvl::None)
        .expect("Failed to compile");
    let executor =
        MathicExecutor::new(&module, OptLvl::default()).expect("Failed to create executor");
    executor
        .call_function("main")
        .expect("Failed to execute main")
}
