use crate::chunk::{Chunk, OpCode};
use crate::token::{Token, TokenType};
use crate::value::Value;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Precedence {
    // 从低到高排列
    ZERO,
    Term,   // 加减法，优先级最低
    Factor, // 乘除法，优先级中等
    Unary,  // 一元负号，优先级最高
}

pub struct Compiler<'a> {
    tokens: Peekable<Iter<'a, Token<'a>>>,
    chunk: Chunk,
    previous: Option<Token<'a>>,
}

impl<'a> Compiler<'a> {
    pub fn new(tokens: &'a Vec<Token<'a>>) -> Self {
        Compiler {
            tokens: tokens.iter().peekable(),
            chunk: Chunk::new(),
            previous: None,
        }
    }

    pub fn compile(mut self) -> Result<Chunk, String> {
        while !self.check(TokenType::Eof) {
            self.parse_precedence(Precedence::ZERO as u8)?;
        }
        self.emit_return();
        println!("{:?}", self.chunk.code);
        println!("{:?}", self.chunk.values);
        Ok(self.chunk)
    }

    fn parse_precedence(&mut self, min_precedence: u8) -> Result<(), String> {
        // 1. 处理前缀
        self.advance();
        let prev_token = self.previous.as_ref().unwrap().clone();
        self.parse_prefix_rule(prev_token.token_type)?;
        // 2. 循环处理中缀
        loop {
            if let Some(current_token) = self.peek().cloned() {
                // 如果下一个 token 没有中缀规则，或者优先级太低，就停止
                if let Some((l_bp, _)) = self.infix_binding_power(current_token.token_type) {
                    if l_bp < min_precedence {
                        break;
                    }
                } else {
                    break;
                }

                // 消费并处理这个中缀操作符
                self.advance();
                let infix_token = self.previous.as_ref().unwrap().clone();
                self.parse_infix_rule(infix_token.token_type)?;
            } else {
                break;
            }
        }
        Ok(())
    }

    // 新增：处理前缀规则的辅助函数
    fn parse_prefix_rule(&mut self, token_type: TokenType) -> Result<(), String> {
        // `self.previous` 在这里就是刚刚被 advance 的前缀 token
        let prev_token = self.previous.as_ref().unwrap().clone();

        match token_type {
            TokenType::Number => {
                let value: f64 = prev_token.lexeme.parse().unwrap();
                self.emit_constant(Value::from_number(value))?;
            }
            TokenType::LeftParen => {
                self.parse_precedence(Precedence::ZERO as u8)?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            }
            TokenType::Minus | TokenType::Bang => {
                let (_, r_bp) = self.prefix_binding_power(token_type).unwrap();
                self.parse_precedence(r_bp)?;

                if token_type == TokenType::Minus {
                    self.emit_opcode(OpCode::Negate);
                }
                // else if token_type == TokenType::Bang { self.emit_opcode(OpCode::Not); }
            }
            _ => {
                return Err(format!(
                    "Error at line {}: Expected an expression, but found '{}'.",
                    prev_token.line, prev_token.lexeme
                ));
            }
        }
        Ok(())
    }

    // 新增：处理中缀规则的辅助函数
    fn parse_infix_rule(&mut self, token_type: TokenType) -> Result<(), String> {
        // `self.previous` 在这里是刚刚被 advance 的中缀 token
        // 我们用传入的 token_type 来决定做什么
        if let Some((_, r_bp)) = self.infix_binding_power(token_type) {
            // 递归解析右侧表达式
            self.parse_precedence(r_bp)?;

            // 发射操作码
            match token_type {
                TokenType::Plus => self.emit_opcode(OpCode::Add),
                TokenType::Minus => self.emit_opcode(OpCode::Subtract),
                TokenType::Star => self.emit_opcode(OpCode::Multiply),
                TokenType::Slash => self.emit_opcode(OpCode::Divide),
                _ => unreachable!("Infix rule called for a non-infix token"),
            }
        } else {
            // 这不应该发生，因为调用者已经检查过绑定力了
            unreachable!();
        }
        Ok(())
    }

    fn prefix_binding_power(&self, op: TokenType) -> Option<((), u8)> {
        use Precedence::*;
        match op {
            TokenType::Minus | TokenType::Bang => Some(((), Unary as u8)),
            _ => None,
        }
    }

    fn infix_binding_power(&self, op: TokenType) -> Option<(u8, u8)> {
        use Precedence::*;

        match op {
            TokenType::Plus | TokenType::Minus => Some((Term as u8, Term as u8 + 1)),

            TokenType::Star | TokenType::Slash => Some((Factor as u8, Factor as u8 + 1)),

            // 如果需要赋值操作符（右结合）
            // TokenType::Equal => Some((Assignment as u8, Assignment as u8)),
            _ => None,
        }
    }
    //代码生成辅助
    fn emit_opcode(&mut self, opcode: OpCode) {
        let line = self
            .previous
            .as_ref()
            .expect("Cannot emit opcode without a previous token. Did you forget to advance?")
            .line;
        self.chunk.write(opcode, line);
    }

    // 发射一个没有参数的 OpCode
    // fn emit_opcodes(&mut self, op1: OpCode, op2: OpCode) {
    //     self.emit_opcode(op1);
    //     self.emit_opcode(op2);
    // }

    // 示例：发射一个返回指令
    fn emit_return(&mut self) {
        self.emit_opcode(OpCode::Return);
    }

    // 示例：发射一个常量
    fn emit_constant(&mut self, value: Value) -> Result<(), String> {
        let constant_index = self.chunk.add_constant(value);
        if constant_index > u8::MAX as usize {
            // 常量索引通常限制在一个字节内
            return Err("Too many constants in one chunk.".to_string());
        }
        self.emit_opcode(OpCode::Constant(constant_index));
        Ok(())
    }
    // ---- 解析辅助方法 ----
    fn advance(&mut self) -> Option<Token<'a>> {
        let next_token = self.tokens.next();
        self.previous = next_token.cloned();
        self.previous.clone()
    }

    fn peek(&mut self) -> Option<&Token<'a>> {
        self.tokens.peek().map(|token| *token)
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        match self.peek() {
            Some(token) => token.token_type == token_type,
            None => false,
        }
    }
    fn consume(&mut self, token_type: TokenType, error_message: &str) -> Result<Token<'a>, String> {
        if self.check(token_type) {
            Ok(self.advance().unwrap())
        } else {
            let p = self.previous;
            let found_token_str = match self.peek() {
                Some(token) => format!("found '{:?}'", token.token_type),
                None => "found end of file".to_string(),
            };
            Err(format!(
                "Error at line {}: {}. {}",
                self.peek().map_or(p.as_ref().unwrap().line, |t| t.line),
                error_message,
                found_token_str
            ))
        }
    }
}
