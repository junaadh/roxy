#[derive(Debug)]
pub enum Opcode {
    Constant(u8),
    Nil,
    True,
    False,
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Not,
    Return,
}
