use super::Opcode;

#[derive(Debug, Default)]
pub struct Chunk {
    chunk: Vec<Opcode>,
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write(&mut self, op: Opcode) {
        self.chunk.push(op)
    }
}
