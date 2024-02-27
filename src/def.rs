use std::collections::HashMap;

use crate::{
    ast::{Declaration, Expr, AST},
    stdlib,
};

#[derive(Debug, PartialEq)]
pub struct Program {
    definitions: Vec<Definition>,
    index: HashMap<String, usize>,
}

#[derive(Debug, PartialEq)]
struct Definition {
    id: String,
    code: Vec<Op>,
}

#[derive(Debug, PartialEq)]
enum Op {
    NOP,       // DO NOTHING
    ARGC(u32), // Specify argument count for Cmd

    /* Stack manipulation */
    PUSH_UNIT,        // Push unit onto the stack
    PUSH_BOOL(bool),  // Push bool onto the stack
    PUSH_U8(u8),      // Push u8 onto the stack
    PUSH_I32(i32),    // Push i32 onto the stack
    PUSH_FN(String),  // Push fn onto the stack (std)
    PUSH_CMD(String), // Push cmd onto the stack
    PUSH_ARG(u32),    // Push cmd argument (by its index) onto the stack
    DROP(u32),        // Drop top value off of the stack

    /* Program flow */
    FEED(u32), // FEED N top values into the function beneath
    BRANCH,    // BRANCH left or right based on a condition
    RETURN,    // Return from the routine
}

impl From<AST> for Program {
    fn from(ast: AST) -> Self {
        let definitions: Vec<Definition> = ast
            .declarations
            .iter()
            .map(|decl| from(&ast, decl))
            .collect();

        Self {
            definitions,
            index: stdlib::index(),
        }
        .indexed()
    }
}

impl Program {
    fn indexed(mut self) -> Self {
        let mut offset = 0;
        for def in self.definitions.iter() {
            self.index.insert(def.id.clone(), offset);
            offset += def.code.len();
        }
        self
    }
}

fn from(ast: &AST, declaration: &Declaration) -> Definition {
    let expr = &declaration.expr;
    let code = argc(expr)
        .into_iter()
        .chain(expand(ast, expr).into_iter())
        .chain(vec![Op::RETURN].into_iter())
        .collect();
    Definition {
        id: declaration.id.clone(),
        code,
    }
}

fn argc(expr: &Expr) -> Vec<Op> {
    let count = match expr {
        Expr::Func(params, _) => params.len() as u32,
        _ => 0,
    };
    vec![Op::ARGC(count)]
}

fn expand(ast: &AST, expr: &Expr) -> Vec<Op> {
    match expr {
        Expr::Int(i) => vec![Op::PUSH_I32(*i)],
        Expr::Name(_) => todo!(),
        Expr::ID(id) => {
            if id.starts_with("std.") {
                vec![Op::PUSH_FN(id.clone())]
            } else {
                match ast.get_declaration(id).expr {
                    // Treat it as a closure.
                    Expr::Func(_, _) => {
                        vec![Op::PUSH_CMD(id.clone())]
                    }
                    // Treat it as value by calling it with a Unit argument.
                    other => vec![Op::PUSH_CMD(id.clone()), Op::PUSH_UNIT, Op::FEED(1)],
                }
            }
        }
        Expr::Call(f, args) => expand(ast, f)
            .into_iter()
            .chain(args.iter().map(|expr| expand(ast, expr)).flatten())
            .chain(vec![Op::FEED(args.len() as u32)].into_iter())
            .collect(),
        Expr::Func(_, body) => expand(ast, body),
    }
}
