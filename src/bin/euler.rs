use std::path::PathBuf;

use clap::Parser;
use mathic::{
    MathicResult,
    compiler::{MathicCompiler, OptLvl},
    executor::MathicExecutor,
};

#[derive(Debug, Parser)]
struct MathiCLI {
    file_path: PathBuf,
}

fn main() -> MathicResult<()> {
    let args = MathiCLI::parse();

    let compiler = MathicCompiler::new()?;
    let module = compiler.compile(&args.file_path, OptLvl::default())?;
    let executor = MathicExecutor::new(&module, OptLvl::O1)?;

    dbg!("Executor Created");
    let result = executor.execute_main()?;

    dbg!("Execution Done");
    println!("RESULT: {}", result);

    Ok(())
}
