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

    rustipelago_frontend::main(config_dir, internal_dir);
}
