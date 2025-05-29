use crate::{chunk::OpCode, compiler::compile, debug, value::Value};

pub struct VM {
    ip: usize,
    stack: Vec<Value>,
}
impl VM {
    pub fn new() -> Self {
        VM {
            ip: 0,
            stack: Vec::new(),
        }
    }
    pub fn interpret(&mut self, source: &str) {
        compile(source);
    }
    // fn run(&mut self) -> Result<(), String> {
    //     loop {
    //         let instruction = self.read_byte();
    //         // 仅在启用 `debug_print` 时打印调试信息
    //         #[cfg(feature = "debug_print")]
    //         {
    //             self.debug_print_stack();
    //             debug::dissemble_instruction(self.ip - 1, &instruction, self.chunk);
    //         }
    //         match instruction {
    //             OpCode::Return => {
    //                 if self.stack.is_empty() {
    //                     return Err("Stack is empty, cannot return.".to_string());
    //                 } else {
    //                     println!("{}", self.stack.pop().unwrap());
    //                     return Ok(());
    //                 }
    //             }
    //             OpCode::Constant(index) => {
    //                 let c = &self.chunk.constants[index];
    //                 self.stack.push(c.clone());
    //             }
    //             OpCode::Negate => {
    //                 if let Some(value) = self.stack.pop() {
    //                     match value {
    //                         Value::Number(n) => {
    //                             self.stack.push(Value::Number(-n));
    //                         }
    //                     }
    //                 } else {
    //                     return Err("Stack is empty, cannot negate.".to_string());
    //                 }
    //             }
    //             OpCode::Add => self.perform_binary_numeric_op(|a, b| a + b)?,
    //             OpCode::Subtract => self.perform_binary_numeric_op(|a, b| a - b)?,
    //             OpCode::Multiply => self.perform_binary_numeric_op(|a, b| a * b)?,
    //             OpCode::Divide => self.perform_binary_numeric_op(|a, b| a / b)?,
    //         }
    //     }
    // }
    // fn read_byte(&mut self) -> OpCode {
    //     let byte = &self.chunk.code[self.ip];
    //     self.ip += 1;
    //     byte.clone()
    // }
    // fn debug_print_stack(&self) {
    //     print!("          ");
    //     for value in &self.stack {
    //         print!("[ ");
    //         print!("{}, ", value);
    //         print!(" ]");
    //     }
    //     println!();
    // }
    // fn perform_binary_numeric_op<F>(&mut self, op: F) -> Result<(), String>
    // where
    //     F: Fn(f64, f64) -> f64, // 假设操作是在两个 f64 上进行
    // {
    //     if self.stack.len() < 2 {
    //         return Err("Not enough values on the stack for binary operation.".to_string());
    //     }
    //     let b = self.stack.pop().unwrap();
    //     let a = self.stack.pop().unwrap();

    //     match (a, b) {
    //         (Value::Number(a_num), Value::Number(b_num)) => {
    //             if let OpCode::Divide = self.chunk.code[self.ip - 1] {
    //                 if b_num == 0.0 {
    //                     return Err("Division by zero.".to_string());
    //                 }
    //             }
    //             let result = op(a_num, b_num);
    //             self.stack.push(Value::Number(result));
    //             Ok(())
    //         }
    //     }
    // }
}
