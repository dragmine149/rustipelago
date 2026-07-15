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
    pub path: String,
    pub card_type: CardType,
}

impl Default for ApCard {
    fn default() -> Self {
        Self {
            icon: None,
            name: String::from("Unknown card"),
            description: String::default(),
            path: String::default(),
            card_type: CardType::Misc,
        }
    }
}
