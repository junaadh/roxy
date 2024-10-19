use std::{
    fs,
    io::{self, BufRead, Read, Write},
};

use crate::{
    ast::{AstPrinter, Expr, ExprVisitor, TokenType},
    lexer::Cursor,
    parser::Parser,
    types::Object,
    Res,
};

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
            self.interpret(&expr);
        }

        Ok(())
    }

    fn evaluate(&self, expr: &Expr) -> Res<Object> {
        expr.accept(self)
    }

    pub fn interpret(&self, expr: &Expr) {
        match self.evaluate(expr) {
            Ok(value) => println!("{}", value),
            Err(err) => eprintln!("{}", err),
        }
    }

    // visitor
}

impl ExprVisitor<Res<Object>> for Interpreter {
    fn visit_binary(&self, expr: &crate::ast::Binary) -> Res<Object> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.kind {
            TokenType::BangEqual => Ok(Object::Bool(left != right)),
            TokenType::EqualEqual => Ok(Object::Bool(left == right)),
            TokenType::Greater => Ok(Object::Bool(left > right)),
            TokenType::GreaterEqual => Ok(Object::Bool(left >= right)),
            TokenType::Less => Ok(Object::Bool(left < right)),
            TokenType::LessEqual => Ok(Object::Bool(left <= right)),
            TokenType::Minus => left - right,
            TokenType::Slash => left / right,
            TokenType::Star => left * right,
            TokenType::Plus => left + right,
            _ => unreachable!("Shouldnt be here"),
        }
    }

    fn visit_grouping(&self, expr: &crate::ast::Grouping) -> Res<Object> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal(&self, expr: &crate::ast::Literal) -> Res<Object> {
        Ok(expr.value.clone())
    }

    fn visit_unary(&self, expr: &crate::ast::Unary) -> Res<Object> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.kind {
            TokenType::Minus => -right,
            TokenType::Bang => !right,
            _ => unreachable!("Shouldnt be here?"),
        }
    }
}
