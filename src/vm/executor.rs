use primitive_types::U256;

use crate::vm::{
    evm::EVM,
    opcodes::*,
};

impl EVM {
    pub fn run(&mut self) {
        while self.pc < self.bytecode.len() && !self.halted {
            let opcode = self.bytecode[self.pc];
            self.execute(opcode);
        }
    }

    fn execute(&mut self, opcode: u8) {
        match opcode {
            STOP => {
                self.halted = true;
                self.pc += 1;
            }

            ADD => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(a + b);
                self.pc += 1;
            }

            SUB => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(a - b);
                self.pc += 1;
            }

            MUL => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(a * b);
                self.pc += 1;
            }

            DIV => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(if b == U256::zero() { U256::zero() } else { a / b });
                self.pc += 1;
            }

            PUSH1 => {
                let value = self.bytecode[self.pc + 1];
                self.stack.push(U256::from(value));
                self.pc += 2;
            }

            POP => {
                self.stack.pop();
                self.pc += 1;
            }

            DUP1 => {
                let val = *self.stack.peek().unwrap();
                self.stack.push(val);
                self.pc += 1;
            }

            SWAP1 => {
                let len = self.stack.len();
                self.stack.swap(len - 1, len - 2);
                self.pc += 1;
            }

            _ => panic!("Unknown opcode: {:x}", opcode),
        }
    }
}