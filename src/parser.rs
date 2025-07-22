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
            // 现在解析声明，而不是语句
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }
    // program        → declaration* EOF ;
    // declaration    → varDecl | statement ;
    fn declaration(&mut self) -> Result<Stmt<'a>, ParseError<'a>> {
        if self.match_token(&[TokenType::Class]) {
            self.class_declaration()
        } else if self.match_token(&[TokenType::Fun]) {
            self.function("function")
        } else if self.match_token(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }
    fn class_declaration(&mut self) -> Result<Stmt<'a>, ParseError<'a>> {
        let name = self.consume(TokenType::Identifier, "Expect class name.")?;

        // 解析可选的父类
        let superclass = if self.match_token(&[TokenType::Less]) {
            self.consume(TokenType::Identifier, "Expect superclass name.")?;
            // 将父类名包装成一个 Variable 表达式
            Some(Expr::Variable {
                name: self.tokens[self.current - 1],
            })
        } else {
            None
        };

        self.consume(TokenType::LeftBrace, "Expect '{' before class body.")?;

        let mut methods = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            // 类体内部都是方法（在 Lox 中是 Stmt::Function）
            methods.push(self.function("method")?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after class body.")?;

        Ok(Stmt::Class {
            name,
            superclass,
            methods,
        })
    }
    fn function(&mut self, kind: &str) -> Result<Stmt<'a>, ParseError<'a>> {
        let name = self.consume(TokenType::Identifier, &format!("Expect {} name.", kind))?;
        self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {} name.", kind),
        )?;

        let mut params = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if params.len() >= 255 {
                    // 不直接报错，而是记录一个错误，但继续解析
                    // 这是一个高级技巧，现在可以先直接返回 Err
                    return Err(ParseError {
                        token: *self.peek().unwrap(),
                        message: "Can't have more than 255 parameters.".to_string(),
                    });
                }
                params.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' before {} body.", kind),
        )?;
        let body = self.block()?;

        Ok(Stmt::Function { name, params, body })
    }

    // varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
    fn var_declaration(&mut self) -> Result<Stmt<'a>, ParseError<'a>> {
        // 'var' 已经被消费
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let initializer = if self.match_token(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Var { name, initializer })
    }

    // statement      → exprStmt | ifStmt | printStmt | block ;
    fn statement(&mut self) -> Result<Stmt<'a>, ParseError<'a>> {
        if self.match_token(&[TokenType::For]) {
            self.for_statement()
        } else if self.match_token(&[TokenType::If]) {
            self.if_statement()
        } else if self.match_token(&[TokenType::Print]) {
            self.print_statement()
        } else if self.match_token(&[TokenType::Return]) {
            self.return_statement()
        } else if self.match_token(&[TokenType::While]) {
            self.while_statement()
        } else if self.match_token(&[TokenType::LeftBrace]) {
            Ok(Stmt::Block {
                statements: self.block()?,
            })
        } else {
            self.expression_statement()
        }
    }
    fn for_statement(&mut self) -> Result<Stmt<'a>, ParseError<'a>> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        // 1. Initializer
        let initializer = if self.match_token(&[TokenType::Semicolon]) {
            None
        } else if self.match_token(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        // 2. Condition
        let mut condition = if !self.check(TokenType::Semicolon) {
            self.expression()?
        } else {
            // 如果没有条件，就是一个无限循环
            Expr::Literal(LiteralValue::Boolean(true))
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        // 3. Increment
        let increment = if !self.check(TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        // 4. Body
        let mut body = self.statement()?;

        // 把它们组合起来
        // a. 如果有增量，它在循环体每次执行后运行
        if let Some(inc_expr) = increment {
            body = Stmt::Block {
                statements: vec![body, Stmt::Expression(inc_expr)],
            };
        }

        // b. 创建 while 循环
        let while_loop = Stmt::While {
            condition,
            body: Box::new(body),
        };

        // c. 如果有初始化器，它在整个循环之前运行
        if let Some(init_stmt) = initializer {
            Ok(Stmt::Block {
                statements: vec![init_stmt, while_loop],
            })
        } else {
            Ok(while_loop)
        }
    }

    fn while_statement(&mut self) -> Result<Stmt<'a>, ParseError<'a>> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = Box::new(self.statement()?);

        Ok(Stmt::While { condition, body })
    }

    fn return_statement(&mut self) -> Result<Stmt<'a>, ParseError<'a>> {
        let keyword = self.tokens[self.current - 1]; // the 'return' token
        let value = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return { keyword, value })
    }
    // ifStmt         → "if" "(" expression ")" statement ( "else" statement )? ;
    fn if_statement(&mut self) -> Result<Stmt<'a>, ParseError<'a>> {
        // 'if' 已经被消费
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_token(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }
    // block          → "{" declaration* "}" ;
    // 注意：block 返回 Vec<Stmt>，而不是 Result<Stmt>，因为它内部是一系列语句
    fn block(&mut self) -> Result<Vec<Stmt<'a>>, ParseError<'a>> {
        // '{' 已经被消费
        let mut statements = Vec::new();

        // 循环解析，直到遇到 '}' 或文件末尾
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
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
        self.assignment()
    }
    fn assignment(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        // 1. 解析一个可能是左值的表达式。这个表达式本身可以包含所有更高优先级的操作符
        //    因此，我们调用下一级（这里是 equality）
        let expr = self.logic_or()?;

        // 2. 检查后面是否跟着一个 '='
        if self.match_token(&[TokenType::Equal]) {
            let equals = self.tokens[self.current - 1]; // '=' token
            // 赋值是右结合的，所以递归调用 assignment
            let value = self.assignment()?;

            // 3. 检查左边的 'expr' 是否是一个合法的赋值目标
            match expr {
                Expr::Variable { name } => {
                    return Ok(Expr::Assign {
                        name,
                        value: Box::new(value),
                    });
                }
                Expr::Get { object, name } => {
                    // 如果左值是 a.b，我们创建一个 Set 表达式
                    return Ok(Expr::Set {
                        object,
                        name,
                        value: Box::new(value),
                    });
                }
                // 如果左值是其他东西，比如 1+2，就是无效的
                _ => {
                    return Err(ParseError {
                        token: equals,
                        message: "Invalid assignment target.".to_string(),
                    });
                }
            }
        }

        // 4. 如果没有 '='，就返回我们从 equality() 解析到的表达式
        Ok(expr)
    }

    fn logic_or(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        let mut expr = self.logic_and()?;

        while self.match_token(&[TokenType::Or]) {
            let operator = self.tokens[self.current - 1];
            let right = self.logic_and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    // 新增 logic_and
    fn logic_and(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        let mut expr = self.equality()?;

        while self.match_token(&[TokenType::And]) {
            let operator = self.tokens[self.current - 1];
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
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
            self.call()
        }
    }
    fn call(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(&[TokenType::Dot]) {
                // 新增: 处理 .property
                let name =
                    self.consume(TokenType::Identifier, "Expect property name after '.'.")?;
                expr = Expr::Get {
                    object: Box::new(expr),
                    name,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }
    fn finish_call(&mut self, callee: Expr<'a>) -> Result<Expr<'a>, ParseError<'a>> {
        let mut arguments = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(ParseError {
                        token: *self.peek().unwrap(),
                        message: "Can't have more than 255 arguments.".to_string(),
                    });
                }
                arguments.push(self.expression()?);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;

        Ok(Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    // primary        → NUMBER | STRING | "true" | "false" | "nil"
    //                | "(" expression ")" | IDENTIFIER ;
    fn primary(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        if let Some(token) = self.peek() {
            let token_copy = *token;
            match token_copy.token_type {
                TokenType::False
                | TokenType::True
                | TokenType::Nil
                | TokenType::Number
                | TokenType::String => {
                    self.advance();
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
                        _ => unreachable!(),
                    };
                }

                TokenType::Identifier => {
                    self.advance();
                    return Ok(Expr::Variable { name: token_copy });
                }

                TokenType::LeftParen => {
                    self.advance();
                    let expr = self.expression()?;
                    if self.match_token(&[TokenType::RightParen]) {
                        return Ok(Expr::Grouping(Box::new(expr)));
                    } else {
                        return Err(ParseError {
                            token: *self.peek().unwrap_or(&token_copy),
                            message: "Expected ')' after expression.".to_string(),
                        });
                    }
                }
                TokenType::This => {
                    self.advance();
                    return Ok(Expr::This {
                        keyword: token_copy,
                    });
                }
                // 新增: super
                TokenType::Super => {
                    self.advance(); // consume 'super'
                    let keyword = token_copy;
                    self.consume(TokenType::Dot, "Expect '.' after 'super'.")?;
                    let method =
                        self.consume(TokenType::Identifier, "Expect superclass method name.")?;
                    return Ok(Expr::Super { keyword, method });
                }
                _ => {
                    return Err(ParseError {
                        token: token_copy,
                        message: "Expected a primary expression.".to_string(),
                    });
                }
            }
        }

        Err(ParseError {
            token: *self.tokens.last().unwrap(),
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
