use crate::writer::{Save, Writer};
use gpui::SharedString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub stream_url: Vec<String>,
    pub active_theme: SharedString,
    pub debug_tick: bool,
}

impl Writer for Config {
    fn get_name() -> &'static str {
        "Config"
    }
}

impl Save for Config {
    fn pre_save(&mut self) {
        self.stream_url.retain(|x| !x.is_empty());
    }
}
