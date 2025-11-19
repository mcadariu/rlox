use crate::vm::{InterpretResult, VM};
use std::io::Write;
use std::{env, fs, io, process};

mod chunk;
mod compiler;
mod debug;
mod scanner;
mod table;
mod value;
mod vm;

fn main() {
    let mut vm = VM::new();
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(&mut vm),
        2 => run_file(&args[1], &mut vm),
        _ => {
            eprintln!("Usage: rlox [path]");
            process::exit(64);
        }
    }
}

fn run_file(path: &str, vm: &mut VM) {
    let source = read_file(path);
    let result = interpret(&source, vm);
    match result {
        InterpretResult::CompileError => process::exit(65),
        InterpretResult::RuntimeError => process::exit(70),
        _ => {}
    }
}

fn interpret(source: &str, vm: &mut VM) -> InterpretResult {
    match compiler::compile(source, vm) {
        Some(chunk) => vm.interpret(chunk),
        None => InterpretResult::CompileError,
    }
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read file")
}

fn repl(vm: &mut VM) {
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {
                interpret(&line, vm);
            }
            Err(_) => break,
        };
    }
}
