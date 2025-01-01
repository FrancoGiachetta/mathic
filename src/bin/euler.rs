use std::{fs::File, io::Read};

use clap::Parser;
use mathic::lexer::Lexer;

#[derive(Debug, Parser)]
struct MathCli {
    file_path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = MathCli::parse();
    let mut file = File::open(args.file_path)?;
    let mut input = String::new();

    file.read_to_string(&mut input)?;

    let mut lexer = Lexer::new(&input);

    lexer.lex()?;

    for t in lexer.tokens().iter() {
        println!("{t}");
    }

    Ok(())
}
