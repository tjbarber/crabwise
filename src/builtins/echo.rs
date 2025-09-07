use crate::builtins::builtin_command::BuiltinCommand;
use crate::shell::{Shell, ShellAction};

pub struct Echo;

impl BuiltinCommand for Echo {
    fn execute(&self, args: Vec<&str>, _: &Shell) -> Vec<ShellAction> {
        println!("{}", args.join(" "));
        return vec![ShellAction::Continue];
    }
}
