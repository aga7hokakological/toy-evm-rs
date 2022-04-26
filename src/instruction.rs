use std::collections::HashMap;

pub mod execution;
use crate::execution::Execution;

pub struct Instruction {
    opcode: u64, 
    name: String,
}

impl Instruction {

    pub fn new(opcode: u64, name: String) -> Self {
        Self { 
            opcode: opcode,
            name: name,
        }
    }

    pub fn execute(&self, context: Execution) {
        panic!("Not implemented")
    }
}

let mut INSTRUCTION: Instruction = vec![];
let mut INSTRUCTION_BY_OPCODE = HashMap::new();

pub fn register_instruction(
    opcode: u32, 
    name: String, 
    // execute_func: 
) {
    let mut instruction = Instruction::new(opcode, name);
    instruction.execute();

    INSTRUCTION.push(instruction);

    INSTRUCTION_BY_OPCODE.insert(opcode, instruction)
}

pub fn decode_opcode(context: Execution) -> Instruction {
    if context.pc < 0 {
        panic!("Invalid code offset::=> code: {} pc: {}", context.code, context.pc)
    }

    if context.pc >= context.code.len() {
        return 0
    }

    let opcode = context.read_code(1);
    let instruction = INSTRUCTION_BY_OPCODE.get(opcode);
    if instruction == None {
        panic!("{:?}", opcode)
    }
}