use strum_macros::{Display, EnumIter};

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
    pub favourite: bool,
    pub path: String,
    pub card_type: CardType,
}

// pub fn load_apworlds(archipelago_dir: PathBuf) {}

pub fn load_dummy_worlds() -> Vec<ApCard> {
    [
        ApCard {
            icon: None,
            name: "dummy".to_string(),
            description: "".to_string(),
            favourite: false,
            path: "".to_string(),
            card_type: CardType::Client,
        },
        ApCard {
            icon: None,
            name: "dummy2".to_string(),
            description: "".to_string(),
            favourite: true,
            path: "".to_string(),
            card_type: CardType::Client,
        },
    ]
    .to_vec()
}
