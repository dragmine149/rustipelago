use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, EnumIter, Display, PartialEq, Eq)]
pub enum CardType {
    Client,
    Tool,
    Adjuster,
    World,
    Misc,
}

#[derive(Debug, Clone)]
pub struct ApCard {
    pub icon: Option<String>,
    pub name: String,
    pub description: String,
    pub python: bool,
    pub card_type: CardType,
}

impl Default for ApCard {
    fn default() -> Self {
        Self {
            icon: None,
            name: String::from("Unknown card"),
            description: String::default(),
            python: false,
            card_type: CardType::Misc,
        }
    }
}

/// Information about a given slot.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Slot {
    /// The server address and port number for said slot.
    pub server: String,
    pub name: String,
    pub alias: String,
    /// Completion of the slot, stored as [checks, total]
    pub completion: [u64; 2],
    /// Last time we accessed said slot. Only client-sided though.
    pub accessed: usize,
}

impl Slot {
    /// Return the completion as a percentage.
    pub fn get_completion_percent(&self) -> f64 {
        self.completion[0] as f64 / self.completion[1] as f64
    }
}
