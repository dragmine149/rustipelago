use std::sync::mpsc::{Receiver, Sender};

use crate::update::check_launcher_update;
use rustipelago_bridge::{
    MessageHandler,
    messages::{MessageToBackend, MessageToFrontend},
};
pub mod apworld;
pub mod install;
pub mod update;

pub struct BackendState {
    sender: Sender<MessageToFrontend>,
}

impl MessageHandler<MessageToFrontend, MessageToBackend> for BackendState {
    async fn start(self, receiver: Receiver<MessageToBackend>) {
        self.handle(receiver, async |msg| match msg {
            MessageToBackend::CheckLauncherUpdate => {
                let update = check_launcher_update().await;
                let _ = match update {
                    Ok(version) => self.sender.send(MessageToFrontend::LauncherUpdate {
                        new_version: version,
                    }),
                    Err(error) => self.sender.send(MessageToFrontend::ReqwestFailed {
                        url: "http://rustipelago.dragmine.me/version.json".to_string(),
                        error,
                    }),
                };
            }
        })
        .await;
    }

    fn new(sender: Sender<MessageToFrontend>) -> Self {
        Self { sender }
    }
}
