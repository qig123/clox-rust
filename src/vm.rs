use crate::{
    chunk::{Chunk, OpCode},
    debug,
};

pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: usize,
}
impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        VM { chunk, ip: 0 }
    }
    pub fn interpret(&mut self) -> Result<(), String> {
        self.run()
    }
    fn run(&mut self) -> Result<(), String> {
        loop {
            let instruction = self.read_byte();
            //每次执行前打印指令
            debug::dissemble_instruction(self.ip - 1, &instruction, self.chunk);
            match instruction {
                OpCode::Return => {
                    return Ok(());
                }
                OpCode::Constant(index) => {
                    let _ = &self.chunk.constants[index];
                }
            }
        }
    }
    fn read_byte(&mut self) -> OpCode {
        let byte = &self.chunk.code[self.ip];
        self.ip += 1;
        byte.clone()
    }
}
