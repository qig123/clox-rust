// src/parser.rs

use crate::ast::{Expr, LiteralValue};
use crate::token::{Token, TokenType};

pub struct Parser<'a> {
    tokens: &'a [Token<'a>],
    current: usize,
}
#[derive(Debug)]
pub struct ParseError<'a> {
    pub token: Token<'a>,
    pub message: String,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token<'a>]) -> Self {
        Parser { tokens, current: 0 }
    }

    // 主入口，目标是解析一个完整的表达式
    pub fn parse(&mut self) -> Result<Expr, ParseError<'a>> {
        self.expression()
    }

    // 解析表达式的规则 (现在只有一个)
    fn expression(&mut self) -> Result<Expr, ParseError<'a>> {
        self.literal()
    }

    fn literal(&mut self) -> Result<Expr, ParseError<'a>> {
        if let Some(token) = self.tokens.get(self.current) {
            self.current += 1;
            match token.token_type {
                TokenType::Number => match token.lexeme.parse::<f64>() {
                    Ok(num) => Ok(Expr::Literal(LiteralValue::Number(num))),
                    Err(_) => Err(ParseError {
                        token: *token,
                        message: "Could not parse number.".to_string(),
                    }),
                },
                _ => Err(ParseError {
                    token: *token,
                    message: format!("Expected a literal, but found {:?}", token.token_type),
                }),
            }
        } else {
            // 对于文件末尾的错误，我们可以创建一个虚拟的 Token
            let eof_token = *self.tokens.last().unwrap(); // 获取最后的 EOF token
            Err(ParseError {
                token: eof_token,
                message: "Unexpected end of input.".to_string(),
            })
        }
    }
}
