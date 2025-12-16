pub(crate) use crate::value::Value;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    OpConstant,
    OpNil,
    OpTrue,
    OpFalse,
    OpPop,
    OpEqual,
    OpGreater,
    OpLess,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNot,
    OpNegate,
    OpPrint,
    OpDefineGlobal,
    OpGetGlobal,
    OpSetGlobal,
    OpGetLocal,
    OpSetLocal,
    OpJumpIfFalse,
    OpJump,
    OpLoop,
    OpReturn,
}

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write(&mut self, opcode: OpCode, line: usize) {
        self.code.push(opcode as u8);
        self.lines.push(line);
    }

    pub fn write_byte(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn get_constant(&self, index: usize) -> Value {
        self.constants[index].clone()
    }

    pub fn emit_jump(&mut self, instruction: OpCode, line: usize) -> usize {
        self.write(instruction, line);
        // Emit placeholder bytes for the jump offset
        self.write_byte(0xff, line);
        self.write_byte(0xff, line);
        self.code.len() - 2
    }

    pub fn patch_jump(&mut self, offset: usize) {
        // -2 to adjust for the bytecode for the jump offset itself
        let jump = self.code.len() - offset - 2;

        if jump > u16::MAX as usize {
            panic!("Too much code to jump over.");
        }

        self.code[offset] = ((jump >> 8) & 0xff) as u8;
        self.code[offset + 1] = (jump & 0xff) as u8;
    }

    pub fn emit_loop(&mut self, loop_start: usize, line: usize) {
        self.write(OpCode::OpLoop, line);

        let offset = self.code.len() - loop_start + 2;
        if offset > u16::MAX as usize {
            panic!("Loop body too large.");
        }

        self.write_byte(((offset >> 8) & 0xff) as u8, line);
        self.write_byte((offset & 0xff) as u8, line);
    }
}
