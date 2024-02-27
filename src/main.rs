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
use def::Program;
use parser::{PureParser, Rule};
use pest::Parser;
use std::fs;
use std::io::{self, Write};

#[derive(Clap, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to source code file
    source: String,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    let src = fs::read_to_string(args.source).expect("Failed to read source file");

    let parsed = PureParser::parse(Rule::file, &src);
    if let Err(syntax_error) = parsed {
        eprintln!("Syntax error:\n{}", syntax_error);
        return Ok(());
    }

    let ast: Result<AST, String> = parsed.unwrap().try_into();
    if let Err(semantic_error) = ast {
        eprintln!("Semantic error:\n{}", semantic_error);
        return Ok(());
    }

    let program: Program = ast.unwrap().into();
    println!("{:#?}", program);
    Ok(())
    // fs::File::create("main.pure.exe")
    //     .expect("Failed to create executable file")
    //     .write_all(program.as_vec().as_slice())
}
