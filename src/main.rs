use crate::shell::Shell;

mod builtins;
mod shell;

fn main() {
    let mut shell = Shell::new();
    shell.run_loop();
}
