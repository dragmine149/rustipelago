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
}

#[derive(Display)]
pub enum MessageToBackend {
    CheckLauncherUpdate,
}

#[derive(Display)]
pub enum MessageToCards {}
