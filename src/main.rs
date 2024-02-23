extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;

use parser::{PureParser, Rule};
use pest::Parser;
use std::io::{stdin, Read};

fn main() {
    let src = read_src_from_stdin();
    let parsed = PureParser::parse(Rule::file, &src);
    if let Err(syntax_error) = parsed {
        eprintln!("Syntax error:\n{}", syntax_error);
        return;
    }
    println!("OK");
}

fn read_src_from_stdin() -> String {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Failed to read from stdin");
    buffer
}
