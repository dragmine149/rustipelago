use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        mpsc::{Receiver, Sender},
    },
};

use strum_macros::Display;

// use tokio::sync::mpsc::{Receiver, UnboundedReceiver, UnboundedSender};

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

pub enum MessageToCards {}

pub struct BackendSender {
    sender: Sender<MessageToBackend>,
}
pub struct FrontendSender {
    sender: Sender<MessageToFrontend>,
}

pub struct BackendReceiver {
    pub receiver: Receiver<MessageToBackend>,
}
pub struct FrontendReceiver {
    pub receiver: Receiver<MessageToFrontend>,
}

pub fn create_pairs() -> (
    (BackendSender, BackendReceiver),
    (FrontendSender, FrontendReceiver),
) {
    let (frontend_send, frontend_recv) = std::sync::mpsc::channel();
    let (backend_send, backend_recv) = std::sync::mpsc::channel();

    (
        (
            BackendSender {
                sender: backend_send,
            },
            BackendReceiver {
                receiver: backend_recv,
            },
        ),
        (
            FrontendSender {
                sender: frontend_send,
            },
            FrontendReceiver {
                receiver: frontend_recv,
            },
        ),
    )
}

impl BackendSender {
    pub fn send(
        &self,
        message: MessageToBackend,
    ) -> Result<(), std::sync::mpsc::SendError<MessageToBackend>> {
        self.sender.send(message)
    }
}
impl FrontendSender {
    pub fn send(
        &self,
        message: MessageToFrontend,
    ) -> Result<(), std::sync::mpsc::SendError<MessageToFrontend>> {
        self.sender.send(message)
    }
}
