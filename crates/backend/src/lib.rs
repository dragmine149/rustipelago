use crate::{
    apworld::{list_worlds, mount_world, unmount_world},
    cards::default_cards::{get_default_cards, load_dummy_worlds},
    update::check_launcher_update,
};
use rustipelago_bridge::{
    MessageHandler,
    messages::{MessageToBackend, MessageToFrontend},
};
use rustipelago_schema::archipelago::{ApCard, DefaultCards};
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
                    let update = check_launcher_update().await;
                    let _ = match update {
                        Ok(version) => sender.send(MessageToFrontend::LauncherUpdate {
                            new_version: version,
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
                    true => {
                        this.mount_world(&card);
                    }
                    false => match DefaultCards::try_from(card.name.clone()).unwrap() {
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
        cards.extend(load_dummy_worlds());
        cards.extend(list_worlds(&self.get_world_dir()));
        let _ = self.sender.send(MessageToFrontend::CardsLoaded { cards });
    }
    fn get_world_dir(&self) -> PathBuf {
        self.archipelago_dir.join("worlds")
    }
    fn mount_world(&mut self, card: &ApCard) {
        let world_file = self.get_world_dir().join(format!("{}.apworld", card.name));
        let world = match mount_world(&world_file, false) {
            Ok(w) => w,
            Err(e) => {
                _ = self.sender.send(MessageToFrontend::ReadFailed {
                    path: world_file,
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
