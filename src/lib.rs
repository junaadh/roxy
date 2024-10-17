pub mod ast;
pub mod error;
pub mod interpreter;
pub mod lexer;

pub type Res<T> = Result<T, error::RxError>;
