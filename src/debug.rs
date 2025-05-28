use crate::chunk::{Chunk, OpCode};
#[allow(unused)]
pub fn dissemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    for (i, op) in chunk.code.iter().enumerate() {
        dissemble_instruction(i, op, chunk);
    }
}
pub fn dissemble_instruction(i: usize, op: &OpCode, chunk: &Chunk) {
    print!("{:04} ", i);
    if i > 0 && chunk.line_numbers[i] == chunk.line_numbers[i - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.line_numbers[i]);
    }
    match op {
        OpCode::Return => println!("OP_RETURN"),
        OpCode::Constant(index) => {
            println!("OP_CONSTANT {}", chunk.constants[*index]);
        }
        OpCode::Negate => println!("OP_NEGATE"),
        OpCode::Add => println!("OP_ADD"),
        OpCode::Subtract => println!("OP_SUBTRACT"),
        OpCode::Multiply => println!("OP_MULTIPLY"),
        OpCode::Divide => println!("OP_DIVIDE"),
        // Add more OpCode cases here as needed
    }
}
