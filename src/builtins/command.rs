use crate::shell::{Shell, ShellAction};

pub trait Command {
    fn execute(&self, args: Vec<&str>, shell: &Shell) -> ShellAction;
}
