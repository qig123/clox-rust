use chunk::OpCode;

mod chunk;
mod debug;
mod value;
fn main() {
    let mut chunk = chunk::Chunk::new();
    let index = chunk.add_constant(value::Value::Number(42.0));
    chunk.write_chunk(OpCode::Constant(index), 15);
    chunk.write_chunk(OpCode::Return, 15);
    debug::dissemble_chunk(&chunk, "test chunk");
}
