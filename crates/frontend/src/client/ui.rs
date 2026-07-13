use gpui::{Context, IntoElement, Render, Window, div};

use crate::GPUIStructHelper;

pub struct ClientUI {
    chat: Vec<String>,
}

impl GPUIStructHelper for ClientUI {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self { chat: vec![] }
    }
}

impl Render for ClientUI {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
