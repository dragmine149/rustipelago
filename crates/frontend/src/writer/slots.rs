use crate::writer::{Save, Writer};
use gpui::Global;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Slot {
    pub server: String,
    pub name: String,
    pub alias: String,
    pub completion: [u64; 2],
    pub accessed: usize,
}

impl Slot {
    pub fn get_completion_percent(&self) -> f64 {
        self.completion[0] as f64 / self.completion[1] as f64
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Slots {
    pub slots: Vec<Slot>,
}

// impl Global for Slots {}
impl Writer for Slots {
    fn get_name() -> &'static str {
        "Slots"
    }
}

impl Save for Slots {
    fn pre_save(&mut self) {}
}
