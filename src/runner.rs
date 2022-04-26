use crate::execution::Execution;

pub fn run(code: Bytes) {
    let mut context = Execution::new(code);

    while !context.stop {
        let pc_before = context.pc;
        let instruction = decode_opcode(context);
        instruction.execute(context)

        println!("{:?} @ pc={:?}", instruction, pc_before);
        println!("{:?}", context);
    }

    
}