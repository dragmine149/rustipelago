use std::sync::Arc;

use rustipelago_bridge::{BackendReceiver, BackendSender, FrontendSender, MessageToFrontend};
use tokio::runtime::Runtime;

use crate::update::check_launcher_update;

pub mod apworld;
pub mod install;
pub mod update;

pub struct BackendState {
    sender: FrontendSender,
}

pub fn start(
    runtime: Runtime,
    frontend_handler: FrontendSender,
    backend_receiver: BackendReceiver,
) {
    let state = BackendState {
        sender: frontend_handler,
    };

    runtime.block_on(async { state.start(backend_receiver).await });
    println!("Spawn done");
}

impl BackendState {
    async fn start(self, receiver: BackendReceiver) {
        println!("Starting loop on backend");
        self.handle(receiver).await;
    }

    async fn handle(self, receiver: BackendReceiver) {
        while let Ok(msg) = receiver.receiver.recv() {
            println!("{msg}");
            match msg {
                rustipelago_bridge::MessageToBackend::CheckLauncherUpdate => {
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
            }
        }
    }
}
