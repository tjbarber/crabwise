use crate::builtins::command::Command;
use crate::shell::{Shell, ShellAction};

pub struct Type;

impl Command for Type {
    fn execute(&self, args: Vec<&str>, shell: &Shell) -> Vec<ShellAction> {
        let mut actions: Vec<ShellAction> = Vec::new();
        for arg in args {
            if shell.builtins.contains_key(arg) {
                println!("{} is a shell builtin", arg);
            } else if let Some(path) = shell.find_executable(arg) {
                println!("{} is {}", arg, path);
                actions.push(ShellAction::CachePath { cmd: arg.to_string(), path })
            } else {
                println!("{}: not found", arg);
            }
        }
        return actions;
    }
}
