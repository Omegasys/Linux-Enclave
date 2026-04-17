mod core;
mod crypto;

use core::enclave_runtime::EnclaveRuntime;

fn main() {
    println!("[Linux Enclave] Starting runtime...");

    let mut runtime = EnclaveRuntime::new();

    runtime.init();

    let task = b"hello secure enclave task";

    match runtime.execute(task.to_vec()) {
        Ok(result) => {
            println!("[Enclave Result] {:?}", result);
        }
        Err(e) => {
            eprintln!("[Enclave Error] {}", e);
        }
    }

    runtime.shutdown();

    println!("[Linux Enclave] Shutdown complete.");
}
