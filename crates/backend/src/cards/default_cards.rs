//! These cards are default to rustipelago and will always exist.
//! Unlike most cards, these have their own native rust implementation where as most others just use the official python implementations.
//!
//! Additionally, in order to update these cards, the whole application needs to be updated.

use rustipelago_schema::archipelago::{ApCard, CardType};

pub(crate) fn get_default_cards() -> [ApCard; 2] {
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

pub(crate) fn load_dummy_worlds() -> Vec<ApCard> {
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
