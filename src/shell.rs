use std::collections::HashMap;
use std::io::{self, Write};

use crate::builtins::command::Command;
use crate::builtins::echo::Echo;
use crate::builtins::exit::Exit;

pub enum ShellAction {
    Continue,
    Exit(i32),
}

pub struct Shell {
    builtins: HashMap<String, Box<dyn Command>>,
}

impl Shell {
    pub fn new() -> Self {
        let mut builtins: HashMap<String, Box<dyn Command>> = HashMap::new();
        builtins.insert("exit".into(), Box::new(Exit));
        builtins.insert("echo".into(), Box::new(Echo));
        Shell { builtins }
    }

    pub fn run_loop(&self) {
        loop {
            let input = self.prompt();

            if input.is_empty() {
                continue;
            }

            let action = self.execute_command(input);

            match action {
                ShellAction::Continue => continue,
                ShellAction::Exit(code) => std::process::exit(code),
            }
        }
    }

    fn prompt(&self) -> String {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed_input = input.trim();

        return trimmed_input.to_string();
    }

    fn execute_command(&self, input: String) -> ShellAction {
        let split_input = input.split_once(" ");
        let name;
        let args;

        match split_input {
            Some((first, remaining)) => {
                // Come back to this, we need to just split the whole thing,
                // get the first as the name,
                // pass the rest as arguments
                name = first.to_string();
                args = remaining.to_string();
            }
            None => {
                name = input.to_string();
                args = String::new();
            }
        }

        if let Some(cmd) = self.builtins.get(&name) {
            return cmd.execute(&args);
        } else {
            eprintln!("{}: command not found", name);
            // return code 127, probably will need that
            return ShellAction::Continue;
        }
    }
}
