//! These cards are default to rustipelago and will always exist.
//! Unlike most cards, these have their own native rust implementation where as most others just use the official python implementations.
//!
//! Additionally, in order to update these cards, the whole application needs to be updated.

use crate::{ApCard, CardType};

pub fn get_default_cards() -> [ApCard; 2] {
    [
        ApCard {
            icon: None,
            name: "Install APWorld".to_string(),
            description:
                "Install an APWorld to play games not included with archipelago by default."
                    .to_string(),
            path: String::default(),
            card_type: CardType::Tool,
        },
        ApCard {
            icon: None,
            name: "Slot Manager".to_string(),
            description: "Manage all of the \"local\" slots. (need better description)".to_string(),
            path: String::default(),
            card_type: CardType::Tool,
        },
    ]
}
