use crate::writer::{Save, Writer};
use rustipelago_schema::archipelago::Slot;
use serde::{Deserialize, Serialize};

/// Stores information about all the slots.
///
/// The official app doesn't do this, hence we have to do it ourselves. as such can save it however.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Slots {
    pub slots: Vec<Slot>,
}

impl Writer for Slots {
    fn get_name() -> &'static str {
        "Slots"
    }
}

impl Save for Slots {
    fn pre_save(&mut self) {}
}
