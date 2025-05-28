use chunk::OpCode;

mod chunk;
mod debug;
mod value;
mod vm;
fn main() {
    let mut chunk = chunk::Chunk::new();
    let index = chunk.add_constant(value::Value::Number(42.0));
    chunk.write_chunk(OpCode::Constant(index), 15);

    let index = chunk.add_constant(value::Value::Number(29.0));
    chunk.write_chunk(OpCode::Constant(index), 15);

    chunk.write_chunk(OpCode::Add, 15);

    let index = chunk.add_constant(value::Value::Number(10.0));
    chunk.write_chunk(OpCode::Constant(index), 15);
    chunk.write_chunk(OpCode::Subtract, 15);

    chunk.write_chunk(OpCode::Negate, 15);

    chunk.write_chunk(OpCode::Return, 15);
    let mut vm = vm::VM::new(&chunk);
    if let Err(e) = vm.interpret() {
        eprintln!("Error: {}", e);
    }
    // debug::dissemble_chunk(&chunk, "test chunk");
}
