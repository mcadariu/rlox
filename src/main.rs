use crate::chunk::{Chunk, OpCode};
use crate::vm::VM;

mod chunk;
mod debug;
mod value;
mod vm;

fn main() {
    let mut chunk = Chunk::new();

    // Build the expression: -(1.2 + 3.4) / 5.6

    // Push 1.2
    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::OpConstant);
    chunk.write_byte(constant as u8);

    // Push 3.4
    let constant = chunk.add_constant(3.4);
    chunk.write(OpCode::OpConstant);
    chunk.write_byte(constant as u8);

    // Add: 1.2 + 3.4 = 4.6
    chunk.write(OpCode::OpAdd);

    // Negate: -(4.6) = -4.6
    chunk.write(OpCode::OpNegate);

    // Push 5.6
    let constant = chunk.add_constant(5.6);
    chunk.write(OpCode::OpConstant);
    chunk.write_byte(constant as u8);

    // Divide: -4.6 / 5.6
    chunk.write(OpCode::OpDivide);

    chunk.write(OpCode::OpReturn);

    debug::disassemble_chunk(&chunk, "test chunk");

    let mut vm = VM::new();
    vm.interpret(chunk);
}
