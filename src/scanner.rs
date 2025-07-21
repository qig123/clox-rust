// src/scanner.rs

use std::collections::HashMap;

use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,                               // 当前正在扫描的词素的起始位置
    current: usize,                             // 当前正在处理的字符的位置
    line: usize,                                // 当前行号
    keywords: HashMap<&'static str, TokenType>, // 新增：关键字哈希图
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut keywords = HashMap::new();

        // 填充关键字
        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    /// 扫描并返回下一个 token
    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespace(); // 关键：在扫描下一个token前，跳过所有无意义的字符

        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();
        if c.is_ascii_alphabetic() || c == '_' {
            return self.identifier();
        }
        if c.is_ascii_digit() {
            return self.number();
        }
        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.make_token(token_type)
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.make_token(token_type)
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.make_token(token_type)
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.make_token(token_type)
            }
            // 检查是否是字符串的开头
            '"' => self.string(),

            _ => self.error_token("Unexpected character."),
        }
    }

    /// 跳过所有空白字符、换行符和注释
    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                // 普通空白
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                // 换行
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                // 注释，它会消耗掉到行尾的所有内容
                '/' if self.peek_next() == '/' => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                _ => return, // 遇到非空白字符，结束跳过
            }
        }
    }
    fn identifier(&mut self) -> Token<'a> {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let token_type = self.identifier_type();
        self.make_token(token_type)
    }

    /// 新增：确定标识符的具体类型（关键字 vs 普通标识符）
    fn identifier_type(&self) -> TokenType {
        let text = &self.source[self.start..self.current];
        // get() 返回 Option<&TokenType>，我们用 cloned() 和 unwrap_or() 来处理
        // 如果在 keywords 中找到，就返回克隆出的 TokenType
        // 如果没找到 (None)，就返回默认值 TokenType::Identifier
        self.keywords
            .get(text)
            .cloned()
            .unwrap_or(TokenType::Identifier)
    }
    /// 扫描一个字符串字面量
    fn string(&mut self) -> Token<'a> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        // 消耗闭合的引号
        self.advance();
        self.make_token(TokenType::String)
    }

    /// 扫描一个数字字面量
    fn number(&mut self) -> Token<'a> {
        // 消耗整数部分
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // 检查小数部分
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // 消耗 '.'
            self.advance();

            // 消耗小数部分的数字
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        // 匹配成功，消耗字符
        self.current += 1;
        true
    }

    /// 查看当前字符，但不消耗它
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0'; // 用空字符表示文件结束
        }
        self.source.chars().nth(self.current).unwrap()
    }

    /// 查看下一个字符
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    /// 消耗当前字符并返回它，同时移动 current 指针
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    /// 检查是否已经处理完所有字符
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// 基于当前的 start 和 current 指针创建一个 Token
    fn make_token(&self, token_type: TokenType) -> Token<'a> {
        let lexeme = &self.source[self.start..self.current];
        Token::new(token_type, lexeme, self.line)
    }

    /// 创建一个错误 Token
    fn error_token(&self, message: &'static str) -> Token<'a> {
        Token::new(TokenType::Error, message, self.line)
    }
}
