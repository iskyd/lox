use std::env;
use std::io;
use std::io::Write;

mod errors;
mod token_scanner;
mod tokens;

fn run_prompt() {
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        println!("You entered: {}", input.trim());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
    } else if args.len() == 2 {
        // runFile(args[0]);
    } else {
        run_prompt();
    }
}
