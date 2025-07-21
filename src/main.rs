use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
    vm::Vm,
};

mod chunk;
mod value;
mod vm;
fn main() {
    let mut chunk = Chunk::new();

    // 计算 -1.2
    // 1. 将 1.2 加载到常量池
    let constant = chunk.add_constant(Value::from_number(1.2));
    // 2. 生成 OP_CONSTANT 指令，将 1.2 从常量池推到栈上
    chunk.write(OpCode::Constant(constant), 123);
    // 3. 生成 OP_NEGATE 指令，弹出 1.2，计算 -1.2，再推回栈上
    chunk.write(OpCode::Negate, 123);
    // 4. 生成 OP_RETURN 指令，弹出栈顶的值并打印
    chunk.write(OpCode::Return, 123);

    #[cfg(feature = "debug_trace_execution")]
    {
        chunk.disassemble("test chunk");
        println!();
    }

    let mut vm = Vm::new(chunk);
    let _ = vm.interpret();
}
