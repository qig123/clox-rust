use std::fmt;

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
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<'a> {
    // print 表达式;
    Print(Expr<'a>),
    // 表达式;
    Expression(Expr<'a>),
}

impl<'a> fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                // 为了清晰，我们可以创建一个辅助函数或宏
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Unary { operator, right } => {
                write!(f, "({} {})", operator.lexeme, right)
            }
            Expr::Grouping(expr) => {
                write!(f, "(group {})", expr)
            }
            Expr::Literal(value) => {
                // 直接委托给 LiteralValue 的 Display 实现
                write!(f, "{}", value)
            }
        }
    }
}

// 为 Stmt 实现 Display
impl<'a> fmt::Display for Stmt<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Print(expr) => write!(f, "(print {})", expr),
            Stmt::Expression(expr) => write!(f, "(; {})", expr),
        }
    }
}

// 为 LiteralValue 实现 Display
impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::Number(n) => write!(f, "{}", n),
            LiteralValue::String(s) => write!(f, "{}", s), // 可以选择是否加引号
            LiteralValue::Boolean(b) => write!(f, "{}", b),
            LiteralValue::Nil => write!(f, "nil"),
        }
    }
}
