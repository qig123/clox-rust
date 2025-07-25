use crate::chunk::{Chunk, OpCode};
use crate::token::{Token, TokenType};
use crate::value::Value;
use std::iter::Peekable;
use std::rc::Rc;
use std::slice::Iter;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Precedence {
    // 从低到高排列
    ZERO,
    Equality,   //==   !=
    Comparison, // > < >= <=
    Term,       // 加减法，优先级最低
    Factor,     // 乘除法，优先级中等
    Unary,      // 一元负号，优先级最高
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
        let prev_token = self
            .advance()
            .ok_or("Expected an expression at start of file.")?;
        self.parse_prefix_rule(prev_token)?;
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
                let infix_token = self.advance().unwrap();
                self.parse_infix_rule(infix_token)?;
            } else {
                break;
            }
        }
        Ok(())
    }

    fn parse_prefix_rule(&mut self, token: Token<'a>) -> Result<(), String> {
        match token.token_type {
            TokenType::Number => {
                let value: f64 = token.lexeme.parse().unwrap();
                self.emit_constant(Value::from_number(value))?;
            }
            TokenType::String => {
                // 去掉前后的引号
                let s: String = token.lexeme[1..token.lexeme.len() - 1].to_string();
                self.emit_constant(Value::String(Rc::new(s)))?;
            }
            TokenType::LeftParen => {
                self.parse_precedence(Precedence::ZERO as u8)?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            }
            TokenType::Minus | TokenType::Bang => {
                let (_, r_bp) = self.prefix_binding_power(token.token_type).unwrap();
                self.parse_precedence(r_bp)?;
                match token.token_type {
                    TokenType::Minus => self.emit_opcode(OpCode::Negate),
                    TokenType::Bang => self.emit_opcode(OpCode::Not),
                    _ => unreachable!(),
                }
            }
            TokenType::True => self.emit_opcode(OpCode::True),
            TokenType::False => self.emit_opcode(OpCode::False),
            TokenType::Nil => self.emit_opcode(OpCode::Nil),

            _ => {
                return Err(format!(
                    "Error at line {}: Expected an expression, but found '{}'.",
                    token.line, token.lexeme
                ));
            }
        }
        Ok(())
    }
    fn parse_infix_rule(&mut self, token: Token<'a>) -> Result<(), String> {
        if let Some((_, r_bp)) = self.infix_binding_power(token.token_type) {
            self.parse_precedence(r_bp)?;

            match token.token_type {
                TokenType::Plus => self.emit_opcode(OpCode::Add),
                TokenType::Minus => self.emit_opcode(OpCode::Subtract),
                TokenType::Star => self.emit_opcode(OpCode::Multiply),
                TokenType::Slash => self.emit_opcode(OpCode::Divide),
                TokenType::EqualEqual => self.emit_opcode(OpCode::EQUAL),
                TokenType::BangEqual => {
                    self.emit_opcode(OpCode::EQUAL);
                    self.emit_opcode(OpCode::Not);
                }
                TokenType::Greater => self.emit_opcode(OpCode::GREATER),
                TokenType::GreaterEqual => {
                    self.emit_opcode(OpCode::LESS);
                    self.emit_opcode(OpCode::Not);
                }
                TokenType::Less => self.emit_opcode(OpCode::LESS),
                TokenType::LessEqual => {
                    self.emit_opcode(OpCode::GREATER);
                    self.emit_opcode(OpCode::Not);
                }
                _ => unreachable!(),
            }
        } else {
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
            TokenType::EqualEqual | TokenType::BangEqual => {
                Some((Equality as u8, Equality as u8 + 1))
            }
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => Some((Comparison as u8, Comparison as u8 + 1)),
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
    // 在 impl<'a> Compiler<'a> 中

    fn advance(&mut self) -> Option<Token<'a>> {
        // 1. 从迭代器中取出下一个 token
        let next_token = self.tokens.next().cloned();
        // 2. 更新 self.previous 状态 (副作用)
        self.previous = next_token.clone();
        // 3. 直接返回刚刚取出的 token (主要作用)
        next_token
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
