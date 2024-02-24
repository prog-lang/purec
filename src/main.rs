mod asm;

use asm::{Op::*, Program};
use std::io::{self, stdout, Write};

fn main() -> Result<(), io::Error> {
    let std_print = 5;
    let program = Program::new(vec![
        NOP,
        PUSH_FN(std_print),
        PUSH_I32(42),
        FEED(1),
        CALL,
        RETURN,
    ]);
    stdout().write_all(program.as_vec().as_slice())
}

// fn read_src_from_stdin() -> String {
//     let mut buffer = String::new();
//     stdin()
//         .read_to_string(&mut buffer)
//         .expect("Failed to read from stdin");
//     buffer
// }
