use std::{
    fmt, io,
    num::{ParseFloatError, ParseIntError},
};

use crate::ast::Token;

#[derive(Debug)]
pub enum RxError {
    Io(io::Error),
    Lex(Error),
    Parse(Error),
    NumConversion(ParseIntError),
    FloatConversion(ParseFloatError),
}

impl From<io::Error> for RxError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ParseFloatError> for RxError {
    fn from(value: ParseFloatError) -> Self {
        Self::FloatConversion(value)
    }
}

impl From<ParseIntError> for RxError {
    fn from(value: ParseIntError) -> Self {
        Self::NumConversion(value)
    }
}

impl fmt::Display for RxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{}", e),
            Self::Lex(e) => write!(f, "Lexical: {}", e),
            Self::Parse(e) => write!(f, "Parse: {}", e),
            Self::NumConversion(e) => write!(f, "{}", e),
            Self::FloatConversion(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    token: Token,
    msg: String,
}

impl Error {
    pub fn new(token: Token, msg: &str) -> Self {
        Self {
            token,
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.token.kind {
            crate::ast::TokenType::Eof => {
                write!(f, "[line {}] Error at end: {}", self.token.span.2, self.msg)
            }
            _ => write!(
                f,
                "[line {}] Error at Token({}): {}",
                self.token.span.2, self.token.kind, self.msg
            ),
        }
    }
}
