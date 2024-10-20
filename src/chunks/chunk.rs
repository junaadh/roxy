use crate::value::Value;

use super::Opcode;

#[derive(Debug, Default)]
pub struct Chunk {
    pub code: Vec<Opcode>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write(&mut self, op: Opcode, line: usize) {
        self.code.push(op);
        self.lines.push(line);
    }

    pub fn add_constant<T>(&mut self, value: T) -> u8
    where
        T: Into<Value>,
    {
        self.constants.push(value.into());
        self.constants.len() as u8 - 1
    }

    pub fn read_constant(&self, constant: u8) -> Value {
        self.constants[constant as usize]
    }
}
