use crate::{
    chunk::{Chunk, OpCode},
    compiler::Compiler,
    token::Token,
    value::Value,
};

pub struct Vm<'a> {
    tokens: Vec<Token<'a>>,
    chunk: Chunk,
    ip: usize, // ip (instruction pointer) 指向即将被执行的指令
    stack: Vec<Value>,
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
            // 注意：我们将 ip 的增加移到了这里，这更符合 ip 的定义
            // "instruction pointer" 指向下一个要执行的指令
            let instruction = &self.chunk.code[self.ip];
            self.ip += 1;

            match instruction {
                OpCode::Return => {
                    if let Ok(result) = self.pop() {
                        println!("{}", result);
                        return InterpretResult::Ok;
                    } else {
                        self.runtime_error("Stack underflow on return.");
                        return InterpretResult::RuntimeError;
                    }
                }
                OpCode::Constant(index) => {
                    let constant = self.chunk.values[*index].clone();
                    self.push(constant);
                }
                OpCode::Negate => match self.pop().ok() {
                    Some(Value::Number(n)) => self.push(Value::Number(-n)),
                    Some(_) => runtime_error!(self, "Operand must be a number."),
                    None => runtime_error!(self, "Stack underflow on negate."),
                },
                OpCode::Add => {
                    if self.binary_op(|a, b| Value::Number(a + b)).is_err() {
                        runtime_error!(self, "Operands must be numbers.");
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
                    let value = self.pop().ok();
                    match value {
                        Some(Value::Bool(b)) => self.push(Value::Bool(!b)),
                        Some(Value::Nil) => self.push(Value::Bool(true)),
                        _ => self.push(Value::Bool(false)),
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
    fn binary_op<F>(&mut self, op: F) -> Result<(), ()>
    where
        F: FnOnce(f64, f64) -> Value,
    {
        // pop()返回Result,所以我们可以用'?'来简化错误处理
        // 但这里我们需要区分“栈下溢”和“类型错误”，所以手动match更好
        let b = match self.pop() {
            Ok(val) => val,
            Err(_) => return Err(()), // 栈下溢，但我们在这里把它当作通用错误
        };
        let a = match self.pop() {
            Ok(val) => val,
            Err(_) => return Err(()),
        };

        match (a, b) {
            (Value::Number(num_a), Value::Number(num_b)) => {
                self.push(op(num_a, num_b));
                Ok(()) // 操作成功
            }
            _ => Err(()), // 操作数类型错误
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
