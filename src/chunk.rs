use crate::value::Value;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    OpConstant,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpReturn,
}

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write(&mut self, opcode: OpCode) {
        self.code.push(opcode as u8);
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn get_constant(&self, index: usize) -> Value {
        self.constants[index]
    }
}
