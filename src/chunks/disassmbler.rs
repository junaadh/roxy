use super::{Chunk, Opcode};

pub struct Disassmbler<'src> {
    chunk: &'src Chunk,
}

impl<'src> Disassmbler<'src> {
    pub fn new(chunk: &'src Chunk) -> Self {
        Self { chunk }
    }

    pub fn disassemble(&self, name: &str) {
        println!("== BEGIN {} ==", name);

        for (idx, op) in self.chunk.code.iter().enumerate() {
            self.instruction(idx, op)
        }

        println!("== END   {} ==", name);
    }

    fn instruction(&self, offset: usize, opcode: &Opcode) {
        print!("{:04} ", offset);
        let line = self.chunk.lines[offset];
        if offset > 0 && line == self.chunk.lines[offset - 1] {
            print!("   | ")
        } else {
            print!("{:>4} ", line)
        }

        match opcode {
            Opcode::Constant(c) => self.const_op("OP_Constant", *c),
            Opcode::Return => self.simple_op("OP_Return"),
        }
    }

    fn simple_op(&self, name: &str) {
        println!("{name}");
    }

    fn const_op(&self, name: &str, idx: u8) {
        let value = self.chunk.constants[idx as usize];
        println!("{:<16} {:4}", name, value)
    }
}
