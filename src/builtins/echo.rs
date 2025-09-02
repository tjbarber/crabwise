use crate::builtins::command::Command;
use crate::shell::{Shell, ShellAction};

pub struct Echo;

impl Command for Echo {
    fn execute(&self, args: Vec<&str>, _: &Shell) -> ShellAction {
        println!("{}", args.join(" "));
        return ShellAction::Continue;
    }
}
