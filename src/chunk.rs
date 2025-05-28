use crate::value::Value;

pub enum OpCode {
    Return,
    Constant(usize),
}
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
}
impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }
    pub fn add_constant(&mut self, constant: Value) -> usize {
        self.constants.push(constant);
        self.constants.len() - 1
    }
    pub fn write_chunk(&mut self, op: OpCode) {
        self.code.push(op);
    }
}
