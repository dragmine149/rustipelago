use crate::{
    apworld::{list_worlds, mount_world, unmount_world},
    cards::default_cards::get_default_cards,
    update::get_server_versions,
};
use rustipelago_bridge::{
    MessageHandler,
    messages::{MessageToBackend, MessageToFrontend},
};
use rustipelago_schema::archipelago::{ApCard, DefaultCards};
use semver::Version;
use std::{
    path::PathBuf,
    sync::mpsc::{Receiver, Sender},
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
    mounted_worlds: Vec<(PathBuf, String)>,
}

impl MessageHandler<MessageToFrontend, MessageToBackend> for BackendState {
    fn start(&mut self, receiver: Receiver<MessageToBackend>) {
        self.handle(receiver, |this, msg| match msg {
            MessageToBackend::CheckLauncherUpdate => {
                let sender = this.sender.clone();
                tokio::spawn(async move {
                    let update = get_server_versions().await;
                    let _ = match update {
                        Ok(version) => sender.send(MessageToFrontend::LauncherUpdate {
                            new_version: (Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
                                != version.launcher)
                                .then_some(version.launcher),
                        }),
                        Err(error) => sender.send(MessageToFrontend::ReqwestFailed {
                            url: "http://rustipelago.dragmine.me/version.json".to_string(),
                            error,
                        }),
                    };
                });
            }
            MessageToBackend::FetchCards => this.load_cards(),
            MessageToBackend::OpenCard { card } => {
                println!("Opening card {card:?}");
                match card.python {
                    Some(_) => {
                        this.mount_world(&card);
                    }
                    None => match DefaultCards::try_from(card.name.clone()).unwrap() {
                        DefaultCards::InstallApWorld => {
                            _ = this.sender.send(MessageToFrontend::UserInput { card });
                        }
                        DefaultCards::EditApWorld => {
                            _ = this.sender.send(MessageToFrontend::UserInput { card });
                        }
                        DefaultCards::SlotManager => todo!(),
                    },
                }
            }
            MessageToBackend::EditWorld { card } => this.mount_world(&card),
        });
    }

    fn new(sender: Sender<MessageToFrontend>) -> Self {
        Self {
            sender,
            cards: vec![],
            archipelago_dir: dirs::data_dir()
                .expect("Failed to find data dir.")
                .join("Archipelago"),
            mounted_worlds: vec![],
        }
    }
}

impl BackendState {
    fn load_cards(&self) {
        let mut cards = vec![];
        cards.extend(get_default_cards());
        cards.extend(list_worlds(&self.get_world_dir()));
        let _ = self.sender.send(MessageToFrontend::CardsLoaded { cards });
    }
    fn get_world_dir(&self) -> PathBuf {
        self.archipelago_dir.join("worlds")
    }
    /// Mount a world so that we can run files easier.
    ///
    /// TEST: Can we read it mostly from memory?
    fn mount_world(&mut self, card: &ApCard) {
        let path = card.python.as_ref().unwrap();
        // because we're "mounting" it, we assume its already python.
        let world = match mount_world(&path, false) {
            Ok(w) => w,
            Err(e) => {
                _ = self.sender.send(MessageToFrontend::ReadFailed {
                    path: path.clone(),
                    error: e,
                });
                return;
            }
        };
        self.mounted_worlds.push((world, card.name.to_owned()));
    }
}

impl Drop for BackendState {
    fn drop(&mut self) {
        // Extra cleanup if it didn't happen earlier.
        println!("worlds: {:?}", self.mounted_worlds);
        for world in &self.mounted_worlds {
            println!("Unmounting {} due to finished with it", world.0.display());
            _ = unmount_world(&self.get_world_dir(), &world.0, &world.1);
        }
    }
}
