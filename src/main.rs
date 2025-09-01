#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    prompt();
}

fn prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
    let input = await_input();
    let trimmed_input = input.trim();

    if trimmed_input.is_empty() {
        prompt();
    } else {
        println!("{}: command not found", trimmed_input)
    }
}

fn await_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    return input;
}
