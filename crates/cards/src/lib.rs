use crate::default_cards::get_default_cards;
use std::path::PathBuf;
use strum_macros::{Display, EnumIter};

pub mod cards;
pub mod default_cards;

#[derive(Clone, EnumIter, Display, PartialEq, Eq)]
pub enum CardType {
    Client,
    Tool,
    Adjuster,
    World,
    Misc,
}

#[derive(Clone)]
pub struct ApCard {
    pub icon: Option<String>,
    pub name: String,
    pub description: String,
    pub path: String,
    pub card_type: CardType,
}

pub fn load_apworlds(archipelago_dir: PathBuf) -> Vec<ApCard> {
    let mut worlds = vec![];
}

fn load_installed_apworlds(archipelago_dir: PathBuf) {
    let world_folder = archipelago_dir.join("worlds");

    let walkdir = walkdir::WalkDir::new(world_folder);
    for world in walkdir.into_iter() {
        let Ok(world) = world else {
            eprintln!("Failed to load apworld {}", world);
            continue;
        };
    }
}

pub fn load_dummy_worlds() -> Vec<ApCard> {
    [
        ApCard {
            icon: None,
            name: "dummy".to_string(),
            description: "".to_string(),
            path: "".to_string(),
            card_type: CardType::Client,
        },
        ApCard {
            icon: None,
            name: "dummy2".to_string(),
            description: "".to_string(),
            path: "".to_string(),
            card_type: CardType::Client,
        },
    ]
    .to_vec()
}
