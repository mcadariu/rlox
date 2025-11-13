use std::{env, fs, io, process};
use std::io::Write;
use crate::vm::{InterpretResult};
use crate::scanner::init_scanner;

mod chunk;
mod debug;
mod value;
mod vm;
mod scanner;
mod compiler;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: rlox [path]");
            process::exit(64);
        }
    }

}

fn run_file(path: &str) {
    let source = read_file(path);
    let result = interpret(&source);
    match result {
        InterpretResult::CompileError => process::exit(65),
        InterpretResult::RuntimeError => process::exit(70),
        _ => {}
    }
}

fn interpret(source: &str) -> InterpretResult {
    compile(source);
    InterpretResult::Ok
}

fn compile(source: &str) {
    let scanner = init_scanner(source);
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read file")
}

fn repl() {
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(0) => break,  // EOF
            Ok(_) => interpret(&line),
            Err(_) => break,
        };
    }
}
