use std::path::PathBuf;

#[derive(Clone)]
pub struct APWorld {
    pub icon: Option<String>,
    pub name: String,
    pub description: String,
    pub favourite: bool,
    pub path: String,
}

// pub fn load_apworlds(archipelago_dir: PathBuf) {}

pub fn load_dummy_worlds() -> Vec<APWorld> {
    [
        APWorld {
            icon: None,
            name: "dummy".to_string(),
            description: "".to_string(),
            favourite: false,
            path: "".to_string(),
        },
        APWorld {
            icon: None,
            name: "dummy2".to_string(),
            description: "".to_string(),
            favourite: true,
            path: "".to_string(),
        },
    ]
    .to_vec()
}
