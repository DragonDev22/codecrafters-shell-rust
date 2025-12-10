[![progress-banner](https://backend.codecrafters.io/progress/shell/0dfb4d1b-9f97-49ca-9acd-07b5d746561b)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is a starting point for Rust solutions to the
["Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview).

In this challenge, you'll build your own POSIX compliant shell that's capable of
interpreting shell commands, running external programs and builtin commands like
cd, pwd, echo and more. Along the way, you'll learn about shell command parsing,
REPLs, builtin commands, and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

# Command / Feature List:
## Implemented:
- `echo` - Prints the args passed after it.
- `exit` & `quit` - Exits the shell.
- `type` - Prints if the shell is a builtin command, and if not attempts to find the executable by searching through the `PATH` environment variable.
- `pwd` - Prints the current working directory.
- `cd` - Changes the directory. Currently supports absolute paths (e.g. `/home/user/downloads`), and relative paths (e.g. `../` for the parent directory, `./` for the current directory, and `~/` for the user's home directory).
- Upon another command being entered, the shell searches the `PATH` environment variable to find the executable. If found it will execute it with the args following the command.
- Basic errors.

## Planned:
- `$VARIABLE` notation to get environment variables.
- special characters parsing for `echo` (e.g. `echo 'hello\nworld!'` will return `hello\nworld!` without adding a newline).
- plus more!
