use std::{env, fs, process};

use crate::{
    chunk::{Chunk, OpCode},
    compiler::Compiler,
    parser::Parser,
    scanner::Scanner,
    token::{Token, TokenType},
    value::Value,
    vm::Vm,
};

mod ast;
mod chunk;
mod compiler;
mod parser;
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

    // --- 2. Parsing ---
    println!("\n--- Parsing ---");
    let mut parser = Parser::new(&tokens);
    // 调用新的 parse 方法
    let parse_result = parser.parse();

    // 检查解析过程中是否收集到了错误
    if !parse_result.errors.is_empty() {
        had_error = true;
        println!("Found {} parsing error(s):", parse_result.errors.len());
        for err in parse_result.errors {
            report(
                err.token.line,
                &format!(" at '{}'", err.token.lexeme),
                &err.message,
            );
        }
    }

    // 如果有解析错误，就不再继续
    if had_error {
        println!("Execution halted due to parsing errors.");
        return;
    }

    // 如果没有错误，我们可以安全地使用解析出的 AST
    let ast = parse_result.statements;
    println!("Parsing successful. AST:");
    for stmt in ast {
        println!("{}", stmt);
    }

    // 3. 编译 -> 字节码 (如果解析成功)
    println!("\n--- Compiling & Executing ---");
    // let compiler = Compiler::new();
    // let chunk = match compiler.compile(&expression) {
    //     Ok(chunk) => chunk,
    //     Err(e) => {
    //         eprintln!("[Compiler Error] {}", e);
    //         // had_error = true; // 可以在这里设置
    //         return;
    //     }
    // };

    // // 4. 执行 -> 结果
    // let mut vm = Vm::new(chunk);
    // let _ = vm.interpret();
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
