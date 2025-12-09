#[allow(unused_imports)]
use std::env;
use std::{
    fmt::format,
    fs,
    io::{self, Write},
};

enum Cmd {
    Echo(String),
    Exit,
    Other(String),
    Type(String),
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
    let task;
    let first_word = first_word(&command);
    match first_word {
        "exit" => task = Cmd::Exit,
        "quit" => task = Cmd::Exit,
        "echo" => task = Cmd::Echo(command.split_once(" ").unwrap().1.to_string()),
        "type" => task = Cmd::Type(command.split_once(" ").unwrap().1.to_string()),
        _ => task = Cmd::Other(first_word.to_string()),
    }
    task
}

fn execute(task: Cmd) -> bool {
    let mut exit = false;
    match task {
        Cmd::Exit => exit = true,
        Cmd::Echo(value) => println!("{}", value),
        Cmd::Other(value) => error(value),
        Cmd::Type(value) => get_type(first_word(value.as_str()).to_string()),
    }
    exit
}

fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn get_type(value: String) {
    let mut command_type = String::new();
    match value.as_str() {
        "echo" => command_type = "builtin".to_string(),
        "type" => command_type = "builtin".to_string(),
        "exit" => command_type = "builtin".to_string(),
        "quit" => command_type = "builtin".to_string(),
        _ => println!("{}", find_from_path(get_path(), &value)),
    }

    if command_type == "builtin" {
        println!("{} is a shell builtin", value)
    }
}

fn get_path() -> Vec<String> {
    let path_var = env::var("PATH").unwrap_or_default();
    let separators = if cfg!(windows) { ';' } else { ':' };
    path_var.split(separators).map(|s| s.to_string()).collect()
}

fn find_from_path(paths: Vec<String>, cmd: &String) -> String {
    let mut return_path: String = String::new();
    for path in paths {
        let new_path = path.to_string() + "/" + cmd;
        if !fs::exists(&new_path).unwrap_or_else(|_| false) {
            continue;
        }

        return_path = format(format_args!("{} is {}", cmd, new_path));
    }

    if return_path != String::new() {
        return return_path;
    } else {
        return format(format_args!("{}: command not found", cmd));
    }
}
