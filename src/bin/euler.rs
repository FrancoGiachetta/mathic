use clap::Parser;

#[derive(Debug, Parser)]
struct MathCli {
    file_path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
