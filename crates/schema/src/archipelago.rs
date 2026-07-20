use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, EnumIter, Display, PartialEq, Eq, Default)]
pub enum CardType {
    /// Defines a world used to communicate with the server.
    Client,
    /// Defines a world used to modify / generate local files.
    Tool,
    /// Defines a world which requires adjusting the game itself due to lack of mod support.
    Adjuster,
    /// Anything which doesn't fit into the above.
    #[default]
    Misc,
}

/// Information about a specific card to show in the launcher
#[derive(Debug, Clone)]
pub struct ApCard {
    pub icon: Option<String>,
    pub name: String,
    pub description: String,
    /// Installed python cards are handled differently.
    /// If this exists, this is a link to the world itself as that is most likely different from the name.
    pub python: Option<PathBuf>,
    pub card_type: CardType,
}

impl Default for ApCard {
    fn default() -> Self {
        Self {
            icon: None,
            name: String::from("Unknown card"),
            description: String::default(),
            python: None,
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

/// Internal enum of default cards (non-python). Helps us make sure everything is accounted for.
pub enum DefaultCards {
    InstallApWorld,
    EditApWorld,
    SlotManager,
}
impl TryFrom<String> for DefaultCards {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Install ApWorld" => Ok(DefaultCards::InstallApWorld),
            "Slot Manager" => Ok(DefaultCards::SlotManager),
            "Edit ApWorld" => Ok(DefaultCards::EditApWorld),
            _ => Err(value),
        }
    }
}
impl From<DefaultCards> for String {
    fn from(value: DefaultCards) -> Self {
        match value {
            DefaultCards::InstallApWorld => "Install ApWorld",
            DefaultCards::SlotManager => "Slot Manager",
            DefaultCards::EditApWorld => "Edit ApWorld",
        }
        .to_string()
    }
}

/// `archipelago.json` specs as defined by https://github.com/ArchipelagoMW/Archipelago/blob/main/docs/apworld%20specification.md
#[derive(Debug, Serialize, Deserialize)]
pub struct ApWorldInfo {
    /// Minimum version where this world will run. Any version before this won't run this world.
    pub minimum_ap_version: Option<Version>,
    /// Maximum version where this world will run. Any version after this won't run this world. (rarley used)
    pub maximum_ap_version: Option<Version>,
    /// Internal version of the world itself.
    pub world_version: Option<Version>,
    /// Authors who made the world.
    pub authors: Option<Vec<String>>,
    /// The game name the world is related to.
    pub game: String,
}
