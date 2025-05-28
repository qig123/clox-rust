use crate::chunk::{Chunk, OpCode};

pub fn dissemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    for (i, op) in chunk.code.iter().enumerate() {
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
            } // Add more OpCode cases here as needed
        }
    }
}
