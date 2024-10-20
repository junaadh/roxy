pub mod chunks;
pub mod error;
pub mod value;
pub mod vm;

pub type Res<T> = Result<T, error::RxError>;
