use std::env;

use roxy::interpreter::Interpreter;

fn main() {
    let mut args = env::args();

    let program = args.next().unwrap();
    let engine = Interpreter;

    match args.len() {
        0 => engine.run_repl(),
        1 => engine.run_file(&args.next().unwrap()),
        _ => panic!("Usage: {program} [script]"),
    }
    .unwrap();
}
