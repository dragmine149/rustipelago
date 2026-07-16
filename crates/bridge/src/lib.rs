use tokio::runtime::Runtime;

use crate::messages::{MessageToBackend, MessageToCards, MessageToFrontend};
use std::{
    fmt::Display,
    sync::mpsc::{Receiver, Sender},
};

pub mod messages;

/// Helper function to reduce *slightly* the excess use of handling.
///
/// Due to this not really being a library, i count it under the `if you plan to use the trait only in your own code` section.
/// If someone has a better way, fell free to submit a PR for it.
#[allow(async_fn_in_trait)]
pub trait MessageHandler<S, R> {
    async fn handle<H>(&self, receiver: Receiver<R>, handler: H)
    where
        R: Display,
        H: AsyncFn(R) -> (),
    {
        while let Ok(msg) = receiver.recv() {
            println!("{msg}");
            handler(msg).await
        }
    }

    fn setup(runtime: Runtime, sender: Sender<S>, receiver: Receiver<R>)
    where
        Self: Sized,
    {
        let state = Self::new(sender);
        runtime.block_on(async move { state.start(receiver).await })
    }
    async fn start(self, receiver: Receiver<R>);

    fn new(sender: Sender<S>) -> Self;
}

pub fn create_pairs() -> (
    (Sender<MessageToBackend>, Receiver<MessageToBackend>),
    (Sender<MessageToFrontend>, Receiver<MessageToFrontend>),
    (Sender<MessageToCards>, Receiver<MessageToCards>),
) {
    let frontend = std::sync::mpsc::channel();
    let backend = std::sync::mpsc::channel();
    let cards = std::sync::mpsc::channel();

    (backend, frontend, cards)
}
