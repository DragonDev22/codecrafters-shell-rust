#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

enum Cmd {
    Echo(String),
    Exit,
    Other(String),
}

fn main() {
    let mut exit: bool = false;

    while !exit {
        let command = get_input();

        let task = evaluate(&command);

        exit = execute(task);
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

fn evaluate(command: &String) -> Cmd {
    let mut task;
    let first_word = first_word(&command);
    match first_word {
        "exit" => task = Cmd::Exit,
        "quit" => task = Cmd::Exit,
        "echo" => task = Cmd::Echo(command.split_once(" ").unwrap().1.to_string()),
        _ => task = Cmd::Other(first_word.to_string()),
    }
    task
}

fn execute(task: Cmd) -> bool {
    let mut exit = false;
    match task {
        Cmd::Exit => exit = true,
        Cmd::Echo(value) => println!("{}", value),
        Cmd::Other(value) => println!("{}: command not found", value),
    }
    exit
}

fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
