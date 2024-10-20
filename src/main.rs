use roxy::{
    chunks::{Chunk, Opcode},
    vm::Vm,
};

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write(Opcode::Constant(constant), 123);

    let constant = chunk.add_constant(3.4);
    chunk.write(Opcode::Constant(constant), 123);

    chunk.write(Opcode::Add, 123);

    let constant = chunk.add_constant(5.6);
    chunk.write(Opcode::Constant(constant), 123);

    chunk.write(Opcode::Divide, 123);

    chunk.write(Opcode::Negate, 123);

    // let constant = chunk.add_constant(3);
    // chunk.write(Opcode::Constant(constant), 123);

    // let constant = chunk.add_constant(1);
    // chunk.write(Opcode::Constant(constant), 123);

    // chunk.write(Opcode::Subtract, 123);
    chunk.write(Opcode::Return, 123);

    let mut vm = Vm::new(&chunk);
    vm.interpret().unwrap();
}
