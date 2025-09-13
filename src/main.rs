use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::env;
use std::path::Path;



fn main() {
        loop {
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!("> ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        //split input by whitespace to read command and arguments
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;



        match command {
            // Handle the built-in "cd" (change directory) command
            "cd" => {
                // If no argument is given, default to "/" (root directory).
                // Otherwise, take the first argument from `args`.
                let new_dir = args.peekable().peek().map_or("/", |x| *x);

                // Create a Path object from the new directory string
                let root = Path::new(new_dir);

                // Try to change the current working directory.
                // If it fails (e.g., directory doesn’t exist), print the error.
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            "quit" => return,       //quit terminal command

            // Handle all other commands (external programs like `ls`, `pwd`, etc.)
            command => {
                // Spawn a new child process for the given command,
                // passing along any arguments collected in `args`.
                let child = Command::new(command)
                    .args(args)
                    .spawn();
                // gracefully handle malformed user input
                match child {
                    Ok(mut child) => { child.wait(); },
                    Err(e) => eprintln!("{}", e),
                };
            }
        }

    }
}
