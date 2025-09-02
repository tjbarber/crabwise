use crate::shell::Shell;

mod builtins;
mod shell;

fn main() {
    let shell = Shell::new();
    shell.run_loop();
}
