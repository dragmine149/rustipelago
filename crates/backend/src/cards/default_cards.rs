//! These cards are default to rustipelago and will always exist.
//! Unlike most cards, these have their own native rust implementation where as most others just use the official python implementations.
//!
//! Additionally, in order to update these cards, the whole application needs to be updated.

use rustipelago_schema::archipelago::{ApCard, CardType, DefaultCards};

pub(crate) fn get_default_cards() -> [ApCard; 3] {
    [
        ApCard {
            icon: None,
            name: String::from(DefaultCards::InstallApWorld),
            description:
                "Install an APWorld to play games not included with archipelago by default."
                    .to_string(),
            python: None,
            card_type: CardType::Tool,
        },
        ApCard {
            icon: None,
            name: String::from(DefaultCards::EditApWorld),
            description: "Edit an APWorld locally installed".to_string(),
            python: None,
            card_type: CardType::Tool,
        },
        ApCard {
            icon: None,
            name: String::from(DefaultCards::SlotManager),
            description: "Manage all of the \"local\" slots. (need better description)".to_string(),
            python: None,
            card_type: CardType::Tool,
        },
    ]
}
