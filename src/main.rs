#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    let exit: bool = false;

    while !exit {
        let command = get_input();

        // Evaluate:
        evaluate(&command);

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

fn evaluate(command: &String) {
    match command.as_str() {
        "exit" => exit(0),
        "quit" => exit(0),
        _ => return,
    }
}
