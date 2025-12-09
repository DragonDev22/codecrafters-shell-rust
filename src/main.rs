#![allow(unused_imports, reason = "This is a learning project")]
use faccess::PathExt;
use std::env::{self, Args};
use std::ffi::OsStr;
use std::io::Error;
use std::process::Output;
use std::{
    fmt::format,
    fs::{self},
    io::{self, Write},
    path::Path,
    process::Command,
};

enum Instruction {
    Builtin(Builtin),
    Other(String, String),
}

enum Builtin {
    Echo(String),
    Type(String),
    Exit,
}

fn main() {
    let mut exit: bool = false;

    env::set_current_dir(Path::new("/")).unwrap();

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
    command.trim().to_string()
}

fn evaluate(command: &str) -> Instruction {
    let first_word = first_word(command);

    match first_word {
        "exit" => Instruction::Builtin(Builtin::Exit),
        "quit" => Instruction::Builtin(Builtin::Exit),
        "echo" => Instruction::Builtin(Builtin::Echo(
            command.split_once(" ").unwrap().1.to_string(),
        )),
        "type" => Instruction::Builtin(Builtin::Type(
            command.split_once(" ").unwrap().1.to_string(),
        )),
        _ => {
            let args = command.split_at(first_word.len()).1;
            Instruction::Other(first_word.to_string(), args.to_string())
        }
    }
}

fn execute(task: Instruction) -> bool {
    let mut exit = false;
    match task {
        Instruction::Builtin(type_val) => match type_val {
            Builtin::Echo(args) => println!("{}", args),
            Builtin::Exit => exit = true,
            Builtin::Type(args) => get_type(first_word(args.as_str()).to_string()),
        },
        Instruction::Other(cmd, args) => run_external(
            find_from_path(get_path(), &cmd),
            args.split_terminator(' ').collect::<Vec<&str>>(),
        ),
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
        cmd => match find_from_path(get_path(), &cmd.to_string()).as_str() {
            "" => println!("{}: not found", cmd),
            path => println!("{} is {}", cmd, path),
        },
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
    let mut return_paths: Vec<String> = Vec::new();
    for path in paths {
        let new_path = path.to_string() + "/" + cmd;
        if !fs::exists(&new_path).unwrap_or(false) || !Path::new(&new_path).executable() {
            continue;
        }

        return_paths.push(new_path);
    }

    if !return_paths.is_empty() {
        let mut return_string: String = String::new();

        return_string += return_paths[0].as_str();

        return_string
    } else {
        return String::new();
    }
}

fn run_external<I: IntoIterator + std::fmt::Debug>(path: String, args: I)
where
    <I as IntoIterator>::Item: AsRef<OsStr>,
{
    println!("running program at: {} with args: {:#?}", &path, &args);
    let output_raw = run(&path, args);
    let output;
    if output_raw.is_err() {
        println!("failed to run command at: {}", path);
        println!("returned error: {:?}", output_raw.err());
        return;
    } else {
        output = output_raw.ok().unwrap();
    }

    println!("{}", String::from_utf8_lossy(&output.stdout));
    let status = output.status;
    if !status.success() {
        println!("Exited with status: {}", status);
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

fn run<I: IntoIterator>(path: &str, args: I) -> Result<Output, Error>
where
    <I as IntoIterator>::Item: AsRef<OsStr>,
{
    Command::new(Path::new(&path)).args(args).output()
}
