use std::collections::HashMap;

// pub mod execution;
use crate::execution::execution::Execution;

#[derive(Debug)]
pub struct Instruction {
    pub opcode: u64, 
    pub name: String,
}

impl Instruction {

    pub fn new(opcode: u64, name: String) -> Self {
        Self { 
            opcode: opcode,
            name: name,
        }
    }

    // pub fn execute(&self) {
    //     panic!("Not implemented")
    // }

    pub fn register_instruction(
        // &self
        opcode: u32, 
        name: String, 
        // execute_func: bool
    ) -> Instruction {

        let instruction = Instruction::new(opcode.into(), name);
        // instruction.execute() = execute_func;

        // let instruction_closure = |i| i ;
        // instruction_closure(instructions.push(instruction));

        // instructions_by_opcode.insert(opcode, instruction);
        return instruction
    }
}

