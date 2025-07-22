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

        if let Some(c) = self.advance() {
            if c.is_ascii_alphabetic() || c == '_' {
                return self.identifier();
            }
            if c.is_ascii_digit() {
                return self.number();
            }

            // match 语句现在直接作用于字符 'c'
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
                '"' => self.string(),

                _ => self.error_token("Unexpected character."),
            }
        } else {
            // 如果 advance() 返回 None，这意味着我们在 is_at_end() 检查之后到达了文件末尾
            // 理论上，由于 is_at_end() 的检查，这个分支不应该被触及，但为了代码的完整性，
            // 返回 EOF token 是最安全的选择。
            self.make_token(TokenType::Eof)
        }
    }

    /// 跳过所有空白字符、换行符和注释
    // in impl<'a> Scanner<'a>

    fn skip_whitespace(&mut self) {
        loop {
            // peek() 现在返回 Option<char>，所以我们在 match 中处理 Some(char)
            match self.peek() {
                // 匹配到 Some 包裹的空白字符
                Some(' ' | '\r' | '\t') => {
                    self.advance(); // 安全地消耗掉
                }
                // 匹配到 Some 包裹的换行符
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                }
                // 匹配到 Some 包裹的斜杠
                Some('/') => {
                    // peek_next() 也返回 Option<char>
                    if self.peek_next() == Some('/') {
                        // 确认是注释，消耗掉到行尾的所有内容
                        // 循环条件也需要更新
                        while self.peek() != Some('\n') && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        // 如果 '/' 后面不是另一个 '/'，那它不是注释，
                        // 而是除法操作符。我们应该停止跳过空白。
                        return;
                    }
                }
                // 匹配到 None (文件末尾) 或任何其他非空白字符
                _ => {
                    // 停止跳过空白
                    return;
                }
            }
        }
    }
    // in impl<'a> Scanner<'a>

    fn identifier(&mut self) -> Token<'a> {
        // 只要 peek() 返回 Some(c)，并且 c 符合条件，就继续循环
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.advance(); // 消耗这个字符
            } else {
                // 如果字符不符合条件，就跳出循环
                break;
            }
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
    // in impl<'a> Scanner<'a>

    fn string(&mut self) -> Token<'a> {
        // 循环直到遇到 " 或文件末尾
        while self.peek() != Some('"') && !self.is_at_end() {
            // 如果遇到换行符，增加行号
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            // 如果是因为文件末尾而跳出循环，说明字符串未闭合
            return self.error_token("Unterminated string.");
        }

        // 如果是因为遇到了 '"' 而跳出循环，消耗掉这个闭合的引号
        self.advance();

        self.make_token(TokenType::String)
    }

    /// 扫描一个数字字面量
    // in impl<'a> Scanner<'a>

    fn number(&mut self) -> Token<'a> {
        // 消耗整数部分
        // 使用 map_or 来保持简洁
        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
            self.advance();
        }

        // 检查小数部分
        if self.peek() == Some('.') && self.peek_next().map_or(false, |c| c.is_ascii_digit()) {
            // 消耗 '.'
            self.advance();

            // 消耗小数部分的数字
            while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }
    /// 检查并匹配下一个字符。如果匹配，就安全地消费它。
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        // 从当前字节位置开始的子字符串
        let remaining = &self.source[self.current..];
        if remaining.starts_with(expected) {
            // 如果匹配，按字符的字节长度前进
            self.current += expected.len_utf8();
            true
        } else {
            false
        }
    }

    /// 查看当前字符，但不消耗它。返回 Option<char>。
    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        // 安全地从当前字节位置获取第一个字符
        self.source[self.current..].chars().next()
    }

    /// 查看下一个字符。返回 Option<char>。
    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        // 创建一个字符迭代器，跳过第一个，取第二个
        let mut chars = self.source[self.current..].chars();
        chars.next(); // 消耗第一个
        chars.next() // 返回第二个（如果存在）
    }
    /// 消耗当前字符并返回它，同时安全地移动 current 字节指针。
    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?; // 使用新的 peek 获取字符
        // 根据字符的 UTF-8 字节长度来前进 current
        self.current += ch.len_utf8();
        Some(ch)
    }
    /// 检查是否已经处理完所有字节。
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn make_token(&self, token_type: TokenType) -> Token<'a> {
        let lexeme = &self.source[self.start..self.current];
        Token::new(token_type, lexeme, self.line)
    }

    /// 创建一个错误 Token。
    fn error_token(&self, message: &'static str) -> Token<'a> {
        Token::new(TokenType::Error, message, self.line)
    }
}
