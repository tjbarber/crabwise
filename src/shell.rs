use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::os::unix::process::CommandExt;
use std::os::unix::fs::PermissionsExt;
use std::str;

use crate::builtins::builtin_command::BuiltinCommand;
use crate::builtins::echo::Echo;
use crate::builtins::exit::Exit;
use crate::builtins::r#type::Type;
use crate::builtins::pwd::Pwd;

pub enum ShellAction {
    CachePath { cmd: String, path: String },
    Continue,
    Exit(i32),
}

pub struct Shell {
    pub builtins: HashMap<String, Box<dyn BuiltinCommand>>,
    pub exec_map: HashMap<String, String>,
    pub path: Vec<String>,
}

impl Shell {
    pub fn new() -> Self {
        let mut builtins: HashMap<String, Box<dyn BuiltinCommand>> = HashMap::new();

        builtins.insert("exit".into(), Box::new(Exit));
        builtins.insert("echo".into(), Box::new(Echo));
        builtins.insert("type".into(), Box::new(Type));
        builtins.insert("pwd".into(), Box::new(Pwd));

        let exec_map = HashMap::new();
        let mut path: Vec<String> = Vec::new();

        if let Ok(path_str) = env::var("PATH") {
            path = path_str
                .split(":")
                .map(|i| i.to_string())
                .collect::<Vec<String>>();
        }

        Shell { builtins, exec_map, path }
    }

    pub fn run_loop(&mut self) {
        loop {
            let input = self.prompt();

            if input.is_empty() {
                continue;
            }

            let actions = self.execute_command(input);

            for action in actions {
                match action {
                    ShellAction::CachePath { cmd, path } => {
                        self.exec_map.insert(cmd, path);
                    },
                    ShellAction::Continue => continue,
                    ShellAction::Exit(code) => std::process::exit(code),
                }
            }
        }
    }

    pub fn find_executable(&self, name: &str) -> Option<String> {
        if let Some(exec_path) = self.exec_map.get(name) {
            return Some(exec_path.to_string());
        }

        let paths = &self.path.clone();
        for dir in paths {
            let path = Path::new(&dir).join(name);

            if let Ok(metadata) = fs::metadata(&path) {
                let permissions = metadata.permissions();
                let mode = permissions.mode();

                // check for 'execute' bit
                if mode & 0o111 != 0 && metadata.is_file() {
                    let path_str = path.to_string_lossy().to_string();
                    return Some(path_str);
                }
            }
        }

        return None;
    }

    fn prompt(&self) -> String {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed_input = input.trim();

        return trimmed_input.to_string();
    }

    fn execute_command(&mut self, input: String) -> Vec<ShellAction> {
        let mut split_input: Vec<&str> = input.split(' ').collect();
        let args = split_input.split_off(1);
        let name = split_input[0];

        if let Some(cmd) = self.builtins.get(name) {
            return cmd.execute(args, self);
        } else if let Some(cmd) = self.find_executable(name) {
            // I want to find a better way of doing this but for now
            // I think this is fine
            let full_path = cmd.clone();
            let path = Path::new(&cmd);
            let file_name = path.file_name().unwrap();
            
            Command::new(full_path)
                .arg0(file_name)
                .args(args)
                .spawn()
                .expect("i don't know what this does")
                .wait()
                .expect("failed to wait on child");

            return vec![ShellAction::Continue];
        }

        eprintln!("{}: command not found", name);
        // return code 127, probably will need that
        return vec![ShellAction::Continue];
    }
}
