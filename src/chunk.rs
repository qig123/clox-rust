use crate::value::Value;

pub struct Chunk {
    code: Vec<OpCode>,
    values: Vec<Value>,
    lines: Vec<usize>,
}
pub enum OpCode {
    Return,
    Constant(usize),
}
impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            values: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: OpCode, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.values.push(value);
        self.values.len() - 1
    }
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        for (offset, _instruction) in self.code.iter().enumerate() {
            self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) {
        print!("{:04} ", offset);
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }

        if let Some(instruction) = self.code.get(offset) {
            match instruction {
                OpCode::Return => self.simple_instruction("OP_RETURN"),
                OpCode::Constant(constant_index) => {
                    self.constant_instruction("OP_CONSTANT", *constant_index)
                }
            }
        }
    }

    /// 辅助函数：处理常量指令
    fn constant_instruction(&self, name: &str, constant_index: usize) {
        if let Some(value) = self.values.get(constant_index) {
            // {name:<16} - 左对齐，宽度为16
            // {constant_index:4} - 右对齐（默认），宽度为4
            // '{value}' - 打印常量值
            println!("{:<16} {:4} '{}'", name, constant_index, value);
        } else {
            println!("{:<16} {:4} (invalid)", name, constant_index);
        }
    }

    /// 辅助函数：处理简单指令（没有操作数的指令）
    fn simple_instruction(&self, name: &str) {
        println!("{}", name);
    }
}
