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
    Assign {
        name: Token<'a>,
        value: Box<Expr<'a>>,
    },
    Variable {
        name: Token<'a>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<'a> {
    // print 表达式;
    Print(Expr<'a>),
    // 表达式;
    Expression(Expr<'a>),
    Block {
        statements: Vec<Stmt<'a>>,
    },
    If {
        condition: Expr<'a>,
        then_branch: Box<Stmt<'a>>,
        else_branch: Option<Box<Stmt<'a>>>,
    },
    Var {
        name: Token<'a>,
        initializer: Option<Expr<'a>>,
    },
}

impl<'a> fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // 新增：赋值表达式
            Expr::Assign { name, value } => {
                // 格式化为 (= variable_name value)
                write!(f, "(= {} {})", name.lexeme, value)
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Grouping(expr) => {
                write!(f, "(group {})", expr)
            }
            Expr::Literal(value) => {
                write!(f, "{}", value)
            }
            Expr::Unary { operator, right } => {
                write!(f, "({} {})", operator.lexeme, right)
            }
            // 新增：变量引用
            Expr::Variable { name } => {
                // 直接打印变量名
                write!(f, "{}", name.lexeme)
            }
        }
    }
}

// 为 Stmt 实现 Display
impl<'a> fmt::Display for Stmt<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // 新增：块语句
            Stmt::Block { statements } => {
                // 格式化为 (block stmt1 stmt2 ...)
                write!(f, "(block")?;
                for stmt in statements {
                    // 递归打印块内的每个语句
                    write!(f, " {}", stmt)?;
                }
                write!(f, ")")
            }
            Stmt::Expression(expr) => {
                write!(f, "(; {})", expr)
            }
            // 新增：If 语句
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                // 格式化为 (if condition then_branch else_branch?)
                write!(f, "(if {} {}", condition, then_branch)?;
                if let Some(else_b) = else_branch {
                    write!(f, " {}", else_b)?;
                }
                write!(f, ")")
            }
            Stmt::Print(expr) => {
                write!(f, "(print {})", expr)
            }
            // 新增：变量声明
            Stmt::Var { name, initializer } => {
                // 格式化为 (var name initializer?)
                write!(f, "(var {}", name.lexeme)?;
                if let Some(init) = initializer {
                    write!(f, " {}", init)?;
                }
                write!(f, ")")
            }
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
