use crate::default_cards::get_default_cards;
use rustipelago_bridge::{
    MessageHandler,
    messages::{MessageToCards, MessageToFrontend},
};
use rustipelago_schema::archipelago::{ApCard, CardType};
use std::{
    path::PathBuf,
    sync::mpsc::{Receiver, Sender},
    thread::JoinHandle,
};
use tokio::runtime::Runtime;

pub mod cards;
pub mod default_cards;

pub struct CardData {
    opened_cards: Vec<JoinHandle<()>>,
    sender: Sender<MessageToFrontend>,
}
impl MessageHandler<MessageToFrontend, MessageToCards> for CardData {
    async fn start(self, receiver: Receiver<MessageToCards>) {
        self.handle(receiver, async |msg| match msg {}).await;
    }

    fn new(sender: Sender<MessageToFrontend>) -> Self {
        Self {
            opened_cards: vec![],
            sender,
        }
    }
}

pub fn load_apworlds(archipelago_dir: PathBuf) -> Vec<ApCard> {
    let mut worlds = vec![];
    worlds.extend(get_default_cards());
    worlds.extend(load_dummy_worlds());
    worlds
}

// fn load_installed_apworlds(archipelago_dir: PathBuf) {
//     let world_folder = archipelago_dir.join("worlds");

//     let walkdir = walkdir::WalkDir::new(world_folder);
//     for world in walkdir.into_iter() {
//         if let Err(e) = world {
//             MessageToFrontend::ReadFailed {
//                 path: (),
//                 error: (),
//             }
//         }
//     }
// }

pub fn load_dummy_worlds() -> Vec<ApCard> {
    [
        ApCard {
            icon: None,
            name: "dummy".to_string(),
            description: "".to_string(),
            path: "".to_string(),
            card_type: CardType::Client,
        },
        ApCard {
            icon: None,
            name: "dummy2".to_string(),
            description: "".to_string(),
            path: "".to_string(),
            card_type: CardType::Client,
        },
    ]
    .to_vec()
}
