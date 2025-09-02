use crate::builtins::command::Command;
use crate::shell::{Shell, ShellAction};

pub struct Type;

impl Command for Type {
    fn execute(&self, args: Vec<&str>, shell: &Shell) -> ShellAction {
        for arg in args {
            if shell.builtins.contains_key(arg) {
                println!("{} is a shell builtin", arg);
            } else {
                println!("{}: not found", arg);
            }
        }
        return ShellAction::Continue;
    }
}
