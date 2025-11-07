use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

const STACK_MAX: usize = 256;

#[derive(Debug)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    chunk: Option<Chunk>,
    ip: usize,  // instruction pointer
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            chunk: None,
            ip: 0,
            stack: Vec::with_capacity(STACK_MAX),
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            #[cfg(feature = "debug_trace_execution")]
            {
                // Print the stack
                print!("          ");
                for value in &self.stack {
                    print!("[ ");
                    crate::value::print_value(*value);
                    print!(" ]");
                }
                println!();

                // Disassemble the current instruction
                crate::debug::disassemble_instruction(
                    self.chunk.as_ref().unwrap(),
                    self.ip
                );
            }

            let instruction = self.read_byte();
            match instruction {
                x if x == OpCode::OpConstant as u8 => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                x if x == OpCode::OpNegate as u8 => {
                    let value = self.pop();
                    self.push(-value);
                }
                x if x == OpCode::OpAdd as u8 => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a + b);
                }
                x if x == OpCode::OpSubtract as u8 => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a - b);
                }
                x if x == OpCode::OpMultiply as u8 => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a * b);
                }
                x if x == OpCode::OpDivide as u8 => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a / b);
                }
                x if x == OpCode::OpReturn as u8 => {
                    crate::value::print_value(self.pop());
                    println!();
                    return InterpretResult::Ok;
                }
                _ => {
                    return InterpretResult::RuntimeError;
                }
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.as_ref().unwrap().code[self.ip];
        self.ip += 1;
        byte
    }

    fn read_constant(&mut self) -> Value {
        let index = self.read_byte() as usize;
        self.chunk.as_ref().unwrap().get_constant(index)
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Stack underflow")
    }
}
