use std::{fmt, ops, rc::Rc};

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
    String(Rc<String>), // 新增：使用 Rc<String> 来表示字符串
}

impl Value {
    pub fn from_number(n: f64) -> Self {
        Value::Number(n)
    }

    // 辅助函数，判断值是否为 "falsey"
    // 在 Lox 中，只有 nil 和 false 是 falsey
    pub fn is_falsey(&self) -> bool {
        matches!(self, Value::Nil | Value::Bool(false))
    }
}

impl ops::Neg for Value {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Value::Number(n) => Value::Number(-n),
            _ => panic!("Operand must be a number."),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s), // 新增：打印字符串
        }
    }
}

// 为了能用 == 比较 Value
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            _ => false, // 不同类型的值不相等
        }
    }
}
