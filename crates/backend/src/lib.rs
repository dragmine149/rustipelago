use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        mpsc::{Receiver, Sender},
    },
};

use crate::{
    cards::default_cards::{get_default_cards, load_dummy_worlds},
    update::check_launcher_update,
};
use rustipelago_bridge::{
    MessageHandler,
    messages::{MessageToBackend, MessageToFrontend},
};
use tokio::task::JoinHandle;
pub mod apworld;
pub(crate) mod cards;
pub mod install;
pub mod update;

pub struct BackendState {
    sender: Sender<MessageToFrontend>,
    cards: Vec<JoinHandle<()>>,
    archipelago_dir: PathBuf,
}

impl MessageHandler<MessageToFrontend, MessageToBackend> for BackendState {
    fn start(self, receiver: Receiver<MessageToBackend>) {
        self.handle(receiver, async |this, msg| match msg {
            MessageToBackend::CheckLauncherUpdate => {
                let update = check_launcher_update().await;
                let _ = match update {
                    Ok(version) => this.sender.send(MessageToFrontend::LauncherUpdate {
                        new_version: version,
                    }),
                    Err(error) => this.sender.send(MessageToFrontend::ReqwestFailed {
                        url: "http://rustipelago.dragmine.me/version.json".to_string(),
                        error,
                    }),
                };
            }
            MessageToBackend::FetchCards => this.load_cards(),
            MessageToBackend::OpenCard { card_name } => println!("Opening card {card_name}"),
        });
    }

    fn new(sender: Sender<MessageToFrontend>) -> Self {
        Self {
            sender,
            cards: vec![],
            archipelago_dir: dirs::data_dir()
                .expect("Failed to find data dir.")
                .join("Archipelago"),
        }
    }
}

impl BackendState {
    fn load_cards(&self) {
        let mut cards = vec![];
        cards.extend(get_default_cards());
        cards.extend(load_dummy_worlds());
        let _ = self.sender.send(MessageToFrontend::CardsLoaded { cards });
    }
}
