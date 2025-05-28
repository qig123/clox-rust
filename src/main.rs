use chunk::OpCode;

mod chunk;
mod debug;
fn main() {
    let mut chunk: Vec<OpCode> = Vec::new();
    chunk.push(OpCode::Return);
    debug::dissemble_chunk(&chunk, "test chunk");
}
