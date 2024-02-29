#![allow(non_camel_case_types)]

use polytype::{ptp, tp, Type, TypeScheme};
use std::collections::HashMap;

pub enum StdLib {
    ID,
    Iff,

    Add_I32,
    Sub_I32,
    Mul_I32,
    Div_I32,

    Prints,
}

pub struct Function {
    pub index: usize,
    pub t: TypeScheme,
}

pub fn index() -> HashMap<String, usize> {
    HashMap::from([
        ("std.id".to_string(), StdLib::ID as usize),
        ("std.iff".to_string(), StdLib::Iff as usize),
        ("std.add".to_string(), StdLib::Add_I32 as usize),
        ("std.sub".to_string(), StdLib::Sub_I32 as usize),
        ("std.mul".to_string(), StdLib::Mul_I32 as usize),
        ("std.div".to_string(), StdLib::Div_I32 as usize),
        ("std.prints".to_string(), StdLib::Prints as usize),
    ])
}

pub fn function(name: &str) -> Option<Function> {
    match name {
        "std.id" => Some(Function {
            index: StdLib::ID as usize,
            t: ptp!(0; @arrow[tp!(0), tp!(0)]),
        }),
        "std.add" => Some(Function {
            index: StdLib::Add_I32 as usize,
            t: ptp!(@arrow[tp!(Int), tp!(Int), tp!(Int)]),
        }),
        "std.sub" => Some(Function {
            index: StdLib::Sub_I32 as usize,
            t: ptp!(@arrow[tp!(Int), tp!(Int), tp!(Int)]),
        }),
        "std.mul" => Some(Function {
            index: StdLib::Mul_I32 as usize,
            t: ptp!(@arrow[tp!(Int), tp!(Int), tp!(Int)]),
        }),
        "std.div" => Some(Function {
            index: StdLib::Div_I32 as usize,
            t: ptp!(@arrow[tp!(Int), tp!(Int), tp!(Int)]),
        }),
        "std.print" => Some(Function {
            index: StdLib::Print as usize,
            t: ptp!(0; @arrow[tp!(0), tp!(Cmd)]),
        }),
        _ => None,
    }
}
