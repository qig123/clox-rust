use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};

mod chunk;
mod value;
fn main() {
    let mut chunk = Chunk::new();

    let const_index = chunk.add_constant(Value::from(1.2));
    chunk.write(OpCode::Constant(const_index), 123);

    let const_index_2 = chunk.add_constant(Value::from(3.4));
    chunk.write(OpCode::Constant(const_index_2), 123);

    chunk.write(OpCode::Return, 124);

    chunk.disassemble("test chunk");
}
