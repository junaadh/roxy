#[cfg(feature = "trace")]
use crate::{
    chunks::{Chunk, Opcode},
    value::Value,
};

#[cfg(feature = "trace")]
pub struct Disassembler<'src> {
    chunk: &'src Chunk,
    stack: Option<&'src Vec<Value>>,
}

#[cfg(feature = "trace")]
impl<'src> Disassembler<'src> {
    pub fn new(chunk: &'src Chunk, stack: Option<&'src Vec<Value>>) -> Self {
        Self { chunk, stack }
    }

    pub fn disassemble(&self, name: &str) {
        println!("== BEGIN {} ==", name);

        for (idx, op) in self.chunk.code.iter().enumerate() {
            self.instruction(idx, op)
        }

        println!("== END   {} ==\n", name);
    }

    fn stack(&self) {
        if let Some(stack) = self.stack {
            print!("S: ");
            if !stack.is_empty() {
                for value in stack {
                    print!("[ {} ]", value);
                }
                println!();
            } else {
                println!("[ ]");
            }
        }
    }

    pub fn instruction(&self, offset: usize, opcode: &Opcode) {
        self.stack();
        print!("{:04} ", offset);
        let line = self.chunk.lines[offset];
        if offset > 0 && line == self.chunk.lines[offset - 1] {
            print!("   | ")
        } else {
            print!("{:>4} ", line)
        }

        match opcode {
            Opcode::Constant(c) => self.const_op("OP_Constant", *c),
            Opcode::Add => self.simple_op("OP_Add"),
            Opcode::Subtract => self.simple_op("OP_Subtract"),
            Opcode::Multiply => self.simple_op("OP_Multiply"),
            Opcode::Divide => self.simple_op("OP_Divide"),
            Opcode::Negate => self.simple_op("OP_Negate"),
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
