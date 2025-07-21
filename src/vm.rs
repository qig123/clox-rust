use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};

pub struct Vm {
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
    // 宏接受 Vm 实例和格式化字符串
    ($vm:expr, $($arg:tt)*) => {{
        $vm.runtime_error(&format!($($arg)*));
        return InterpretResult::RuntimeError;
    }};
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Vm {
            chunk,
            ip: 0,
            stack: Vec::with_capacity(STACK_MAX),
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
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
                OpCode::Negate => {
                    match self.pop().ok() {
                        // pop()返回Result, ok()将其转为Option
                        Some(Value::Number(n)) => self.push(Value::Number(-n)),
                        // Some(_) => runtime_error!(self, "Operand must be a number."),
                        None => runtime_error!(self, "Stack underflow on negate."),
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
    fn runtime_error(&mut self, message: &str) {
        eprintln!("{}", message);
        // ip 已经前进了1，所以要减1找到出错指令的位置
        let line = self.chunk.lines[self.ip - 1];
        eprintln!("[line {}] in script", line);
        // 清空栈，防止后续操作
        self.stack.clear();
    }
}
