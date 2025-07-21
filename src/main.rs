use std::{env, fs, process};

use crate::{
    chunk::{Chunk, OpCode},
    scanner::Scanner,
    token::TokenType,
    value::Value,
    vm::Vm,
};

mod chunk;
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
    run_file(&args[1]).unwrap_or_else(|err| {
        // 这个 unwrap 只处理文件 I/O 错误
        eprintln!("Could not read file: {}", err);
        process::exit(74); // EX_IOERR
    });
}

fn run_file(path: &str) -> Result<(), std::io::Error> {
    let source = fs::read_to_string(path)?;
    let had_error = run(&source);
    // 如果扫描/编译阶段有错误，就以错误码退出
    if had_error {
        process::exit(65); // EX_DATAERR，表示数据格式错误
    }
    Ok(())
}

// run 函数现在返回一个布尔值，表示是否遇到了错误
fn run(source: &str) -> bool {
    let mut scanner = Scanner::new(source);
    let mut line = 0;
    let mut had_error = false; // 错误标志

    loop {
        let token = scanner.scan_token();

        // 如果 token 是错误类型，报告它
        if token.token_type == TokenType::Error {
            report(token.line, "", token.lexeme); // 使用一个新的辅助函数来报告错误
            had_error = true;
        }

        // 为了便于调试，我们仍然可以打印所有 token
        if token.line != line {
            print!("{:4} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }
        println!("{}", token);

        if token.token_type == TokenType::Eof {
            break;
        }
    }

    had_error // 返回最终的错误状态
}

// 新增：一个统一的错误报告函数
fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, location, message);
}
