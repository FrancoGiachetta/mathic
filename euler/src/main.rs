use std::{env, fs};

use clap::{self, Args, Parser, Subcommand, ValueEnum};
use mathic::{
    MathicError,
    compiler::{CompilerOpts, MathicCompiler, OptLvl},
    executor::{MathicExecutor, jit::MathicJITExecutor},
};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::{config::ConfigToml, error::EulerError};

mod config;
mod error;

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
    Run(CompilerOptionsArgs),
}

#[derive(Debug, Clone, Args)]
struct CompilerOptionsArgs {
    #[clap(short, long, value_enum, default_value_t = OptLvlArg::O2)]
    opt_lvl: OptLvlArg,
    #[clap(long)]
    dump_mathir: bool,
    #[clap(long)]
    dump_mlir: bool,
    #[clap(long)]
    dump_llvmir: bool,
}

#[derive(Debug, Clone, ValueEnum)]
enum OptLvlArg {
    O0,
    O1,
    O2,
    O3,
}

impl From<CompilerOptionsArgs> for CompilerOpts {
    fn from(args: CompilerOptionsArgs) -> Self {
        CompilerOpts {
            opt_lvl: match args.opt_lvl {
                OptLvlArg::O0 => OptLvl::None,
                OptLvlArg::O1 => OptLvl::O1,
                OptLvlArg::O2 => OptLvl::O2,
                OptLvlArg::O3 => OptLvl::O3,
            },
            dump_mathir: args.dump_mathir,
            dump_mlir: args.dump_mlir,
            dump_llvmir: args.dump_llvmir,
        }
    }
}

fn main() -> Result<(), EulerError> {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_env_filter(EnvFilter::from_default_env())
            .finish(),
    )
    .expect("Failed to set global suscriber");

    match MathiCLI::parse().command {
        Command::New { project_name } => create_project(project_name)?,
        Command::Run(compiler_opts) => {
            compile_project(compiler_opts.into())?;
        }
    };

    Ok(())
}

fn create_project(project_name: String) -> Result<(), EulerError> {
    let curr_dir = env::current_dir()?;
    let project_path = curr_dir.join(&project_name);
    std::fs::create_dir_all(project_path.join("src"))?;

    let config_toml = ConfigToml::new(&project_name);
    let mathic_toml_path = project_path.join("Mathic.toml");

    let main_file_path = project_path.join("src/main.mth");
    let main_file_content = r#"df main() i32 {
    sym x:expr<i32> ;
    let y: i32 = 5;

    return eval(x+y, x, 10);
}"#;

    std::fs::write(mathic_toml_path, toml::to_string(&config_toml)?)?;
    std::fs::write(&main_file_path, main_file_content)?;

    println!("Project '{}' created successfully!", project_name);

    Ok(())
}

fn compile_project(compiler_opts: CompilerOpts) -> Result<(), EulerError> {
    if !fs::exists(env::current_dir()?.join("src/main.mth"))? {
        return Err(EulerError::MainFileNotFound);
    }

    let compiler = MathicCompiler::new()?;

    let src_root = env::current_dir()?.join("src");

    let modules = match compiler.compile_project(&src_root, compiler_opts) {
        Ok(modules) => modules,
        Err(MathicError::CompilationFailed) => {
            compiler.diagnostics().print_all()?;
            std::process::exit(1);
        }
        Err(e) => {
            return Err(EulerError::from(e));
        }
    };

    let executor = MathicJITExecutor::new(modules, compiler_opts)?;

    tracing::debug!("Executor Created");
    let result = executor.call_function("main::main");

    tracing::debug!("Execution Done");
    println!("RESULT: {:?}", result);

    Ok(())
}
