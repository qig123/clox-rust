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
    Call {
        callee: Box<Expr<'a>>, // 被调用的表达式 (通常是变量)
        paren: Token<'a>,      // 右括号，用于错误报告
        arguments: Vec<Expr<'a>>,
    },
    // 新增: 属性获取 instance.name
    Get {
        object: Box<Expr<'a>>,
        name: Token<'a>,
    },
    Set {
        object: Box<Expr<'a>>,
        name: Token<'a>,
        value: Box<Expr<'a>>,
    },
    // 新增: super 关键字
    Super {
        keyword: Token<'a>, // 'super' token
        method: Token<'a>,  // 要调用的方法名
    },
    // 新增: this 关键字
    This {
        keyword: Token<'a>, // 'this' token
    },
    Logical {
        left: Box<Expr<'a>>,
        operator: Token<'a>, // 'and' or 'or'
        right: Box<Expr<'a>>,
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
    Function {
        name: Token<'a>,
        params: Vec<Token<'a>>,
        body: Vec<Stmt<'a>>, // 函数体是一个语句块
    },
    Return {
        keyword: Token<'a>, // return 关键字，用于错误报告
        value: Option<Expr<'a>>,
    },
    While {
        condition: Expr<'a>,
        body: Box<Stmt<'a>>,
    },
    Class {
        name: Token<'a>,
        // 可选的父类
        superclass: Option<Expr<'a>>, // 必须是 Expr::Variable
        methods: Vec<Stmt<'a>>,       // 必须是 Stmt::Function
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
            Expr::Call {
                callee, arguments, ..
            } => {
                write!(f, "(call {} (", callee)?;
                for (i, arg) in arguments.iter().enumerate() {
                    write!(f, "{}", arg)?;
                    if i < arguments.len() - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, "))")
            }
            // 新增 Get
            Expr::Get { object, name } => {
                write!(f, "(. {} {})", object, name.lexeme)
            }
            // 新增 Set
            Expr::Set {
                object,
                name,
                value,
            } => {
                write!(f, "(= (. {} {}) {})", object, name.lexeme, value)
            }
            // 新增 Super
            Expr::Super { method, .. } => {
                write!(f, "(super {})", method.lexeme)
            }
            // 新增 This
            Expr::This { .. } => {
                write!(f, "this")
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
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
            Stmt::Function { name, params, body } => {
                write!(f, "(fun {} (", name.lexeme)?;
                for (i, param) in params.iter().enumerate() {
                    write!(f, "{}", param.lexeme)?;
                    if i < params.len() - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, ") (block")?;
                for stmt in body {
                    write!(f, " {}", stmt)?;
                }
                write!(f, "))")
            }
            // 新增 Return
            Stmt::Return { value, .. } => {
                if let Some(val) = value {
                    write!(f, "(return {})", val)
                } else {
                    write!(f, "(return)")
                }
            }
            // 新增 While
            Stmt::While { condition, body } => {
                write!(f, "(while {} {})", condition, body)
            }
            Stmt::Class {
                name,
                superclass,
                methods,
            } => {
                write!(f, "(class {}", name.lexeme)?;
                if let Some(sc) = superclass {
                    write!(f, " < {}", sc)?;
                }
                for method in methods {
                    write!(f, " {}", method)?;
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
