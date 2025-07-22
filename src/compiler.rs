// src/compiler.rs

use crate::ast::{Expr, LiteralValue};
use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Compiler
    }

    pub fn compile(&self, expr: &Expr) -> Result<Chunk, String> {
        let mut chunk = Chunk::new();
        self.compile_expression(expr, &mut chunk)?;

        // 每个表达式计算完后，都应该有一个 Return 指令来查看结果
        chunk.write(OpCode::Return, 1); // 假设行号是 1

        Ok(chunk)
    }

    fn compile_expression(&self, expr: &Expr, chunk: &mut Chunk) -> Result<(), String> {
        match expr {
            Expr::Literal(value) => match value {
                LiteralValue::Number(num) => {
                    // 1. 将数字包装成 Value
                    let value = Value::from_number(*num);
                    // 2. 将 Value 添加到 chunk 的常量池
                    let constant_index = chunk.add_constant(value);
                    // 3. 生成 OP_CONSTANT 指令
                    chunk.write(OpCode::Constant(constant_index), 1); // 假设行号是 1
                }
                _ => {
                    panic!()
                }
            },
            _ => {
                panic!()
            }
        }
        Ok(())
    }
}
