pub enum LogLevel {
    Info,
    Warn,
    Error,
}

pub fn log(level: LogLevel, msg: &str) {
    match level {
        LogLevel::Info => println!("[INFO] {}", msg),
        LogLevel::Warn => println!("[WARN] {}", msg),
        LogLevel::Error => eprintln!("[ERROR] {}", msg),
    }
}
