pub struct Program {
    data: Vec<u8>,
    code: Vec<Op>,
}

impl Program {
    pub fn new(code: Vec<Op>) -> Self {
        Self {
            data: vec![],
            code: code,
        }
    }

    pub fn as_vec(&self) -> Vec<u8> {
        let mut v = (self.data.len() as i32).to_le_bytes().to_vec();
        v.extend_from_slice(self.data.as_slice());
        for op in self.code.iter() {
            v.extend_from_slice(op.as_vec().as_slice());
        }
        v
    }
}

enum Opcode {
    NOP, // DO NOTHING

    /* Stack manipulation */
    PUSH_UNIT, // Push unit onto the stack
    PUSH_BOOL, // Push bool onto the stack
    PUSH_U8,   // Push u8 onto the stack
    PUSH_I32,  // Push i32 onto the stack
    PUSH_FN,   // Push fn onto the stack (std)
    PUSH_CMD,  // Push cmd onto the stack
    DROP,      // Drop top value off of the stack

    /* Program flow */
    FEED,   // FEED N top values into the function beneath
    CALL,   // CALL top function off of the stack
    BRANCH, // BRANCH left or right based on a condition
    RETURN, // Return from the routine
}

pub enum Op {
    NOP, // DO NOTHING

    /* Stack manipulation */
    PUSH_UNIT,       // Push unit onto the stack
    PUSH_BOOL(bool), // Push bool onto the stack
    PUSH_U8(u8),     // Push u8 onto the stack
    PUSH_I32(i32),   // Push i32 onto the stack
    PUSH_FN(i32),    // Push fn onto the stack (std)
    PUSH_CMD(i32),   // Push cmd onto the stack
    DROP(i32),       // Drop top value off of the stack

    /* Program flow */
    FEED(i32), // FEED N top values into the function beneath
    CALL,      // CALL top function off of the stack
    BRANCH,    // BRANCH left or right based on a condition
    RETURN,    // Return from the routine
}

impl Op {
    pub fn as_vec(&self) -> Vec<u8> {
        match self {
            Self::NOP => vec![Opcode::NOP as u8, 0, 0, 0, 0],
            Self::PUSH_UNIT => vec![Opcode::PUSH_UNIT as u8, 0, 0, 0, 0],
            Self::PUSH_BOOL(b) => vec![Opcode::PUSH_BOOL as u8, *b as u8, 0, 0, 0],
            Self::PUSH_U8(u) => vec![Opcode::PUSH_U8 as u8, *u, 0, 0, 0],
            Self::PUSH_I32(i) => Self::join(Opcode::PUSH_I32, &i.to_le_bytes()),
            Self::PUSH_FN(addr) => Self::join(Opcode::PUSH_FN, &addr.to_le_bytes()),
            Self::PUSH_CMD(addr) => Self::join(Opcode::PUSH_CMD, &addr.to_le_bytes()),
            Self::DROP(n) => Self::join(Opcode::DROP, &n.to_le_bytes()),
            Self::FEED(argc) => Self::join(Opcode::FEED, &argc.to_le_bytes()),
            Self::CALL => vec![Opcode::CALL as u8, 0, 0, 0, 0],
            Self::BRANCH => vec![Opcode::BRANCH as u8, 0, 0, 0, 0],
            Self::RETURN => vec![Opcode::RETURN as u8, 0, 0, 0, 0],
        }
    }

    fn join(opcode: Opcode, slice: &[u8]) -> Vec<u8> {
        let mut v = vec![opcode as u8];
        v.extend_from_slice(slice);
        v
    }
}
