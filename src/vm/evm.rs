use primitive_types::U256;
use crate::vm::stack::Stack;

pub struct EVM {
    pub pc: usize,
    pub stack: Stack,
    pub bytecode: Vec<u8>,
    pub halted: bool,
}

impl EVM {
    pub fn new(bytecode: Vec<u8>) -> Self {
        Self {
            pc: 0,
            stack: Stack::new(),
            bytecode,
            halted: false,
        }
    }
}