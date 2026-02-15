use std::path::PathBuf;

use clap::Parser;
use mathic::{
    MathicResult,
    compiler::{MathicCompiler, OptLvl},
    executor::MathicExecutor,
};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Debug, Parser)]
struct MathiCLI {
    file_path: PathBuf,
}

fn main() -> MathicResult<()> {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_env_filter(EnvFilter::from_default_env())
            .finish(),
    )
    .expect("Failed to set global suscriber");

    let args = MathiCLI::parse();

    let compiler = MathicCompiler::new()?;

    let module = compiler.compile_path(&args.file_path, OptLvl::default())?;
    let executor = MathicExecutor::new(&module, OptLvl::O1)?;

    tracing::debug!("Executor Created");
    let result = executor.call_function("main");

    tracing::debug!("Execution Done");
    println!("RESULT: {:?}", result);

    Ok(())
}
