#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let input = prompt();

        if !input.is_empty() {
            command_router(input);
        }
    }
}

fn prompt() -> String {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed_input = input.trim();

    return trimmed_input.to_string();
}

fn command_router(input: String) {
    match input {
        _ => println!("{}: command not found", input),
    }
}
