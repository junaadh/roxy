pub mod ast;
pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod types;

pub type Res<T> = Result<T, error::RxError>;
