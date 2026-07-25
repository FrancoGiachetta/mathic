use std::{env, path::{Path, PathBuf}};

use clap::{self, Parser, Subcommand, Args};
use mathic::{
    MathicResult, compiler::{MathicCompiler, OptLvl}, executor::MathicExecutor,
};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Debug, Parser)]
struct MathiCLI {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    New {
        project_name: String,
    },
    Run(RunCmdArgs)
}

#[derive(Debug, Args)]
struct RunCmdArgs {
    file_path: PathBuf,
    #[clap(short, long, default_value_t = 2)]
    opt_lvl: usize,
}

fn main() -> MathicResult<()> {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_env_filter(EnvFilter::from_default_env())
            .finish(),
    )
    .expect("Failed to set global suscriber");

    match MathiCLI::parse().command {
        Command::New { project_name } => create_project(project_name)?,
        Command::Run (RunCmdArgs { file_path, opt_lvl }) => {
            compile_and_run_source(&file_path, opt_lvl.into())?;
        }
    };

    Ok(())
}

fn create_project(project_name: String) -> MathicResult<()> {
    let curr_dir = env::current_dir()?;
    let project_path = curr_dir.join(&project_name);
    std::fs::create_dir_all(&project_path.join("src"))?;

    let main_file_path = project_path.join("src/main.mth");
    let main_file_content = r#"df main() i32 {
    sym x:expr<i32> ;
    let y: i32 = 5;

    return eval(x+y, x, 10);
}"#;

    std::fs::write(&main_file_path, main_file_content)?;

    println!("Project '{}' created successfully!", project_name);

    Ok(())
}

fn compile_and_run_source(source: &Path, opt_lvl: OptLvl) -> MathicResult<()> {
    let compiler = MathicCompiler::new()?;
    let module = compiler.compile_path(source, opt_lvl)?;
    let executor = MathicExecutor::new(&module, opt_lvl)?;

    tracing::debug!("Executor Created");
    let result = executor.call_function("main");

    tracing::debug!("Execution Done");
    println!("RESULT: {:?}", result);

    Ok(())
}
