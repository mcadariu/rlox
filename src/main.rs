use crate::chunk::{Chunk, OpCode};

mod chunk;
mod debug;
mod value;

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::OpConstant);
    chunk.write_byte(constant as u8);

    chunk.write(OpCode::OpReturn);

    debug::disassemble_chunk(&chunk, "test chunk");
}
