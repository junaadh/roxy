use std::{
    fmt,
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

use crate::{
    error::{RxError, TypeError},
    Res,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    Bool(bool),
    Int(isize),
    Float(f64),
    String(String),
    Nil,
}

impl Object {
    // impl proper typing system??
    pub fn type_of(&self) -> &str {
        match self {
            Self::Bool(_) => "bool",
            Self::Int(_) => "int64",
            Self::Float(_) => "float64",
            Self::String(_) => "string",
            Self::Nil => "nil",
        }
    }
}

impl Add for Object {
    type Output = Res<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => Ok(Self::Int(l + r)),
            (Self::Int(l), Self::Float(r)) => Ok(Self::Float(l as f64 + r)),
            (Self::Float(l), Self::Int(r)) => Ok(Self::Float(l + r as f64)),
            (Self::Float(l), Self::Float(r)) => Ok(Self::Float(l + r)),
            (Self::String(l), Self::String(r)) => Ok(Self::String(format!("{l}{r}"))),
            (Self::String(l), Self::Int(r)) => Ok(Self::String(format!("{l}{r}"))),
            (Self::String(l), Self::Float(r)) => Ok(Self::String(format!("{l}{r}"))),
            (Self::Float(l), Self::String(r)) => Ok(Self::String(format!("{l}{r}"))),
            (Self::Int(l), Self::String(r)) => Ok(Self::String(format!("{l}{r}"))),
            (l, r) => Err(RxError::Ty(TypeError::new(&format!(
                "unable to add lhs: {} and rhs: {}",
                l.type_of(),
                r.type_of()
            )))),
        }
    }
}

impl Sub for Object {
    type Output = Res<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => Ok(Self::Int(l - r)),
            (Self::Int(l), Self::Float(r)) => Ok(Self::Float(l as f64 - r)),
            (Self::Float(l), Self::Int(r)) => Ok(Self::Float(l - r as f64)),
            (Self::Float(l), Self::Float(r)) => Ok(Self::Float(l - r)),
            (l, r) => Err(RxError::Ty(TypeError::new(&format!(
                "unable to subtract lhs: {} and rhs: {}",
                l.type_of(),
                r.type_of()
            )))),
        }
    }
}

impl Mul for Object {
    type Output = Res<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => Ok(Self::Int(l * r)),
            (Self::Int(l), Self::Float(r)) => Ok(Self::Float(l as f64 * r)),
            (Self::Float(l), Self::Int(r)) => Ok(Self::Float(l * r as f64)),
            (Self::Float(l), Self::Float(r)) => Ok(Self::Float(l * r)),
            (l, r) => Err(RxError::Ty(TypeError::new(&format!(
                "unable to multiply lhs: {} and rhs: {}",
                l.type_of(),
                r.type_of()
            )))),
        }
    }
}

impl Div for Object {
    type Output = Res<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => Ok(Self::Int(l / r)),
            (Self::Int(l), Self::Float(r)) => Ok(Self::Float(l as f64 / r)),
            (Self::Float(l), Self::Int(r)) => Ok(Self::Float(l / r as f64)),
            (Self::Float(l), Self::Float(r)) => Ok(Self::Float(l / r)),
            (l, r) => Err(RxError::Ty(TypeError::new(&format!(
                "unable to divide lhs: {} and rhs: {}",
                l.type_of(),
                r.type_of()
            )))),
        }
    }
}

impl Neg for Object {
    type Output = Res<Self>;

    fn neg(self) -> Self::Output {
        match self {
            Self::Int(i) => Ok(Self::Int(-i)),
            Self::Float(i) => Ok(Self::Float(-i)),
            _ => Err(RxError::Ty(TypeError::new(&format!(
                "Unable to negate object of type: {}",
                self.type_of()
            )))),
        }
    }
}

impl Not for Object {
    type Output = Res<Self>;

    fn not(self) -> Self::Output {
        match self {
            Self::Bool(b) => Ok(Self::Bool(!b)),
            Self::Int(i) => Ok(Self::Int(!i)),
            Self::Nil => Ok(Self::Bool(false)),
            _ => Err(RxError::Ty(TypeError::new(&format!(
                "Unable to use logical not on object of type: {}",
                self.type_of()
            )))),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(true) => write!(f, "true"),
            Self::Bool(false) => write!(f, "false"),
            Self::Int(val) => write!(f, "{val}"),
            Self::Float(val) => write!(f, "{val}"),
            Self::String(val) => write!(f, "\"{val}\""),
            Self::Nil => write!(f, "nil"),
        }
    }
}
