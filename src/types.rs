#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    Bool(bool),
    Int(usize),
    Float(f64),
    String(String),
    Nil,
}
