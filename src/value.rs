pub type Value = f64;

struct ValueArray {
    values: Vec<Value>
}

impl ValueArray {
    fn new() -> Self {
        ValueArray {
            values: Vec::new()
        }
    }

    fn write(&mut self, value: Value) {
        self.values.push(value);
    }
}

pub fn print_value(value: Value) {
    print!("{}", value);
}
