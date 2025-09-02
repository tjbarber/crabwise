use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, Write};

pub enum ShellAction {
    Continue(i32),
    Exit(i32),
}

pub trait BuiltinCommand {
    fn execute(&self, args: &str) -> ShellAction;
}

pub struct Exit;

impl BuiltinCommand for Exit {
    fn execute(&self, _: &str) -> ShellAction {
        return ShellAction::Exit(0);
    }
}

pub struct Shell {
    builtins: HashMap<String, Box<dyn BuiltinCommand>>,
}

impl Shell {
    fn new() -> Self {
        let mut builtins: HashMap<String, Box<dyn BuiltinCommand>> = HashMap::new();
        builtins.insert("exit".into(), Box::new(Exit));
        Shell { builtins }
    }

    fn run_loop(&self) {
        loop {
            let input = self.prompt();

            if input.is_empty() {
                continue;
            }

            let action = self.execute_command(input);

            match action {
                ShellAction::Continue(_) => continue,
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
            return ShellAction::Continue(127);
        }
    }
}

fn main() {
    let shell = Shell::new();
    shell.run_loop();
}
