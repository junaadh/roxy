use std::env;

use roxy::{
    ast::{AstPrinter, Binary, Expr, Grouping, Literal, Span, Token, Unary},
    interpreter::Interpreter,
    types::Object,
};

fn main() {
    let mut args = env::args();

    let program = args.next().unwrap();
    let engine = Interpreter;

    // match args.len() {
    //     0 => engine.run_repl(),
    //     1 => engine.run_file(&args.next().unwrap()),
    //     _ => panic!("Usage: {program} [script]"),
    // }
    // .unwrap();

    let expr = Expr::Binary(Box::new(Binary {
        left: Expr::Unary(Box::new(Unary {
            operator: Token::new(roxy::ast::TokenType::Minus, Span::new(0, 0, 0)),
            right: Expr::Literal(Box::new(Literal {
                value: Object::Int(123),
            })),
        })),
        operator: Token::new(roxy::ast::TokenType::Star, Span::new(0, 0, 0)),
        right: Expr::Grouping(Box::new(Grouping {
            expression: Expr::Literal(Box::new(Literal {
                value: Object::Float(45.6),
            })),
        })),
    }));

    let ast = AstPrinter;

    println!("{}", ast.print(&expr));
}
