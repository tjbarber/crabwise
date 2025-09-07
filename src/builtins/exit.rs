use crate::builtins::builtin_command::BuiltinCommand;
use crate::shell::{Shell, ShellAction};

pub struct Exit;

impl BuiltinCommand for Exit {
    fn execute(&self, _: Vec<&str>, _: &Shell) -> Vec<ShellAction> {
        return vec![ShellAction::Exit(0)];
    }
}
