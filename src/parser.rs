// src/parser.rs

use crate::ast::{Expr, LiteralValue, Stmt};
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
    pub fn parse(&mut self) -> Result<Vec<Stmt<'a>>, ParseError<'a>> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            // 解析一个语句并添加到列表中
            statements.push(self.statement()?);
        }
        Ok(statements)
    }
    // statement      → exprStmt | printStmt ;
    fn statement(&mut self) -> Result<Stmt<'a>, ParseError<'a>> {
        // 查看下一个 token 来决定是哪种语句
        if self.match_token(&[TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    // printStmt      → "print" expression ";" ;
    fn print_statement(&mut self) -> Result<Stmt<'a>, ParseError<'a>> {
        // "print" 关键字已经被 match_token 消费掉了
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(value))
    }

    // exprStmt       → expression ";" ;
    fn expression_statement(&mut self) -> Result<Stmt<'a>, ParseError<'a>> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(expr))
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
    // 检查并消费一个 token，如果类型不匹配则返回错误
    fn consume(
        &mut self,
        token_type: TokenType,
        message: &str,
    ) -> Result<Token<'a>, ParseError<'a>> {
        if self.check(token_type) {
            // advance 返回 Option<&Token>，这里我们确定它不是 None
            // 使用 clone 是因为 ParseError 需要拥有 Token
            Ok(*self.advance().unwrap())
        } else {
            Err(ParseError {
                token: *self.peek().unwrap(), // 错误发生在这里
                message: message.to_string(),
            })
        }
    }

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
