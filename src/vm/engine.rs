use crate::{
    chunks::{Chunk, Opcode},
    compiler::{Parser, TokenType},
    object::ObjRef,
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

    // fn peek(&self, distance: usize) -> &Value {
    //     &self.stack[self.stack.len() - 1 - distance]
    // }

    pub fn interpret(&mut self, buf: &str) -> Res<()> {
        let parser = Parser::new(buf, self.chunk.borrow_mut());
        parser.compile();

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.run()));

        match result {
            Ok(_) => (),
            Err(e) => {
                self.runtime_error(
                    e.downcast_ref::<String>()
                        .unwrap_or(&"typed error occured.. Resetting stack.. Fallback".to_string())
                        .to_owned(),
                );
            }
        }
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
                Opcode::Nil => self.push(Value::Nil),
                Opcode::True => self.push(Value::Bool(true)),
                Opcode::False => self.push(Value::Bool(false)),
                Opcode::Equal => {
                    let (r, l) = (self.pop(), self.pop());
                    self.push(Value::Bool(l == r))
                }
                Opcode::Greater => {
                    let (r, l) = (self.pop(), self.pop());
                    self.push(Value::Bool(l > r))
                }
                Opcode::Less => {
                    let (r, l) = (self.pop(), self.pop());
                    self.push(Value::Bool(l < r))
                }
                Opcode::Add => {
                    let val = match (self.pop(), self.pop()) {
                        (Value::String(r), Value::String(l)) => {
                            let str = format!("{}{}", l, r);
                            Value::String(ObjRef::new(Box::into_raw(Box::new(str))))
                        }
                        (r, l) => l + r,
                    };
                    self.push(val);
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
                Opcode::Not => {
                    let value = self.pop();
                    self.push(!value);
                }
                Opcode::Return => {
                    if !self.stack.is_empty() {
                        println!("{}", self.pop());
                    }
                    return Ok(());
                }
            }
        }
    }

    // error
    fn runtime_error(&mut self, s: String) {
        println!("{s}");

        let instruction = self
            .ip
            .checked_sub(self.chunk.code.len().checked_sub(1).unwrap_or_default())
            .unwrap_or_default();
        let line = self.chunk.lines[instruction];

        println!("[line {}] in script", line);

        self.stack.clear();
    }
}
