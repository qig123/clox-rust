use std::{fmt, ops};

#[derive(Debug, Clone)]
pub enum Value {
    // Bool(bool),
    // Nil,
    Number(f64),
}
impl Value {
    pub fn from_number(n: f64) -> Self {
        Value::Number(n)
    }
}
impl ops::Neg for Value {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Value::Number(n) => Value::Number(-n),
            // 对非数字取负是一个运行时错误！
            // _ => panic!("Operand must be a number."), // 我们马上会改进这个 panic
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Value::Bool(b) => write!(f, "{}", b),
            // Value::Nil => write!(f, "nil"),
            Value::Number(n) => write!(f, "{}", n),
        }
    }
}
