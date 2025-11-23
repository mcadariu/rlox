use crate::chunk::{Chunk, OpCode};
use crate::table::Table;
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
    strings: Table,
    globals: Table,
}

impl VM {
    pub fn new() -> Self {
        VM {
            chunk: None,
            ip: 0,
            stack: Vec::with_capacity(STACK_MAX),
            strings: Table::new(),
            globals: Table::new(),
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
                x if x == OpCode::OpPop as u8 => {
                    self.pop();
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
                    if self.peek(0).is_string() && self.peek(1).is_string() {
                        let b = self.pop();
                        let a = self.pop();
                        let result = format!("{}{}", a.as_string(), b.as_string());
                        self.push(Value::string(result));
                    } else if self.peek(0).is_number() && self.peek(1).is_number() {
                        let b = self.pop().as_number();
                        let a = self.pop().as_number();
                        self.push(Value::number(a + b));
                    } else {
                        self.runtime_error("Operands must be two numbers or two strings.");
                        return InterpretResult::RuntimeError;
                    }
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
                x if x == OpCode::OpPrint as u8 => {
                    crate::value::print_value(&self.pop());
                    println!();
                }
                x if x == OpCode::OpDefineGlobal as u8 => {
                    let constant = self.read_constant();
                    let name = constant.as_string().to_string();
                    let value = self.pop();
                    self.globals.set(name, value);
                }
                x if x == OpCode::OpGetGlobal as u8 => {
                    let constant = self.read_constant();
                    let name = constant.as_string();
                    match self.globals.get(name) {
                        Some(value) => {
                            self.push(value.clone());
                        }
                        None => {
                            self.runtime_error(&format!("Undefined variable '{}'.", name));
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                x if x == OpCode::OpSetGlobal as u8 => {
                    let constant = self.read_constant();
                    let name = constant.as_string().to_string();
                    if self.globals.set(name.clone(), self.peek(0).clone()) {
                        self.globals.delete(&name);
                        self.runtime_error(&format!("Undefined variable '{}'.", name));
                        return InterpretResult::RuntimeError;
                    }
                }
                x if x == OpCode::OpGetLocal as u8 => {
                    let slot = self.read_byte() as usize;
                    self.push(self.stack[slot].clone());
                }
                x if x == OpCode::OpSetLocal as u8 => {
                    let slot = self.read_byte() as usize;
                    self.stack[slot] = self.peek(0).clone();
                }
                x if x == OpCode::OpReturn as u8 => {
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

    pub fn intern_string(&mut self, string: String) -> String {
        let hash = crate::table::hash_string(&string);

        if let Some(interned) = self.strings.find_string(&string, hash) {
            return interned.to_string();
        }

        self.strings.set(string.clone(), Value::nil());
        string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_interning() {
        let mut vm = VM::new();

        let str1 = vm.intern_string("hello".to_string());
        let str2 = vm.intern_string("hello".to_string());
        let str3 = vm.intern_string("world".to_string());

        assert_eq!(str1, str2);
        assert_eq!(str1, "hello");
        assert_ne!(str1, str3);
        assert_eq!(str3, "world");
    }

    #[test]
    fn test_multiple_interning() {
        let mut vm = VM::new();

        for _ in 0..10 {
            vm.intern_string("test".to_string());
        }

        let result = vm.intern_string("test".to_string());
        assert_eq!(result, "test");
    }
}
