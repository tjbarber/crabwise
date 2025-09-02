use std::collections::HashMap;
use std::io::{self, Write};

use crate::builtins::command::Command;
use crate::builtins::echo::Echo;
use crate::builtins::exit::Exit;
use crate::builtins::r#type::Type;

pub enum ShellAction {
    Continue,
    Exit(i32),
}

pub struct Shell {
    pub builtins: HashMap<String, Box<dyn Command>>,
}

impl Shell {
    pub fn new() -> Self {
        let mut builtins: HashMap<String, Box<dyn Command>> = HashMap::new();

        builtins.insert("exit".into(), Box::new(Exit));
        builtins.insert("echo".into(), Box::new(Echo));
        builtins.insert("type".into(), Box::new(Type));

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
        let mut split_input: Vec<&str> = input.split(' ').collect();
        let args = split_input.split_off(1);
        let name = split_input[0];

        if let Some(cmd) = self.builtins.get(name) {
            return cmd.execute(args, &self);
        } else {
            eprintln!("{}: command not found", name);
            // return code 127, probably will need that
            return ShellAction::Continue;
        }
    }
}
