use crate::builtins::command::Command;
use crate::shell::{Shell, ShellAction};

pub struct Exit;

impl Command for Exit {
    fn execute(&self, _: Vec<&str>, _: &Shell) -> Vec<ShellAction> {
        return vec![ShellAction::Exit(0)];
    }
}
