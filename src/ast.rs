// src/ast.rs (部分)
use crate::token::Token;

// LiteralValue 也要扩展
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
    // Binary: 用于所有二元操作符 (+, -, *, /, ==, !=, <, etc.)
    Binary {
        left: Box<Expr<'a>>,
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    // Unary: 用于 ! 和 -
    Unary {
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    // Literal: 数字、字符串、布尔值、nil
    Literal(LiteralValue),
    // Grouping: 用于括号 (...)
    Grouping(Box<Expr<'a>>),
}
