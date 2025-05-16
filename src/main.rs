mod shell;
use shell::Shell;

fn main() {
    println!("Hello, world!");
    let mut interactive_shell = Shell::new();
    interactive_shell.start();
}
