use std::{collections::HashMap, sync::mpsc::Sender};

use crate::{GPUIStructHelper, cards::CardHandler};
use gpui::{
    Context, InteractiveElement, IntoElement, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder,
};
use gpui_component::{button::Button, h_flex, scroll::ScrollableElement};
use rustipelago_bridge::messages::MessageToBackend;
use rustipelago_schema::archipelago::ApCard;

pub(crate) struct CardManager {
    cards: HashMap<ApCard, bool>,
    // sender: Sender<MessageToBackend>,
}
impl GPUIStructHelper for CardManager {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            cards: HashMap::new(),
            // sender,
        }
    }
}
impl CardHandler for CardManager {
    fn get_name() -> impl Into<SharedString> {
        "Card Manager"
    }
}
impl Render for CardManager {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .id("Scrollable card manager")
            .overflow_scroll()
            .overflow_scrollbar()
            .children(self.cards.iter().map(|(card, open)| {
                let c = card.clone();
                Button::new(card.name.clone())
                    .when_else(
                        *open,
                        |this| this.label(format!("Edit {}", card.name)),
                        |this| this.label(format!("Finish editing {}", card.name)),
                    )
                    .size_full()
                    .on_click(cx.listener(move |this, _, _, _| {
                        // _ = this
                        //     .sender
                        //     .send(MessageToBackend::EditWorld { card: c.clone() });
                    }))
            }))
    }
}
