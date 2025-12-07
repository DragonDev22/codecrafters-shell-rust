#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let exit: bool = false;

    while !exit {
        let command = get_input();

        // Evaluate:

        // If command doesn't exist:
        error(command);
    }
}

fn get_input() -> String {
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    return command.trim().to_string();
}

fn error(command: String) {
    println!("{}: command not found", command)
}
