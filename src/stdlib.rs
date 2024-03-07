#![allow(non_camel_case_types)]

use std::collections::HashMap;

pub enum StdLib {
    ID, // func id(any) any

    Add_I32, // func add(i32, i32) i32
    Sub_I32, // func sub(i32, i32) i32
    Mul_I32, // func, mul(i32, i32) i32
    Div_I32, // func div,(i32, i32) i32

    Print, // func print(any)
}

pub fn index() -> HashMap<String, usize> {
    HashMap::from([
        ("std.id".to_string(), StdLib::ID as usize),
        ("std.add".to_string(), StdLib::Add_I32 as usize),
        ("std.sub".to_string(), StdLib::Sub_I32 as usize),
        ("std.mul".to_string(), StdLib::Mul_I32 as usize),
        ("std.div".to_string(), StdLib::Div_I32 as usize),
        ("std.prints".to_string(), StdLib::Print as usize),
    ])
}
