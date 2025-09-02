use crate::builtins::command::Command;
use crate::shell::ShellAction;

pub struct Echo;

impl Command for Echo {
    fn execute(&self, args: &str) -> ShellAction {
        println!("{}", args.to_string());
        return ShellAction::Continue;
    }
}
