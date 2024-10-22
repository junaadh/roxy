pub mod chunks;
pub mod compiler;
pub mod error;
pub mod object;
pub mod value;
pub mod vm;

pub type Res<T> = Result<T, error::RxError>;
