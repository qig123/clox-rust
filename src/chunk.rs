use crate::value::Value;
#[derive(Debug, Clone)]
pub enum OpCode {
    Negate,
    Return,
    Constant(usize),
    Add,
    Subtract,
    Multiply,
    Divide,
}
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub line_numbers: Vec<usize>, // Optional: to track line numbers for debugging
}
impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            line_numbers: Vec::new(), // Initialize with an empty vector
        }
    }
    pub fn add_constant(&mut self, constant: Value) -> usize {
        self.constants.push(constant);
        self.constants.len() - 1
    }
    pub fn write_chunk(&mut self, op: OpCode, current_line_number: usize) {
        self.code.push(op);
        // Optionally, you can also track the line number for each operation
        self.line_numbers.push(current_line_number); // You would need to define how to get the current line number
    }
}
