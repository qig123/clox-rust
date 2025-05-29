use chunk::OpCode;

mod chunk;
mod compiler;
mod debug;
mod scanner;
mod token_type;
mod value;
mod vm;
fn main() {
    // let args = std::env::args().collect::<Vec<String>>();

    // match args.len() {
    //     2 => {
    //         // let script = &args[1];
    //         let script = r"./test.lox";
    //         let _ = run_file(script);
    //     }
    //     _ => {
    //         eprintln!("Usage: {} <script>", args[0]);
    //         std::process::exit(1);
    //     }
    // }
    let script = r"./src/test.lox";
    let _ = run_file(script);
}
fn run_file(script: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(script);
    match source {
        Ok(content) => {
            let mut vm = vm::VM::new();
            let c = vm.interpret(content);
            match c {
                Ok(_) => println!("Script executed successfully."),
                Err(e) => eprintln!("Error executing script: {}", e),
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Error reading file {}: {}", script, e);
            return Err(Box::new(e));
        }
    }
}
