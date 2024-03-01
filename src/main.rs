#[macro_use]
extern crate pest_derive;
extern crate im_rc as im;
extern crate pest;
extern crate polytype;

mod asm;
mod ast;
mod def;
mod js;
mod infer;
mod parser;
mod stdlib;
mod types;
mod valid;

use ast::AST;
use clap::Parser as Clap;
use parser::{PureParser, Rule};
use pest::iterators::Pairs;
use pest::Parser;
use std::io::{self, Write};
use std::{fs, process};

#[derive(Clap, Debug)]
#[command(version, about, long_about = None)]
struct App {
    /// Path to source code file
    source: String,

    /// Path to output file
    #[arg(short, long, default_value_t = String::from("main.js"))]
    output: String,

    /// Output architecture (vm | js)
    #[arg(long, default_value_t = String::from("node"))]
    arch: String,
}

impl App {
    fn run(self) -> Result<(), io::Error> {
        self.compile(self.read_source()?)
    }

    fn read_source(&self) -> io::Result<String> {
        fs::read_to_string(&self.source)
    }

    fn compile(&self, input: String) -> Result<(), io::Error> {
        self.generate_executable_code(Self::parse_input(input))
    }

    fn parse_input(input: String) -> AST {
        let parsed = PureParser::parse(Rule::file, &input);
        match parsed {
            Err(syntax_error) => {
                exit(format!("Syntax error:\n{}", syntax_error));
                AST::default()
            }
            Ok(pairs) => Self::form_ast(pairs),
        }
    }

    fn form_ast(pairs: Pairs<'_, Rule>) -> AST {
        let attempt: Result<AST, String> = pairs.try_into();
        match attempt {
            Err(semantic_error) => {
                exit(format!("Semantic error:\n{}", semantic_error));
                AST::default()
            }
            Ok(ast) => ast,
        }
    }

    fn generate_executable_code(&self, ast: AST) -> Result<(), io::Error> {
        match self.arch.as_str() {
            "vm" => {
                let program: asm::Program = def::Program::from(ast).into();
                fs::File::create(&self.output)
                    .expect("Failed to create executable file")
                    .write_all(program.as_vec().as_slice())
            }
            "node" => {
                let program: js::Program = ast.into();
                let code: String = program.into();
                fs::File::create(&self.output)
                    .expect("Failed to create executable file")
                    .write_all(code.as_bytes())
            }
            unknown_arch => {
                exit(format!("Unknown arch '{}'", unknown_arch));
                Ok(())
            }
        }
    }
}

fn main() -> Result<(), io::Error> {
    return App::parse().run();
}

fn exit(message: String) {
    eprintln!("{}", message);
    process::exit(1);
}
