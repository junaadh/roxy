use std::env;

use roxy::{chunks::Chunk, vm::Vm};

fn main() {
    let mut args = env::args();

    let program = args.next().unwrap();

    let mut chunk = Chunk::new();
    let mut vm = Vm::new(&mut chunk);

    match args.len() {
        0 => vm.run_repl(),
        1 => vm.run_file(&args.next().unwrap()),
        _ => panic!("Usage: {program} [script]"),
    }
    .unwrap();
}
