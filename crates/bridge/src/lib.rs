use crate::messages::{MessageToBackend, MessageToFrontend};
use std::{
    fmt::Display,
    sync::{
        Arc,
        mpsc::{Receiver, Sender},
    },
};
use tokio::runtime::Runtime;

pub mod messages;

/// Helper function to reduce *slightly* the excess use of handling.
pub trait MessageHandler<S, R> {
    /// Written by T3 Chat (Kimi K2.5)
    fn handle<H, Fut>(self, receiver: Receiver<R>, handler: H)
    where
        R: Display + Send + Sync + 'static,
        H: Fn(Arc<Self>, R) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
        Self: Send + Sync + 'static + Sized,
    {
        let this = Arc::new(self);

        while let Ok(msg) = receiver.recv() {
            println!("{msg}");

            let this_clone = Arc::clone(&this);
            let handler_clone = handler.clone();

            tokio::spawn(async move {
                handler_clone(this_clone, msg).await;
            });
        }
    }

    fn setup(runtime: Runtime, sender: Sender<S>, receiver: Receiver<R>)
    where
        Self: Sized,
    {
        let state = Self::new(sender);
        runtime.block_on(async move { state.start(receiver) });
    }
    fn start(self, receiver: Receiver<R>);

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
