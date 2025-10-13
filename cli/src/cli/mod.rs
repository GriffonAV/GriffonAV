use std::io::{self, Write};
use std::process::Command;

pub fn cli_sh() {
    loop {
        // Print prompt
        print!("! ");
        io::stdout().flush().unwrap();

        // Read input
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        }

        // Trim spaces & check for empty input
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Exit shell
        if input == "exit" || input == "quit" {
            break;
        }

        // Split input into command + args
        let parts: Vec<&str> = input.split_whitespace().collect();
        let (cmd, args) = parts.split_first().unwrap();

        // Run command
        match Command::new(cmd).args(args).spawn() {
            Ok(mut child) => {
                let _ = child.wait();
            }
            Err(e) => {
                eprintln!("Error running {}: {}", cmd, e);
            }
        }
    }
}
