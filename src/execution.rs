use bytes::{Bytes, BytesMut};
pub mod instruction;

use crate::evmstack::EVMStack;
use crate::memory::EVMMemory;
use crate::instruction::Instruction;


pub struct Execution<T> {
    code: Bytes,
    stack: EVMStack<T>,
    memory: EVMMemory,
    pc: u8,
    stopped: bool,
}

impl<T> Execution<T> {
    pub fn new(code: Bytes) -> Self {
        Self {
            code: code,
            stack: EVMStack::new(),
            memory: EVMMemory::new(256),
            pc: 0,
            stopped: false,
        }
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn read_code(&mut self, num_bytes: u8) -> u8 {
        let value = self.code[self.pc as usize + num_bytes as usize];
        self.pc += num_bytes;

        return value
    }
}