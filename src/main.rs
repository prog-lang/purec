mod asm;

use asm::Op::*;
use std::io::{stdout, Write};

fn main() {
    stdout().write_all(PUSH_I32(42).as_vec().as_slice());
}

// fn read_src_from_stdin() -> String {
//     let mut buffer = String::new();
//     stdin()
//         .read_to_string(&mut buffer)
//         .expect("Failed to read from stdin");
//     buffer
// }
