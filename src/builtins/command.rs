use crate::shell::ShellAction;

pub trait Command {
    fn execute(&self, args: &str) -> ShellAction;
}
