use crate::builtins::command::Command;
use crate::shell::ShellAction;

pub struct Exit;

impl Command for Exit {
    fn execute(&self, _: &str) -> ShellAction {
        return ShellAction::Exit(0);
    }
}
