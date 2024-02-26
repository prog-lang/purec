#![allow(dead_code)]
#![allow(non_camel_case_types)]

pub struct Program {
    data: Vec<u8>,
    code: Vec<Op>,
}

impl Program {
    pub fn new(code: Vec<Op>) -> Self {
        Self::from(vec![], code)
    }

    pub fn from(data: Vec<u8>, code: Vec<Op>) -> Self {
        Self { data, code }
    }

    pub fn as_vec(&self) -> Vec<u8> {
        let mut v = self.data_vec();
        for op in self.code.iter() {
            v.extend_from_slice(op.as_vec().as_slice());
        }
        v
    }

    /// Data is going to be aligned to 64 bits.
    fn data_vec(&self) -> Vec<u8> {
        let length = self.aligned_data_length();
        let padding = length - self.data.len();
        let mut v = (length as u64).to_le_bytes().to_vec();
        v.extend_from_slice(self.data.as_slice());
        v.extend_from_slice(vec![0; padding].as_slice());
        v
    }

    fn aligned_data_length(&self) -> usize {
        let n = self.data.len();
        let bits = n & 7;
        if bits == 0 {
            n
        } else {
            n + (8 - bits)
        }
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
    PUSH_ARG,  // Push cmd argument (by its index) onto the stack
    DROP,      // Drop top value off of the stack

    /* Program flow */
    FEED,   // FEED N top values into the function beneath
    BRANCH, // BRANCH left or right based on a condition
    RETURN, // Return from the routine
}

pub enum Op {
    NOP,       // DO NOTHING
    ARGC(u32), // Specify argument count for Cmd

    /* Stack manipulation */
    PUSH_UNIT,       // Push unit onto the stack
    PUSH_BOOL(bool), // Push bool onto the stack
    PUSH_U8(u8),     // Push u8 onto the stack
    PUSH_I32(i32),   // Push i32 onto the stack
    PUSH_FN(u32),    // Push fn onto the stack (std)
    PUSH_CMD(u32),   // Push cmd onto the stack
    PUSH_ARG(u32),   // Push cmd argument (by its index) onto the stack
    DROP(u32),       // Drop top value off of the stack

    /* Program flow */
    FEED(u32), // FEED N top values into the function beneath
    BRANCH,    // BRANCH left or right based on a condition
    RETURN,    // Return from the routine
}

impl Opcode {
    pub fn as_vec(self) -> Vec<u8> {
        (self as u32).to_le_bytes().to_vec()
    }
}

impl Op {
    pub fn as_vec(&self) -> Vec<u8> {
        match self {
            Self::NOP => Self::just(Opcode::NOP),
            Self::ARGC(argc) => Self::join(Opcode::NOP, &argc.to_le_bytes()),
            Self::PUSH_UNIT => Self::just(Opcode::PUSH_UNIT),
            Self::PUSH_BOOL(b) => Self::join(Opcode::PUSH_BOOL, &[*b as u8, 0, 0, 0]),
            Self::PUSH_U8(u) => Self::join(Opcode::PUSH_U8, &[*u, 0, 0, 0]),
            Self::PUSH_I32(i) => Self::join(Opcode::PUSH_I32, &i.to_le_bytes()),
            Self::PUSH_FN(addr) => Self::join(Opcode::PUSH_FN, &addr.to_le_bytes()),
            Self::PUSH_CMD(addr) => Self::join(Opcode::PUSH_CMD, &addr.to_le_bytes()),
            Self::PUSH_ARG(index) => Self::join(Opcode::PUSH_ARG, &index.to_le_bytes()),
            Self::DROP(n) => Self::join(Opcode::DROP, &n.to_le_bytes()),
            Self::FEED(argc) => Self::join(Opcode::FEED, &argc.to_le_bytes()),
            Self::BRANCH => Self::just(Opcode::BRANCH),
            Self::RETURN => Self::just(Opcode::RETURN),
        }
    }

    fn just(opcode: Opcode) -> Vec<u8> {
        Self::join(opcode, &0_i32.to_le_bytes())
    }

    fn join(opcode: Opcode, slice: &[u8]) -> Vec<u8> {
        let mut v = opcode.as_vec();
        v.extend_from_slice(slice);
        v
    }
}
