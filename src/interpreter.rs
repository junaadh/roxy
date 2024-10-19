use std::{
    fs,
    io::{self, BufRead, Read, Write},
};

use crate::{ast::AstPrinter, lexer::Cursor, parser::Parser, Res};

pub struct Interpreter;

impl Interpreter {
    pub fn run_file(&self, src: &str) -> Res<()> {
        let mut file = fs::File::open(src)?;
        let mut buf = String::new();

        file.read_to_string(&mut buf)?;
        self.execute(buf.trim())?;
        Ok(())
    }

    pub fn run_repl(&self) -> Res<()> {
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

            match self.execute(check) {
                Ok(_) => continue,
                Err(e) => {
                    eprintln!("RoxyUnwind: {e}");
                    continue;
                }
            }
        }
        println!("Exiting...");
        Ok(())
    }

    fn execute(&self, contents: &str) -> Res<()> {
        let lexer = Cursor::new(contents).tokenize().collect::<Vec<_>>();
        let parser = Parser::new(lexer).parse();

        if let Some(expr) = parser {
            let ast = AstPrinter;
            println!("{}", ast.print(&expr));
        }

        Ok(())
    }
}
