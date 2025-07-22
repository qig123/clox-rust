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
    // -- 语法规则函数 --
    fn expression(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        self.equality()
    }
    // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.tokens[self.current - 1];
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }
    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        let mut expr = self.term()?;

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.tokens[self.current - 1];
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }
    // term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.tokens[self.current - 1];
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }
    // factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        let mut expr = self.unary()?; // 左侧操作数

        // 循环匹配 `*` 或 `/`
        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.tokens[self.current - 1];
            let right = self.unary()?;
            // 将之前的表达式作为新的二元表达式的左侧部分
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    // unary          → ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            // 注意这里我们拿的是被消费掉的前一个 token
            let operator = self.tokens[self.current - 1];
            // 递归调用 unary，可以处理像 --1 或 !!false 这样的情况
            let right = self.unary()?;
            Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            })
        } else {
            // 如果没有一元操作符，就是一个 primary 表达式
            self.primary()
        }
    }

    // primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        // 提前拿到上一个 token 的引用，用于出错时回溯
        // 注意：peek 在这里比 advance 更好，因为它不消耗 token，方便错误处理
        if let Some(token) = self.peek() {
            let token_copy = *token; // 创建一个副本，因为接下来可能会消耗它
            match token_copy.token_type {
                TokenType::False
                | TokenType::True
                | TokenType::Nil
                | TokenType::Number
                | TokenType::String => {
                    self.advance(); // 匹配成功，消耗 token
                    return match token_copy.token_type {
                        TokenType::False => Ok(Expr::Literal(LiteralValue::Boolean(false))),
                        TokenType::True => Ok(Expr::Literal(LiteralValue::Boolean(true))),
                        TokenType::Nil => Ok(Expr::Literal(LiteralValue::Nil)),
                        TokenType::Number => {
                            let value = token_copy.lexeme.parse::<f64>().unwrap();
                            Ok(Expr::Literal(LiteralValue::Number(value)))
                        }
                        TokenType::String => {
                            let value = &token_copy.lexeme[1..token_copy.lexeme.len() - 1];
                            Ok(Expr::Literal(LiteralValue::String(value.to_string())))
                        }
                        _ => unreachable!(), // 逻辑上不可能到达这里
                    };
                }
                TokenType::LeftParen => {
                    self.advance(); // 消耗 '('
                    let expr = self.expression()?;
                    if self.match_token(&[TokenType::RightParen]) {
                        return Ok(Expr::Grouping(Box::new(expr)));
                    } else {
                        // 错误发生在 ')' 的位置，所以 peek() 指向的 token 是最合适的
                        return Err(ParseError {
                            token: *self.peek().unwrap_or(&token_copy),
                            message: "Expected ')' after expression.".to_string(),
                        });
                    }
                }
                _ => {
                    // 不匹配任何 primary 表达式，报错
                    return Err(ParseError {
                        token: token_copy,
                        message: "Expected a primary expression.".to_string(),
                    });
                }
            }
        }

        // 如果 peek() 返回 None，说明已经到文件末尾了
        Err(ParseError {
            token: *self.tokens.last().unwrap(), // 使用真实的 EOF token
            message: "Expected primary expression, but reached end of file.".to_string(),
        })
    }

    // -- 辅助函数 --

    // 查看当前 token 但不消费它
    fn peek(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.current)
    }

    // 检查是否已到末尾
    fn is_at_end(&self) -> bool {
        self.peek()
            .map_or(true, |token| token.token_type == TokenType::Eof)
    }

    // 消费当前 token 并前进
    fn advance(&mut self) -> Option<&Token<'a>> {
        if !self.is_at_end() {
            let token = self.tokens.get(self.current);
            self.current += 1;
            token
        } else {
            None
        }
    }

    // 检查当前 token 类型是否匹配
    fn check(&self, token_type: TokenType) -> bool {
        self.peek()
            .map_or(false, |token| token.token_type == token_type)
    }

    // 如果当前 token 类型匹配给定的类型之一，就消费它并返回 true
    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for &t_type in types {
            if self.check(t_type) {
                self.advance();
                return true;
            }
        }
        false
    }
}
