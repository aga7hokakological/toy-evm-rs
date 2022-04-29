pub mod evmstack;
pub mod memory;
pub mod execution;

use std::collections::HashMap;
use bytes::{Bytes, BytesMut};
use crate::evmstack::EVMStack;
use crate::memory::EVMMemory;
use crate::execution::execution::Execution;
use crate::execution::instruction::Instruction;


fn main() {

    let instructions: Vec<Instruction> = Vec::new();
    let instructions_by_opcode: HashMap<String, Instruction> = HashMap::new();
    let mut stack: EVMStack<Instruction> = EVMStack::new();
    let memory: EVMMemory = EVMMemory::new(256);
    // let mut execution: Execution<u32> = Execution::new(Bytes::from("Hello World"));

    let STOP = Instruction::register_instruction(
        0x00,
        "STOP".to_string(),
    );

    let PUSH1 = Instruction::register_instruction(
        0x60,
        "PUSH1".to_string(),
    );

    stack.push(STOP);
    stack.push(PUSH1);

    println!("{:?}", stack);

    // execution.stop();

    // pub fn register_instruction(
    //     opcode: u32, 
    //     name: String, 
    //     // execute_func: 
    // ) {

    //     let mut instruction = Instruction::new(opcode.into(), name);
    //     instruction.execute();

    //     let instruction_closure = |i| i ;
    //     instruction_closure(instructions.push(instruction));

    //     instructions_by_opcode.insert(opcode, instruction);
    // }

    // pub fn decode_opcode<T>(context: Execution<T>) -> Instruction where T: u32 {
    //     if context.pc < 0 {
    //         panic!("Invalid code offset::=> code: {} pc: {}", context.code, context.pc)
    //     }

    //     if context.pc >= context.code.len() {
    //         return context.instruction
    //     }

    //     let opcode = context.read_code(1);
    //     let instruction = instructions_by_opcode.get(opcode);
    //     if instruction == None {
    //         panic!("{:?}", opcode)
    //     }

    //     return instruction
    // }
}
