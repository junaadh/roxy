use crate::{
    chunks::{Chunk, Opcode},
    compiler::{Cursor, Parser},
    error::{Runtime, RxError},
    value::Value,
    Res,
};
use std::{
    borrow::BorrowMut,
    fs,
    io::{self, BufRead, Read, Write},
};

pub struct Vm<'src> {
    chunk: &'src mut Chunk,
    stack: Vec<Value>,
    ip: usize,
}

impl<'src> Vm<'src> {
    const STACK_SIZE: usize = u8::MAX as usize + 1;

    pub fn new(chunk: &'src mut Chunk) -> Self {
        Vm {
            chunk,
            stack: Vec::with_capacity(Self::STACK_SIZE),
            ip: 0,
        }
    }

    // runners
    pub fn run_file(&mut self, file_name: &str) -> Res<()> {
        let mut file = fs::File::open(file_name)?;
        let mut buf = String::new();

        file.read_to_string(&mut buf)?;
        self.interpret(&buf)?;
        Ok(())
    }

    pub fn run_repl(&mut self) -> Res<()> {
        let mut input = String::new();
        loop {
            input.clear();
            print!("roxy:> ");
            let _ = io::stdout().lock().flush();
            if let Err(err) = io::stdin().lock().read_line(&mut input) {
                eprintln!("RoxyUnwind: {err}");
                continue;
            }

            let check = input.trim();
            if check == "q" {
                break;
            } else if check.is_empty() {
                continue;
            }

            self.interpret(check)?;
            // match self.execute(check) {
            //     Ok(_) => continue,
            //     Err(e) => {
            //         eprintln!("RoxyUnwind: {e}");
            //         continue;
            //     }
            // }
        }
        println!("Exiting...");
        Ok(())
    }

    // misc
    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Empty stack")
    }

    pub fn interpret(&mut self, buf: &str) -> Res<()> {
        let parser = Parser::new(buf, self.chunk.borrow_mut());
        parser.compile();

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
