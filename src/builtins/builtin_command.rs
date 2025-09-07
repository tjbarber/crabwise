use crate::shell::{Shell, ShellAction};

pub trait BuiltinCommand {
    fn execute(&self, args: Vec<&str>, shell: &Shell) -> Vec<ShellAction>;
}
