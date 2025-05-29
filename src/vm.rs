// vm.rs
use crate::{
    chunk::{self, Chunk, OpCode}, // Need Chunk struct definition
    compiler::Parser,
    debug,
    value::Value,
};

pub struct VM {
    ip: usize,
    stack: Vec<Value>,
    // Removed the chunk field from VM. VM will receive the chunk to run
    // as a parameter to the run method.
    // chunk: Chunk, // REMOVE THIS FIELD
}

impl VM {
    pub fn new() -> Self {
        VM {
            ip: 0,
            stack: Vec::new(),
            // Initialize chunk here if you add it back
            // chunk: Chunk::new(), // Only if VM owns the chunk permanently
        }
    }

    // 3. Modified interpret: Creates parser, compiles, gets chunk, and passes to run.
    pub fn interpret(&mut self, source: String) -> Result<(), String> {
        // Create the parser, it will create its own chunk
        let mut parser = Parser::new(source);

        // Compile the source. parser.compile() returns the compiled chunk or an error.
        let compiled_chunk = parser.compile()?; // Use ? to propagate parser errors

        // If compilation was successful, run the compiled chunk.
        self.run(&compiled_chunk)?; // 4. Pass a reference to the compiled chunk to run

        Ok(())
    }

    // 4. Modified run: Takes a reference to the Chunk to execute.
    fn run(&mut self, chunk: &Chunk) -> Result<(), String> {
        // Added chunk parameter
        // Reset instruction pointer and stack for the new execution
        self.ip = 0;
        self.stack.clear();

        loop {
            // 5. Access chunk data via the parameter
            let instruction = self.read_byte(chunk); // Pass chunk to read_byte

            // 仅在启用 `debug_print` 时打印调试信息
            #[cfg(feature = "debug_print")]
            {
                self.debug_print_stack();
                // 5. Access chunk data via the parameter
                debug::dissemble_instruction(self.ip - 1, &instruction, chunk); // Pass chunk to dissemble_instruction
            }

            match instruction {
                OpCode::Return => {
                    if self.stack.is_empty() {
                        return Err("Stack is empty, cannot return.".to_string());
                    } else {
                        // The value is on top of the stack, pop and print it.
                        // This assumes the script's result is left on the stack.
                        // For functions, the return instruction would handle returning
                        // the value and unwinding the call stack.
                        let result = self.stack.pop().unwrap();
                        println!("{}", result); // Use Debug or Display impl for Value
                        return Ok(()); // Script finished successfully
                    }
                }
                // 5. Access chunk data via the parameter
                OpCode::Constant(index) => {
                    // Ensure the index is within bounds
                    if index >= chunk.constants.len() {
                        return Err(format!("Invalid constant index: {}", index));
                    }
                    let c = &chunk.constants[index];
                    self.stack.push(c.clone()); // Clone constant value onto stack
                }
                OpCode::Negate => {
                    // Use perform_unary_numeric_op instead for consistency (need to implement it)
                    // Or keep it here and handle the stack pop/push directly.
                    if let Some(value) = self.stack.pop() {
                        match value {
                            Value::Number(n) => {
                                self.stack.push(Value::Number(-n));
                            }
                            // Handle other types or report runtime error
                            _ => {
                                return Err(format!(
                                    "Operand must be a number for Negate. Got {}",
                                    value
                                ));
                            }
                        }
                    } else {
                        return Err("Stack empty for Negate.".to_string());
                    }
                }
                // 5. Pass chunk to perform_binary_numeric_op
                OpCode::Add => self.perform_binary_numeric_op(chunk, |a, b| a + b)?,
                OpCode::Subtract => self.perform_binary_numeric_op(chunk, |a, b| a - b)?,
                OpCode::Multiply => self.perform_binary_numeric_op(chunk, |a, b| a * b)?,
                OpCode::Divide => self.perform_binary_numeric_op(chunk, |a, b| a / b)?,

                // Add other opcodes (Equality, Comparison, Logical, Jumps, etc.)
                // OpCode::Equal => self.perform_binary_equality_op()?, // Implement this
                // OpCode::Greater => self.perform_binary_comparison_op(|a, b| a > b)?, // Implement this
                // OpCode::Less => self.perform_binary_comparison_op(|a, b| a < b)?, // Implement this
                // OpCode::Not => self.perform_unary_logical_op()?, // Implement this
                // OpCode::JumpIfFalse(offset) => self.ip += offset, // Adjust based on how offsets are stored
                // OpCode::Jump(offset) => self.ip += offset,
                // OpCode::Pop => { self.stack.pop(); },
                // OpCode::False => self.stack.push(Value::Bool(false)),
                // OpCode::True => self.stack.push(Value::Bool(true)),
                // OpCode::Nil => self.stack.push(Value::Nil),
                // ... etc.
                _ => return Err(format!("Unknown opcode: {:?}", instruction)), // Catch unimplemented opcodes
            }
        }
    }

