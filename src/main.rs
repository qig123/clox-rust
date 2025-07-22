use std::{env, fs, process};

use crate::{
    chunk::{Chunk, OpCode},
    compiler::Compiler,
    parser::Parser,
    scanner::Scanner,
    token::TokenType,
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

    // 1. 扫描 -> Token 流
    // 这个阶段只负责生成 token 流，并顺便报告扫描错误。
    let mut scanner = Scanner::new(source);
    let mut tokens = Vec::new();
    println!("--- Scanning ---");
    loop {
        let token = scanner.scan_token();
        if token.token_type == TokenType::Error {
            report(token.line, "", token.lexeme);
            had_error = true;
            // **关键改动**: 不再 return，只是设置标志并继续扫描！
        }

        let token_type = token.token_type;
        tokens.push(token);

        if token_type == TokenType::Eof {
            break;
        }
    }

    // (可选) 打印所有扫描到的 tokens，用于调试
    // for token in &tokens {
    //     println!("{:?}", token);
    // }

    // 如果扫描阶段已经发现错误，我们就不再进入解析和后续阶段。
    // 这是一个重要的“故障保护”，防止解析器处理一个有缺陷的 token 流。
    if had_error {
        println!("Execution halted due to scanning errors.");
        // 如果需要，这里可以 process::exit(65);
        return;
    }

    // 2. 解析 -> AST (如果扫描成功)
    println!("\n--- Parsing ---");
    let mut parser = Parser::new(&tokens);
    let ast = match parser.parse() {
        Ok(expr) => expr,
        Err(err) => {
            // 解析器在遇到第一个无法处理的错误时就会停止并返回。
            // 未来可以实现更复杂的错误恢复，让解析器能报告多个解析错误。
            report(
                err.token.line,
                &format!(" at '{}'", err.token.lexeme),
                &err.message,
            );
            had_error = true;
            // 因为解析已经失败，我们直接返回，不进入编译阶段。
            return;
        }
    };
    for stmt in ast {
        // 直接打印，因为 Stmt 实现了 Display
        println!("{}", stmt);
    }
    // 如果解析阶段有错误，就不再继续
    if had_error {
        println!("Execution halted due to parsing errors.");
        return;
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

// 统一的错误报告函数
fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, location, message);
}
