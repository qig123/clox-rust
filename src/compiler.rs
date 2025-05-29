use crate::{scanner, token_type};

pub fn compile(source: &str) {
    let mut scanner = scanner::Scanner::new(source);
    let mut line = 0;
    loop {
        let token = scanner.scan_token();
        if line != token.line {
            println!("{:04}", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }
        println!("{:?} ", token);
        if token.kind == token_type::TokenType::Eof {
            break;
        }
    }
}
