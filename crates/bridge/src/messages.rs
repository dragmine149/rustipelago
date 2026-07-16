use rustipelago_schema::archipelago::ApCard;
use std::{path::Path, sync::Arc};
use strum_macros::Display;

#[derive(Display)]
pub enum MessageToFrontend {
    ReadFailed {
        path: Arc<Path>,
        error: anyhow::Error,
    },
    ReqwestFailed {
        url: String,
        error: anyhow::Error,
    },
    LauncherUpdate {
        new_version: Option<String>,
    },
    CardsLoaded {
        cards: Vec<ApCard>,
    },
}

#[derive(Display)]
pub enum MessageToBackend {
    CheckLauncherUpdate,
    OpenCard { card_name: String },
    FetchCards,
}
