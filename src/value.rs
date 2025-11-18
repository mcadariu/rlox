struct ValueArray {
    values: Vec<Value>,
}

impl ValueArray {
    fn new() -> Self {
        ValueArray { values: Vec::new() }
    }

    fn write(&mut self, value: Value) {
        self.values.push(value);
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
    String(String),
}

impl Value {
    pub fn bool(value: bool) -> Self {
        Value::Bool(value)
    }

    pub fn nil() -> Self {
        Value::Nil
    }

    pub fn number(value: f64) -> Self {
        Value::Number(value)
    }

    pub fn string(value: String) -> Self {
        Value::String(value)
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            _ => panic!("Not a boolean"),
        }
    }

    pub fn as_number(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            _ => panic!("Not a number"),
        }
    }

    pub fn as_string(&self) -> &str {
        match self {
            Value::String(s) => s.as_str(),
            _ => panic!("Not a string"),
        }
    }

    pub fn is_falsey(&self) -> bool {
        matches!(self, Value::Nil | Value::Bool(false))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            _ => false,
        }
    }
}

pub fn print_value(value: &Value) {
    match value {
        Value::Bool(b) => print!("{}", b),
        Value::Nil => print!("nil"),
        Value::Number(n) => print!("{}", n),
        Value::String(s) => print!("{}", s),
    }
}
