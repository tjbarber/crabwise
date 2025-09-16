use std::env;

use crate::builtins::builtin_command::BuiltinCommand;
use crate::shell::{Shell, ShellAction};

pub struct Cd;

impl BuiltinCommand for Cd{
    fn execute(&self, args: Vec<&str>, _: &Shell) -> Vec<ShellAction> {
        if args.len() > 1 {
            println!("cd: too many arguments");
            return vec![ShellAction::Continue];
        }

        let mut dir = args[0].to_string();

        if dir == "~" {
            dir = env::var("HOME").unwrap();
        } 

        let result = env::set_current_dir(&dir);
       
        // TODO: Return error codes from ShellAction::Continue
        let _ = match result {
            Ok(_) => 0,
            Err(_) => {
                println!("cd: {}: No such file or directory", dir);
                1
            },
        };
        return vec![ShellAction::Continue];
    }
}
