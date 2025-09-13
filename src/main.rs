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

        //Close shell 
        if command == "quit" {
            break; // exit the loop
        }

        match command{
            "cd" => {
                // default to '/' as new directory if one was not provided
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);               
            }
        },
        command => {
                let mut child = Command::new(command)
                    .args(args)
                    .spawn()
                    .unwrap();

                child.wait();            
            }
        }
    }
}
