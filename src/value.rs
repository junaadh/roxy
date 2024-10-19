use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Float(f64),
    Int(i64),
    Bool(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float(fl) => write!(f, "{fl}"),
            Self::Int(i) => write!(f, "{i}"),
            Self::Bool(b) => write!(f, "{b}"),
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}
