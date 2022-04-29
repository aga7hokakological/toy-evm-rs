use bytes::{Bytes, BytesMut};
// pub mod execution;

use crate::evmstack::EVMStack;
use crate::memory::EVMMemory;
use crate::execution::instruction::Instruction;


pub struct Execution<T> {
    pub code: Bytes,
    pub stack: EVMStack<T>,
    pub memory: EVMMemory,
    pub pc: u8,
    pub stopped: bool,
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

    // pub fn set_return_data(&self, offset: usize, len: usize) {
    //     self.stopped = true;
    //     self.return_data = self.memory.load_range(offset, len);
    // }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn read_code(&mut self, num_bytes: u8) -> u8 {
        let value = self.code[self.pc as usize + num_bytes as usize];
        self.pc += num_bytes;

        return value
    }
}