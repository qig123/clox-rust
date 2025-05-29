#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)] // <--- Add this attribute
pub enum TokenType {
    // Single-character tokens. 单字符词法
    LeftParen,  // TOKEN_LEFT_PAREN
    RightParen, // TOKEN_RIGHT_PAREN
    LeftBrace,  // TOKEN_LEFT_BRACE
    RightBrace, // TOKEN_RIGHT_BRACE
    Comma,      // TOKEN_COMMA
    Dot,        // TOKEN_DOT
    Minus,      // TOKEN_MINUS
    Plus,       // TOKEN_PLUS
    Semicolon,  // TOKEN_SEMICOLON
    Slash,      // TOKEN_SLASH
    Star,       // TOKEN_STAR

    // One or two character tokens. 一或两字符词法
    Bang,         // TOKEN_BANG
    BangEqual,    // TOKEN_BANG_EQUAL
    Equal,        // TOKEN_EQUAL
    EqualEqual,   // TOKEN_EQUAL_EQUAL
    Greater,      // TOKEN_GREATER
    GreaterEqual, // TOKEN_GREATER_EQUAL
    Less,         // TOKEN_LESS
    LessEqual,    // TOKEN_LESS_EQUAL

    // Literals. 字面量
    Identifier, // TOKEN_IDENTIFIER
    String,     // TOKEN_STRING
    Number,     // TOKEN_NUMBER

    // Keywords. 关键字
    And,    // TOKEN_AND
    Class,  // TOKEN_CLASS
    Else,   // TOKEN_ELSE
    False,  // TOKEN_FALSE
    For,    // TOKEN_FOR
    Fun,    // TOKEN_FUN
    If,     // TOKEN_IF
    Nil,    // TOKEN_NIL
    Or,     // TOKEN_OR
    Print,  // TOKEN_PRINT
    Return, // TOKEN_RETURN
    Super,  // TOKEN_SUPER
    This,   // TOKEN_THIS
    True,   // TOKEN_TRUE
    Var,    // TOKEN_VAR
    While,  // TOKEN_WHILE

    Error, // TOKEN_ERROR
    Eof,   // TOKEN_EOF
    Count, // <--- Add this as the last variant
}
