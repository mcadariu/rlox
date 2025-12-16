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
        x if x == OpCode::OpNil as u8 => simple_instruction("OP_NIL", offset),
        x if x == OpCode::OpTrue as u8 => simple_instruction("OP_TRUE", offset),
        x if x == OpCode::OpFalse as u8 => simple_instruction("OP_FALSE", offset),
        x if x == OpCode::OpEqual as u8 => simple_instruction("OP_EQUAL", offset),
        x if x == OpCode::OpGreater as u8 => simple_instruction("OP_GREATER", offset),
        x if x == OpCode::OpLess as u8 => simple_instruction("OP_LESS", offset),
        x if x == OpCode::OpAdd as u8 => simple_instruction("OP_ADD", offset),
        x if x == OpCode::OpSubtract as u8 => simple_instruction("OP_SUBTRACT", offset),
        x if x == OpCode::OpMultiply as u8 => simple_instruction("OP_MULTIPLY", offset),
        x if x == OpCode::OpDivide as u8 => simple_instruction("OP_DIVIDE", offset),
        x if x == OpCode::OpNot as u8 => simple_instruction("OP_NOT", offset),
        x if x == OpCode::OpNegate as u8 => simple_instruction("OP_NEGATE", offset),
        x if x == OpCode::OpPop as u8 => simple_instruction("OP_POP", offset),
        x if x == OpCode::OpPrint as u8 => simple_instruction("OP_PRINT", offset),
        x if x == OpCode::OpDefineGlobal as u8 => constant_instruction("OP_DEFINE_GLOBAL", chunk, offset),
        x if x == OpCode::OpGetGlobal as u8 => constant_instruction("OP_GET_GLOBAL", chunk, offset),
        x if x == OpCode::OpSetGlobal as u8 => constant_instruction("OP_SET_GLOBAL", chunk, offset),
        x if x == OpCode::OpGetLocal as u8 => byte_instruction("OP_GET_LOCAL", chunk, offset),
        x if x == OpCode::OpSetLocal as u8 => byte_instruction("OP_SET_LOCAL", chunk, offset),
        x if x == OpCode::OpJumpIfFalse as u8 => jump_instruction("OP_JUMP_IF_FALSE", 1, chunk, offset),
        x if x == OpCode::OpJump as u8 => jump_instruction("OP_JUMP", 1, chunk, offset),
        x if x == OpCode::OpLoop as u8 => jump_instruction("OP_LOOP", -1, chunk, offset),
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
    value::print_value(&chunk.get_constant(constant_index));
    println!("'");
    offset + 2
}

fn byte_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let slot = chunk.code[offset + 1];
    println!("{:<16} {:4}", name, slot);
    offset + 2
}

fn jump_instruction(name: &str, sign: i32, chunk: &Chunk, offset: usize) -> usize {
    let high = chunk.code[offset + 1] as u16;
    let low = chunk.code[offset + 2] as u16;
    let jump = (high << 8) | low;
    let target = if sign == 1 {
        offset + 3 + jump as usize
    } else {
        offset + 3 - jump as usize
    };
    println!("{:<16} {:4} -> {}", name, offset, target);
    offset + 3
}
