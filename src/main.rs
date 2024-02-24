mod asm;

use asm::{Op::*, Program};
use std::io::{self, stdout, Write};

fn main() -> Result<(), io::Error> {
    let data = vec![1, 2, 3, 4];

    let std_print = 5;
    let code = vec![NOP, PUSH_FN(std_print), PUSH_I32(58), FEED(1), CALL, RETURN];

    let program = Program::from(data, code);

    stdout().write_all(program.as_vec().as_slice())
}

// fn read_src_from_stdin() -> String {
//     let mut buffer = String::new();
//     stdin()
//         .read_to_string(&mut buffer)
//         .expect("Failed to read from stdin");
//     buffer
// }
