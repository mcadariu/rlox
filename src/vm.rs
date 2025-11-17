use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

const STACK_MAX: usize = 256;

#[derive(Debug)]
pub enum InterpretResult {
    Ok,
    RuntimeError,
    CompileError,
}

pub struct VM {
    chunk: Option<Chunk>,
    ip: usize,
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
            let instruction = self.read_byte();
            match instruction {
                x if x == OpCode::OpConstant as u8 => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                x if x == OpCode::OpNil as u8 => {
                    self.push(Value::nil());
                }
                x if x == OpCode::OpTrue as u8 => {
                    self.push(Value::bool(true));
                }
                x if x == OpCode::OpFalse as u8 => {
                    self.push(Value::bool(false));
                }
                x if x == OpCode::OpEqual as u8 => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::bool(a == b));
                }
                x if x == OpCode::OpGreater as u8 => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.");
                        return InterpretResult::RuntimeError;
                    }
                    let b = self.pop().as_number();
                    let a = self.pop().as_number();
                    self.push(Value::bool(a > b));
                }
                x if x == OpCode::OpLess as u8 => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.");
                        return InterpretResult::RuntimeError;
                    }
                    let b = self.pop().as_number();
                    let a = self.pop().as_number();
                    self.push(Value::bool(a < b));
                }
                x if x == OpCode::OpNot as u8 => {
                    let value = self.pop();
                    self.push(Value::bool(value.is_falsey()));
                }
                x if x == OpCode::OpNegate as u8 => {
                    if !self.peek(0).is_number() {
                        self.runtime_error("Operand must be a number.");
                        return InterpretResult::RuntimeError;
                    }
                    let value = self.pop().as_number();
                    self.push(Value::number(-value));
                }
                x if x == OpCode::OpAdd as u8 => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.");
                        return InterpretResult::RuntimeError;
                    }
                    let b = self.pop().as_number();
                    let a = self.pop().as_number();
                    self.push(Value::number(a + b));
                }
                x if x == OpCode::OpSubtract as u8 => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.");
                        return InterpretResult::RuntimeError;
                    }
                    let b = self.pop().as_number();
                    let a = self.pop().as_number();
                    self.push(Value::number(a - b));
                }
                x if x == OpCode::OpMultiply as u8 => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.");
                        return InterpretResult::RuntimeError;
                    }
                    let b = self.pop().as_number();
                    let a = self.pop().as_number();
                    self.push(Value::number(a * b));
                }
                x if x == OpCode::OpDivide as u8 => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.");
                        return InterpretResult::RuntimeError;
                    }
                    let b = self.pop().as_number();
                    let a = self.pop().as_number();
                    self.push(Value::number(a / b));
                }
                x if x == OpCode::OpReturn as u8 => {
                    crate::value::print_value(&self.pop());
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

    fn peek(&self, distance: usize) -> &Value {
        &self.stack[self.stack.len() - 1 - distance]
    }

    fn runtime_error(&self, message: &str) {
        eprintln!("{}", message);
        let line = self.chunk.as_ref().unwrap().lines[self.ip - 1];
        eprintln!("[line {}] in script", line);
    }
}
