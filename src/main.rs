use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
    vm::Vm,
};

mod chunk;
mod value;
mod vm;
fn main() {
    // 我们的目标是计算: (1.2 + 3.4) / 5.6 - 7.8 * 9.0
    // 预期结果: 4.6 / 5.6 - 70.2 = 0.8214... - 70.2 = -69.3785...
    println!("Testing expression: (1.2 + 3.4) / 5.6 - 7.8 * 9.0");
    println!("--------------------------------------------------");

    let mut chunk = Chunk::new();
    let line = 1; // 假设所有指令都在第1行

    // --- (1.2 + 3.4) ---
    let const_1 = chunk.add_constant(Value::from_number(1.2));
    chunk.write(OpCode::Constant(const_1), line);

    let const_2 = chunk.add_constant(Value::from_number(3.4));
    chunk.write(OpCode::Constant(const_2), line);

    chunk.write(OpCode::Add, line);

    // --- / 5.6 ---
    let const_3 = chunk.add_constant(Value::from_number(5.6));
    chunk.write(OpCode::Constant(const_3), line);

    chunk.write(OpCode::Divide, line);

    // --- 7.8 * 9.0 ---
    let const_4 = chunk.add_constant(Value::from_number(7.8));
    chunk.write(OpCode::Constant(const_4), line);

    let const_5 = chunk.add_constant(Value::from_number(9.0));
    chunk.write(OpCode::Constant(const_5), line);

    chunk.write(OpCode::Multiply, line);

    // --- 最后一步：减法 ---
    chunk.write(OpCode::Subtract, line);

    // 添加一个 Negate 来测试它是否还能工作
    chunk.write(OpCode::Negate, line); // 这会将结果取负

    // 最终返回并打印结果
    chunk.write(OpCode::Return, line);

    // 启用调试跟踪时，先打印完整的字节码
    #[cfg(feature = "debug_trace_execution")]
    {
        println!("\n== Disassembled Chunk ==\n");
        chunk.disassemble("Binary Ops Test");
        println!("\n== VM Execution ==\n");
    }

    let mut vm = Vm::new(chunk);
    let _ = vm.interpret();
}
