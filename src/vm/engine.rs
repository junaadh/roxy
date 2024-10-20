use crate::{
    chunks::{Chunk, Opcode},
    value::Value,
    Res,
};

pub struct Vm<'src> {
    chunk: &'src Chunk,
    stack: Vec<Value>,
    ip: usize,
}

impl<'src> Vm<'src> {
    const STACK_SIZE: usize = u8::MAX as usize + 1;

    pub fn new(chunk: &'src Chunk) -> Self {
        Vm {
            chunk,
            stack: Vec::with_capacity(Self::STACK_SIZE),
            ip: 0,
        }
    }

    // misc
    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Empty stack")
    }

    pub fn interpret(&mut self) -> Res<()> {
        self.run()?;
        Ok(())
    }

    pub fn run(&mut self) -> Res<()> {
        loop {
            let op = &self.chunk.code[self.ip];

            #[cfg(feature = "trace")]
            {
                let disassembler = crate::chunks::Disassembler::new(&self.chunk, Some(&self.stack));
                disassembler.instruction(self.ip, op);
            }

            self.ip += 1;

            match *op {
                Opcode::Constant(constant) => {
                    let value = self.chunk.read_constant(constant);
                    self.push(value);
                }
                Opcode::Add => {
                    let (r, l) = (self.pop(), self.pop());
                    self.push(l + r);
                }
                Opcode::Subtract => {
                    let (r, l) = (self.pop(), self.pop());
                    self.push(l - r);
                }
                Opcode::Multiply => {
                    let (r, l) = (self.pop(), self.pop());
                    self.push(l * r);
                }
                Opcode::Divide => {
                    let (r, l) = (self.pop(), self.pop());
                    self.push(l / r);
                }
                Opcode::Negate => {
                    let value = self.pop();
                    self.push(-value);
                }
                Opcode::Return => {
                    println!("{}", self.pop());
                    return Ok(());
                }
            }
        }
    }
}
