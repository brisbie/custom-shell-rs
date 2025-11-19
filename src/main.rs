use std::io::{stdin, stdout, Write};
use std::process::{Command, Stdio, Child};
use std::env;
use std::path::Path;
use colored::*;

fn main() {
    loop {
        //Get the current working directory 
        let cwd = env::current_dir().unwrap(); // returns a PathBuf
        let cwd_str = cwd.to_str().unwrap();   // convert to &str
        // Print a shell prompt
        print!("{}{} ", cwd_str.yellow(), " > ".green().bold());
        stdout().flush().unwrap(); // flush to ensure prompt appears immediately


        // Read a line of input from the user
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // Split input into commands separated by pipes ("|"),
        // and make it peekable so we can check if more commands follow.
        let mut commands = input.trim().split(" | ").peekable();

        // Will hold the last executed command's child process (for piping)
        let mut previous_command = None;

        // Process each command in the pipeline
        while let Some(command) = commands.next() {
            // Split command into the program and its arguments
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap(); // the program itself (e.g. "ls")
            let args = parts;                    // the rest are arguments (e.g. "-la")

            match command {
                // Handle built-in "cd" command
                "cd" => {
                    // Use the provided path or default to "/"
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);

                    // Attempt to change directory, print error if it fails
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    // Reset pipeline since "cd" doesn’t produce output
                    previous_command = None;
                },

                // Handle built-in "quit" or "exit" command to exit shell
                "quit" => return,
                "exit" => return,

                // For all other commands, attempt to execute them
                command => {
                    // If there's a previous command, connect its stdout to this command's stdin.
                    // Otherwise, inherit stdin from the shell.
                    let stdin = previous_command
                        .map_or(Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap()));

                    // If there is another command after this one, pipe the output forward.
                    // If this is the last command, send its output to the shell stdout.
                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    // Spawn the process
                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    // Save the child process handle for piping,
                    // or print an error if the process couldn’t be started.
                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }

        // Wait for the last command in the pipeline to finish before showing a new prompt
        if let Some(mut final_command) = previous_command {
            final_command.wait().unwrap();
        }
    }
}
