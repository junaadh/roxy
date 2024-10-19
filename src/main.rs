use roxy::chunks::{Chunk, Opcode};

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(Opcode::Return);
}
