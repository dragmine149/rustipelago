//! Main file and main part of the workspace.
//!
//! Just links everything together and gives a starting place.
use rustipelago_backend::BackendState;
use rustipelago_bridge::{MessageHandler, create_pairs};

pub fn main() {
    // we can... just "borrow" that dir as all the config is there anyway.
    let config_dir = dirs::data_dir()
        .expect("Failed to find data dir.")
        .join("Archipelago");
    // Internal dir is where we store files that are outside of the normal client usage.
    let internal_dir = config_dir.join(".rustipelago");
    if !internal_dir.exists() {
        let _ = std::fs::create_dir_all(&internal_dir);
    }

    let pairs = create_pairs();
    let frontend_sender = pairs.1.0.clone();

    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().expect("Failed to initialize tokio runtime");
        println!("Starting backend");
        BackendState::setup(runtime, frontend_sender, pairs.0.1);
    });
    println!("Starting frontend");
    rustipelago_frontend::main(config_dir, internal_dir, pairs.1.1, pairs.0.0);
}
