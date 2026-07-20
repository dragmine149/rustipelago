use rustipelago_schema::archipelago::ApCard;
use semver::Version;
use std::path::PathBuf;
use strum_macros::Display;

#[derive(Display)]
pub enum MessageToFrontend {
    ReadFailed { path: PathBuf, error: anyhow::Error },
    ReqwestFailed { url: String, error: anyhow::Error },
    LauncherUpdate { new_version: Option<Version> },
    CardsLoaded { cards: Vec<ApCard> },
    UserInput { card: ApCard },
}

#[derive(Display)]
pub enum MessageToBackend {
    CheckLauncherUpdate,
    OpenCard { card: ApCard },
    FetchCards,
    EditWorld { card: ApCard },
}
