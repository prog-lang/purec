#[macro_use]
extern crate pest_derive;
extern crate pest;

mod asm;
mod parser;

use clap::Parser as Clap;
use parser::{PureParser, Rule};
use pest::Parser;

#[derive(Clap, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to source code file
    source: String,
}

fn main() {
    let args = Args::parse();
    let src = std::fs::read_to_string(args.source).expect("failed to read source file");
    let parsed = PureParser::parse(Rule::file, &src);
    if let Err(syntax_error) = parsed {
        eprintln!("Syntax error:\n{}", syntax_error);
    }
}
