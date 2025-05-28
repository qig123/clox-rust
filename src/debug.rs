use crate::chunk::OpCode;

pub fn dissemble_chunk(chunk: &Vec<OpCode>, name: &str) {
    println!("== {} ==", name);
    for (i, op) in chunk.iter().enumerate() {
        match op {
            OpCode::Return => println!("{:04} OP_RETURN", i),
            // Add more OpCode cases here as needed
        }
    }
}
