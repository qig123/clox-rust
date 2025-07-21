// src/ast.rs

// 为了简单，我们先只定义表达式 AST
// Expr 是一个枚举，代表所有可能的表达式类型
#[derive(Debug)]
pub enum Expr {
    // 字面量表达式，比如 123, "hello", true, nil
    Literal(LiteralValue),
    // ... 后面会添加 Binary, Unary, Grouping 等
}

// 定义字面量的值
#[derive(Debug)]
pub enum LiteralValue {
    Number(f64),
    // String(String),
    // Bool(bool),
    // Nil,
}
