use std::fs;

fn main() {
    println!("[Inspector] Inspecting enclave state...");

    let config_path = "configs/default.toml";

    match fs::read_to_string(config_path) {
        Ok(contents) => {
            println!("[Inspector] Loaded config:");
            println!("{}", contents);
        }
        Err(e) => {
            eprintln!("[Inspector] Failed to read config: {}", e);
        }
    }

    println!("[Inspector] Inspection complete.");
}
