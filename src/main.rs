use bytes::{Bytes, BytesMut};
use crate::evmstack::EVMStack;
use crate::memory::EVMMemory;
use crate::execution::Execution;

pub mod evmstack;
pub mod memory;
pub mod execution;

fn main() {
    let mut stack: EVMStack<u32> = EVMStack::new();
    let memory: EVMMemory = EVMMemory::new(256);
    let mut execution: Execution<u32> = Execution::new(Bytes::from("Hello World"));

    stack.push(1);
    stack.push(2);
    println!("stack: {:?}", stack);
    stack.pop();
    println!("stack: {:?}", stack);

    // memory.store(50, 50);
    println!("memory: {:?}", memory);

    execution.stop();
}