    // 5. Modified read_byte: Takes a reference to Chunk.
    fn read_byte(&mut self, chunk: &Chunk) -> OpCode {
        // Added chunk parameter
        if self.ip >= chunk.code.len() {
            // This should not happen if the compiled code ends with OpCode::Return
            // but as a safeguard:
            panic!("Attempted to read past end of bytecode.");
        }
        let byte = &chunk.code[self.ip];
        self.ip += 1;
        // Assuming OpCode implements Copy or Clone
        byte.clone() // Or *byte if OpCode is Copy
    }

    // 5. Modified debug_print_stack: Needs chunk for disassembling instruction.
    #[cfg(feature = "debug_print")] // Keep the conditional compilation
    fn debug_print_stack(&self) {
        // Added chunk parameter
        print!("          ");
        for value in &self.stack {
            print!("[ ");
            // Ensure Value has a Display or Debug implementation for printing
            print!("{}, ", value); // Use {:?} or {} depending on Value's traits
            print!(" ]");
        }
        println!();
        // The call to debug::dissemble_instruction is now in `run` and passes the chunk
        // debug::dissemble_instruction(self.ip - 1, &instruction, chunk); // REMOVE THIS LINE
    }
    #[cfg(not(feature = "debug_print"))] // Add the corresponding non-debug version
    fn debug_print_stack(&self, _chunk: &Chunk) {
        // Do nothing when debug_print feature is not enabled
    }

    // 5. Modified perform_binary_numeric_op: Needs chunk for division by zero check (accessing self.chunk.code)
    fn perform_binary_numeric_op<F>(&mut self, chunk: &Chunk, op: F) -> Result<(), String>
    // Added chunk parameter
    where
        F: Fn(f64, f64) -> f64, // 假设操作是在两个 f64 上进行
    {
        // Note: Consider implementing type checking more robustly.
        // This current implementation pops, then checks type.
        // A more robust approach might check types *before* popping.
        if self.stack.len() < 2 {
            return Err("Not enough values on the stack for binary operation.".to_string());
        }

        // Pop operands
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();

        match (a, b) {
            (Value::Number(a_num), Value::Number(b_num)) => {
                // Check for division by zero, needs the current instruction type.
                // self.ip points *after* the current instruction.
                // So the current instruction is at chunk.code[self.ip - 1].
                // 5. Access chunk data via the parameter
                if let OpCode::Divide = chunk.code[self.ip - 1] {
                    // Use chunk parameter
                    if b_num == 0.0 {
                        return Err("Division by zero.".to_string());
                    }
                }
                let result = op(a_num, b_num);
                self.stack.push(Value::Number(result));
                Ok(())
            } // Handle type errors for non-numeric operands
              // _ => {
              //     return Err(format!(
              //         "Operands must be numbers for binary operation. Got {} and {}",
              //         a, b
              //     ));
              // }
        }
    }

    // Add similar methods for other operand types (equality, logical, etc.)
    // fn perform_binary_equality_op(&mut self) -> Result<(), String> { ... }
    // fn perform_binary_comparison_op<F>(&mut self, op: F) -> Result<(), String> { ... }
    // fn perform_unary_logical_op(&mut self) -> Result<(), String> { ... }
}
