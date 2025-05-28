use crate::chunk::{Chunk, OpCode};

pub fn dissemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    for (i, op) in chunk.code.iter().enumerate() {
        match op {
            OpCode::Return => println!("{:04} OP_RETURN", i),
            OpCode::Constant(index) => {
                println!("{:04} OP_CONSTANT {:04}", i, chunk.constants[*index]);
            } // Add more OpCode cases here as needed
        }
    }
}
