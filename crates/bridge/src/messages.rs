use rustipelago_schema::archipelago::ApCard;
use std::path::PathBuf;
use strum_macros::Display;

#[derive(Display)]
pub enum MessageToFrontend {
    ReadFailed { path: PathBuf, error: anyhow::Error },
    ReqwestFailed { url: String, error: anyhow::Error },
    LauncherUpdate { new_version: Option<String> },
    CardsLoaded { cards: Vec<ApCard> },
    UserInput { card: ApCard },
}

#[derive(Display)]
pub enum MessageToBackend {
    CheckLauncherUpdate,
    OpenCard { card: ApCard },
    FetchCards,
}
