use std::{env, fs, process};

use crate::{
    scanner::Scanner,
    token::{Token, TokenType},
    vm::Vm,
};

mod chunk;
mod compiler;
mod scanner;
mod token;
mod value;
mod vm;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: rlox [script]");
        process::exit(64);
    }
    run_file(&args[1]);
}

fn run_file(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Could not read file \"{}\": {}", path, e);
            process::exit(74);
        }
    };
    run(&source);
}
fn run(source: &str) {
    let mut had_error = false;

    // --- 1. Scanning ---
    let (tokens, scan_had_error) = scan_source(source);
    had_error |= scan_had_error; // 更新全局错误标志

    if had_error {
        println!("Execution halted due to scanning errors.");
        return;
    }
    // // 4. 执行 -> 结果
    let mut vm = Vm::new(tokens);
    let _ = vm.interpret();
}

fn scan_source(source: &str) -> (Vec<Token>, bool) {
    let mut had_error = false;
    let mut scanner = Scanner::new(source);
    let mut tokens = Vec::new();

    loop {
        let token = scanner.scan_token();
        if token.token_type == TokenType::Error {
            report(token.line, "", token.lexeme);
            had_error = true;
        }
        let token_type = token.token_type;
        tokens.push(token);
        if token_type == TokenType::Eof {
            break;
        }
    }
    (tokens, had_error)
}

// 统一的错误报告函数
fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, location, message);
}

#[test]
fn tests() {
    run_file(r"./test.lox");
}
