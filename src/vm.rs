use crate::{
    chunk::{Chunk, OpCode},
    compiler::Compiler,
    token::Token,
    value::Value,
};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Vm<'a> {
    tokens: Vec<Token<'a>>,
    chunk: Chunk,
    ip: usize, // ip (instruction pointer) 指向即将被执行的指令
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
}
const STACK_MAX: usize = 256;
#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    // CompileError,
    RuntimeError,
}
macro_rules! runtime_error {
    ($vm:expr, $($arg:tt)*) => {{
        $vm.runtime_error(&format!($($arg)*));
        return InterpretResult::RuntimeError;
    }};
}

impl<'a> Vm<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Vm {
            tokens,
            chunk: Chunk::new(),
            ip: 0,
            stack: Vec::with_capacity(STACK_MAX),
            globals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        let c = Compiler::new(&self.tokens);
        let r = c.compile();
        match r {
            Ok(chunk) => self.chunk = chunk,
            Err(e) => {
                eprintln!("[Compiler Error] {}", e);
                return InterpretResult::RuntimeError;
            }
        }
        self.run()
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            // 在执行指令之前，如果启用了跟踪，就打印它
            #[cfg(feature = "debug_trace_execution")]
            {
                print!("          ");
                // 打印栈的状态，这对于调试极其有用！
                for value in &self.stack {
                    print!("[ {} ]", value);
                }
                println!(); // 换行
                self.chunk.disassemble_instruction(self.ip);
            }

            // 读取指令，然后增加 ip
            let instruction = &self.chunk.code[self.ip];
            self.ip += 1;

            match instruction {
                OpCode::Return => {
                    // if let Ok(result) = self.pop() {
                    //     println!("最后求值结果是:{}", result);
                    // }
                    return InterpretResult::Ok;
                }
                OpCode::Constant(index) => {
                    let constant = self.chunk.values[*index].clone();
                    self.push(constant);
                }
                OpCode::Negate => match self.peek(0) {
                    Some(&Value::Number(_)) => {
                        if let Ok(Value::Number(n)) = self.pop() {
                            self.push(Value::Number(-n));
                        }
                    }
                    _ => runtime_error!(self, "Operand must be a number."),
                },
                OpCode::Add => {
                    if let (Some(&Value::String(_)), Some(&Value::String(_))) =
                        (self.peek(0), self.peek(1))
                    {
                        let b_val = self.pop().unwrap();
                        let a_val = self.pop().unwrap();
                        if let (Value::String(a_rc), Value::String(b_rc)) = (a_val, b_val) {
                            let mut s = String::with_capacity(a_rc.len() + b_rc.len());
                            s.push_str(&a_rc);
                            s.push_str(&b_rc);
                            self.push(Value::String(Rc::new(s)));
                        }
                    } else if let (Some(&Value::Number(_)), Some(&Value::Number(_))) =
                        (self.peek(0), self.peek(1))
                    {
                        if self.binary_op(|a, b| Value::Number(a + b)).is_err() {
                            runtime_error!(self, "Operands must be two numbers.");
                        }
                    } else {
                        runtime_error!(self, "Operands must be two numbers or two strings.");
                    }
                }
                OpCode::Multiply => {
                    if self.binary_op(|a, b| Value::Number(a * b)).is_err() {
                        runtime_error!(self, "Operands must be numbers.");
                    }
                }
                OpCode::Subtract => {
                    if self.binary_op(|a, b| Value::Number(a - b)).is_err() {
                        runtime_error!(self, "Operands must be numbers.");
                    }
                }
                OpCode::Divide => {
                    if self.binary_op(|a, b| Value::Number(a / b)).is_err() {
                        runtime_error!(self, "Operands must be numbers.");
                    }
                }
                OpCode::True => self.push(Value::Bool(true)),
                OpCode::False => self.push(Value::Bool(false)),
                OpCode::Nil => self.push(Value::Nil),
                OpCode::Not => {
                    let value = self.pop().unwrap();
                    self.push(Value::Bool(value.is_falsey()));
                }
                OpCode::EQUAL => {
                    let b = self.pop().unwrap();
                    let a = self.pop().unwrap();
                    self.push(Value::Bool(a == b));
                }
                OpCode::GREATER => {
                    if self.binary_op(|a, b| Value::Bool(a > b)).is_err() {
                        runtime_error!(self, "Operands must be numbers.");
                    }
                }
                OpCode::LESS => {
                    if self.binary_op(|a, b| Value::Bool(a < b)).is_err() {
                        runtime_error!(self, "Operands must be numbers.");
                    }
                }
                OpCode::Print => {
                    println!("{}", self.pop().unwrap());
                }
                OpCode::Pop => {
                    self.pop().unwrap();
                }
                OpCode::DefineGlobal(index) => {
                    if let Value::String(name) = &self.chunk.values[*index] {
                        self.globals.insert(name.to_string(), self.peek(0).unwrap().clone());
                        let _ = self.pop();
                    }
                }
                OpCode::GetGlobal(index) => {
                    if let Value::String(name) = &self.chunk.values[*index] {
                        if let Some(value) = self.globals.get(name.as_ref()) {
                            self.push(value.clone());
                        } else {
                            runtime_error!(self, "Undefined variable '{}'.", name);
                        }
                    }
                }
                OpCode::SetGlobal(index) => {
                    if let Value::String(name) = &self.chunk.values[*index] {
                        if self.globals.contains_key(name.as_ref()) {
                            self.globals.insert(name.to_string(), self.peek(0).unwrap().clone());
                        } else {
                            runtime_error!(self, "Undefined variable '{}'.", name);
                        }
                    }
                }
            }
        }
    }
    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
    fn pop(&mut self) -> Result<Value, ()> {
        self.stack.pop().ok_or(())
    }
    fn peek(&self, distance: usize) -> Option<&Value> {
        self.stack.get(self.stack.len() - 1 - distance)
    }
    fn binary_op<F>(&mut self, op: F) -> Result<(), ()>
    where
        F: FnOnce(f64, f64) -> Value,
    {
        if let (Some(&Value::Number(_)), Some(&Value::Number(_))) = (self.peek(0), self.peek(1)) {
            let b = self.pop().unwrap();
            let a = self.pop().unwrap();
            if let (Value::Number(num_a), Value::Number(num_b)) = (a, b) {
                self.push(op(num_a, num_b));
                Ok(())
            } else {
                unreachable!()
            }
        } else {
            Err(())
        }
    }
    fn runtime_error(&mut self, message: &str) {
        eprintln!("{}", message);
        // ip 已经前进了1，所以要减1找到出错指令的位置
        let line = self.chunk.lines[self.ip - 1];
        eprintln!("[line {}] in script", line);
        // 清空栈，防止后续操作
        self.stack.clear();
    }
}