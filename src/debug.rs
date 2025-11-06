use crate::chunk::{Chunk, OpCode};
use crate::value;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    let instruction = chunk.code[offset];

    match instruction {
        x if x == OpCode::OpConstant as u8 => constant_instruction("OP_CONSTANT", chunk, offset),
        x if x == OpCode::OpNegate as u8 => simple_instruction("OP_NEGATE", offset),
        x if x == OpCode::OpAdd as u8 => simple_instruction("OP_ADD", offset),
        x if x == OpCode::OpSubtract as u8 => simple_instruction("OP_SUBTRACT", offset),
        x if x == OpCode::OpMultiply as u8 => simple_instruction("OP_MULTIPLY", offset),
        x if x == OpCode::OpDivide as u8 => simple_instruction("OP_DIVIDE", offset),
        x if x == OpCode::OpReturn as u8 => simple_instruction("OP_RETURN", offset),
        _ => {
            println!("Unknown opcode {}", instruction);
            offset + 1
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant_index = chunk.code[offset + 1] as usize;
    print!("{:<16} {:4} '", name, constant_index);
    value::print_value(chunk.get_constant(constant_index));
    println!("'");
    offset + 2  
}
