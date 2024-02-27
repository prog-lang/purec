#[macro_use]
extern crate pest_derive;
extern crate pest;
extern crate strum;

mod asm;
mod ast;
mod def;
mod parser;
mod stdlib;
mod valid;

use ast::AST;
use clap::Parser as Clap;
use parser::{PureParser, Rule};
use pest::Parser;
use std::io::{self, Write};
use std::{fs, process};

#[derive(Clap, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to source code file
    source: String,

    /// Path to output file
    #[arg(short, long, default_value_t = String::from("main.pure.exe"))]
    output: String,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    let src = fs::read_to_string(args.source).expect("Failed to read source file");

    let parsed = PureParser::parse(Rule::file, &src);
    if let Err(syntax_error) = parsed {
        eprintln!("Syntax error:\n{}", syntax_error);
        process::exit(1);
    }

    let ast: Result<AST, String> = parsed.unwrap().try_into();
    if let Err(semantic_error) = ast {
        eprintln!("Semantic error:\n{}", semantic_error);
        process::exit(1);
    }

    let program: asm::Program = def::Program::from(ast.unwrap()).into();
    fs::File::create(args.output)
        .expect("Failed to create executable file")
        .write_all(program.as_vec().as_slice())
}
