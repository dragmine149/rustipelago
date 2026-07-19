use crate::messages::{MessageToBackend, MessageToFrontend};
use std::{
    fmt::Display,
    sync::mpsc::{Receiver, Sender},
};
use tokio::runtime::Runtime;

pub mod messages;

/// Helper function to reduce *slightly* the excess use of handling.
pub trait MessageHandler<S, R> {
    /// Written by T3 Chat (Kimi K2.5)
    fn handle<H>(&mut self, receiver: Receiver<R>, handler: H)
    where
        R: Display,
        H: Fn(&mut Self, R) -> (),
    {
        while let Ok(msg) = receiver.recv() {
            println!("{msg}");
            handler(self, msg);
        }
    }

    fn setup(runtime: Runtime, sender: Sender<S>, receiver: Receiver<R>)
    where
        Self: Sized,
    {
        let mut state = Self::new(sender);
        runtime.block_on(async move { state.start(receiver) });
    }
    fn start(&mut self, receiver: Receiver<R>);

    fn new(sender: Sender<S>) -> Self;
}

pub fn create_pairs() -> (
    (Sender<MessageToBackend>, Receiver<MessageToBackend>),
    (Sender<MessageToFrontend>, Receiver<MessageToFrontend>),
) {
    let frontend = std::sync::mpsc::channel();
    let backend = std::sync::mpsc::channel();

    (backend, frontend)
}
