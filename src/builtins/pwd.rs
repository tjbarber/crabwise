use std::env;

use crate::builtins::builtin_command::BuiltinCommand;
use crate::shell::{Shell, ShellAction};

pub struct Pwd;

impl BuiltinCommand for Pwd {
    fn execute(&self, args: Vec<&str>, _: &Shell) -> Vec<ShellAction> {
        if args.len() > 0 {
            println!("pwd: too many arguments");
            return vec![ShellAction::Continue];
        }

        let current_path = env::current_dir().expect("if PWD isn't set we have bigger problems.");
        println!("{}", current_path.display());

        return vec![ShellAction::Continue];
    }
}
