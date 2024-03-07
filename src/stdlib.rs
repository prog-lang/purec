#![allow(non_camel_case_types)]

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
