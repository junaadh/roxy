use std::{
    fmt,
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

use crate::{
    error::{Compile, RxError},
    object::ObjRef,
};

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Float(f64),
    Int(i64),
    Bool(bool),
    Nil,
    String(ObjRef<String>),
}

impl Value {
    fn get_ty(&self) -> &str {
        match self {
            Self::Float(_) => "float64",
            Self::Int(_) => "int64",
            Self::Bool(_) => "bool",
            Self::Nil => "nil",
            Self::String(_) => "string",
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Float(l), Self::Float(r)) => l == r,
            (Self::Int(l), Self::Int(r)) => l == r,
            (Self::Bool(l), Self::Bool(r)) => l == r,
            (Self::Nil, Self::Nil) => true,
            (Self::Int(l), Self::Float(r)) => &(*l as f64) == r,
            (Self::Float(l), Self::Int(r)) => l == &(*r as f64),
            (Self::String(s1), Self::String(s2)) => unsafe { *s1.value == *s2.value },
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Int(l), Self::Int(r)) => l.partial_cmp(r),
            (Self::Float(l), Self::Float(r)) => l.partial_cmp(r),
            (Self::Bool(l), Self::Bool(r)) => l.partial_cmp(r),
            (Self::Float(l), Self::Int(r)) => l.partial_cmp(&(*r as f64)),
            (Self::Int(l), Self::Float(r)) => (*l as f64).partial_cmp(r),
            (Self::Nil, Self::Nil) => Some(std::cmp::Ordering::Equal),
            // (Self::String(s1), Self::String(s2)) => s1.partial_cmp(s2),
            _ => None,
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Self::Float(l), Self::Float(r)) => Self::Float(l + r),
            (Self::Int(l), Self::Int(r)) => Self::Int(l + r),
            (Self::Float(l), Self::Int(r)) => Self::Float(l + *r as f64),
            (Self::Int(l), Self::Float(r)) => Self::Float(*l as f64 + r),
            _ => panic!(
                "TypeError: Unable to add lhs: {} with rhs: {}",
                self.get_ty(),
                rhs.get_ty()
            ),
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Self::Float(l), Self::Float(r)) => Self::Float(l - r),
            (Self::Int(l), Self::Int(r)) => Self::Int(l - r),
            (Self::Float(l), Self::Int(r)) => Self::Float(l - *r as f64),
            (Self::Int(l), Self::Float(r)) => Self::Float(*l as f64 - r),
            _ => panic!(
                "TypeError: Unable to subtract lhs: {} with rhs: {}",
                self.get_ty(),
                rhs.get_ty()
            ),
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Self::Float(l), Self::Float(r)) => Self::Float(l * r),
            (Self::Int(l), Self::Int(r)) => Self::Int(l * r),
            (Self::Float(l), Self::Int(r)) => Self::Float(l * *r as f64),
            (Self::Int(l), Self::Float(r)) => Self::Float(*l as f64 * r),
            _ => panic!(
                "TypeError: Unable to multiply lhs: {} with rhs: {}",
                self.get_ty(),
                rhs.get_ty()
            ),
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Self::Float(l), Self::Float(r)) => Self::Float(l / r),
            (Self::Int(l), Self::Int(r)) => Self::Int(l / r),
            (Self::Float(l), Self::Int(r)) => Self::Float(l / *r as f64),
            (Self::Int(l), Self::Float(r)) => Self::Float(*l as f64 / r),
            _ => panic!(
                "TypeError: Unable to divide lhs: {} with rhs: {}",
                self.get_ty(),
                rhs.get_ty()
            ),
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Float(f) => Self::Float(-f),
            Self::Int(i) => Self::Int(-i),
            _ => panic!("TypeError: Unable to negate {}", self.get_ty()),
        }
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Int(i) => Self::Int(-i),
            Self::Bool(b) => Self::Bool(!b),
            Self::Nil => Self::Bool(false),
            _ => panic!("TypeError: Unable to use logical not on {}", self.get_ty()),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Float(fl) => write!(f, "{fl}"),
            Self::Int(i) => write!(f, "{i}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Nil => write!(f, "nil"),
            Self::String(v) => write!(f, "{}", v),
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl TryFrom<Value> for f64 {
    type Error = RxError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(f) => Ok(f),
            _ => Err(RxError::new(Compile::new(&format!(
                "TypeError: Unable to convert {} to an f64",
                value.get_ty()
            )))),
        }
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int(value)
    }
}

impl TryFrom<Value> for i64 {
    type Error = RxError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(i) => Ok(i),
            _ => Err(RxError::new(Compile::new(&format!(
                "TypeError: Unable to convert {} to an i64",
                value.get_ty()
            )))),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<Value> for bool {
    fn from(value: Value) -> Self {
        match value {
            Value::Bool(b) => b,
            Value::Nil => false,
            _ => true,
        }
    }
}
