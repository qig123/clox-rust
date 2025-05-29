use crate::token_type::TokenType;

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenType,
    pub lexeme: &'a str,
    pub line: usize,
}
impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }
        if self.peek().is_ascii_digit() {
            return self.number();
        }
        let c = self.advance();
        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            ';' => self.make_token(TokenType::Semicolon),
            '*' => self.make_token(TokenType::Star),
            '/' => self.make_token(TokenType::Slash),
            '!' => {
                if self.match_char('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            '"' => self.string(),
            _ if Self::is_alpha(c) => return self.identifier(),
            _ => return self.error_token("Unexpected character."), // Handle other cases as needed
        }
    }
    fn number(&mut self) -> Token<'a> {
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            self.advance();
        }
        // 可能有小数点
        if !self.is_at_end() && self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // 跳过小数点
            while !self.is_at_end() && self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let lexeme = &self.source[self.start..self.current];
        self.make_token_with_lexeme(TokenType::Number, lexeme)
    }
    fn string(&mut self) -> Token<'a> {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }
        // 现在我们已经到达了结束的引号
        self.advance(); // 跳过结束的引号
        let lexeme = &self.source[self.start + 1..self.current - 1]; // 去掉引号
        self.make_token_with_lexeme(TokenType::String, lexeme)
    }

    fn advance(&mut self) -> char {
        //这里能确保不会返回 None，因为 is_at_end 已经检查过了
        let c = self.source[self.current..].chars().next().unwrap();
        self.current += c.len_utf8();
        c
    }
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let c = self.source[self.current..].chars().next().unwrap();
        if c == expected {
            self.current += c.len_utf8();
            true
        } else {
            false
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        // Single-line comment
                        while !self.is_at_end() && self.peek() != '\n' {
                            self.advance();
                        }
                    } else {
                        break; // Not a comment, break out of the loop
                    }
                }
                _ => break,
            }
        }
    }
    fn peek(&self) -> char {
        self.source[self.current..].chars().next().unwrap()
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0'; // Return null character if at the end
        }
        self.source[self.current + 1..].chars().next().unwrap()
    }

    fn identifier(&mut self) -> Token<'a> {
        while !self.is_at_end() && (Self::is_alpha(self.peek()) || self.peek().is_ascii_digit()) {
            self.advance();
        }
        let lexeme = &self.source[self.start..self.current];
        let kind = match lexeme {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ if Self::is_alpha(lexeme.chars().next().unwrap()) => TokenType::Identifier, // 其他标识符
            _ => return self.error_token("Unexpected identifier."),
        };
        self.make_token_with_lexeme(kind, lexeme)
    }
    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }
    fn make_token(&self, kind: TokenType) -> Token<'a> {
        Token {
            kind,
            lexeme: &self.source[self.start..self.current],
            line: self.line,
        }
    }

    fn make_token_with_lexeme(&self, kind: TokenType, lexeme: &'a str) -> Token<'a> {
        Token {
            kind,
            lexeme: lexeme,
            line: self.line,
        }
    }
    fn error_token(&self, message: &'a str) -> Token<'a> {
        Token {
            kind: TokenType::Error,
            lexeme: message,
            line: self.line,
        }
    }
}
