#[derive(Debug)]
pub enum Opcode {
    Constant(u8),
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}
