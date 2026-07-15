use rustipelago_bridge::create_pairs;

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

    // let runtime = tokio::runtime::Builder::new_multi_thread()
    //     .worker_threads(1)
    //     .enable_all()
    //     .build()
    //     .expect("Failed to initialize Tokio runtime");
    // let runtime = tokio::runtime::Builder::new_current_thread()
    //     .build()
    //     .expect("Failed to initialize Tokio runtime");
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().expect("Failed to initialize tokio runtime");
        // runtime.enter();

        runtime.spawn(async {
            println!("e");
        });

        println!("Starting backend");
        rustipelago_backend::start(runtime, pairs.1.0, pairs.0.1);
    });
    println!("Starting frontend");
    rustipelago_frontend::main(config_dir, internal_dir, pairs.1.1, pairs.0.0);
}
