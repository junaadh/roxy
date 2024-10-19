use roxy::chunks::{Chunk, Disassmbler, Opcode};

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write(Opcode::Constant(constant), 123);

    chunk.write(Opcode::Return, 123);

    let disassmbler = Disassmbler::new(&chunk);
    disassmbler.disassemble("test chunk");
}
