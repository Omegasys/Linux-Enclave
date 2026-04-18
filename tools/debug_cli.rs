use std::io::{self, Write};

fn main() {
    println!("Linux Enclave Debug CLI");
    println!("Type 'help' for commands.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let cmd = input.trim();

        match cmd {
            "help" => {
                println!("Commands:");
                println!("  help        - Show this message");
                println!("  status      - Show enclave status");
                println!("  exit        - Exit CLI");
            }
            "status" => {
                println!("[Debug] Enclave status: RUNNING (simulated)");
            }
            "exit" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
