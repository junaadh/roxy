use std::{fmt, io};

#[derive(Debug)]
pub enum RxError {
    Compile(Compile),
    Runtime(Runtime),
}

impl RxError {
    pub fn new<T>(err: T) -> Self
    where
        T: Into<Self>,
    {
        err.into()
    }
}

#[derive(Debug)]
pub struct Compile {
    msg: String,
}

impl Compile {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct Runtime {
    msg: String,
}

impl Runtime {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
        }
    }
}

impl From<Compile> for RxError {
    fn from(value: Compile) -> Self {
        RxError::Compile(value)
    }
}

impl From<Runtime> for RxError {
    fn from(value: Runtime) -> Self {
        RxError::Runtime(value)
    }
}

impl fmt::Display for Compile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl fmt::Display for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl fmt::Display for RxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compile(e) => write!(f, "compile error: {}", e),
            Self::Runtime(e) => write!(f, "runtime error: {}", e),
        }
    }
}

impl From<io::Error> for RxError {
    fn from(value: io::Error) -> Self {
        RxError::Compile(Compile {
            msg: value.to_string(),
        })
    }
}
