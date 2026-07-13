use gpui::{
    AppContext, Context, DefiniteLength, Entity, Hsla, InteractiveElement, IntoElement, Length,
    ParentElement, Render, StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder,
    px,
};
use gpui_component::{
    ActiveTheme, Icon, IconName,
    button::Button,
    cyan_950, green, green_950, h_flex,
    input::{Input, InputState},
    label::Label,
    scroll::ScrollableElement,
    v_flex,
};

use crate::{
    GPUIStructHelper,
    writer::{
        Writer,
        slots::{Slot, Slots},
    },
};

pub struct SlotRender {
    slot_search: Entity<InputState>,
    slot_server: Entity<InputState>,
    slot_name: Entity<InputState>,
}
impl GPUIStructHelper for SlotRender {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            slot_search: cx.new(|cx| {
                let is = InputState::new(window, cx).placeholder("Find Slot");
                is.focus(window, cx);
                is
            }),
            slot_server: cx
                .new(|cx| InputState::new(window, cx).default_value("archipelago.gg:38281")),
            slot_name: cx.new(|cx| InputState::new(window, cx).placeholder("slot name")),
        }
    }
}
impl Render for SlotRender {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let search_value = self
            .slot_search
            .read_with(cx, |search, _| search.value())
            .to_string()
            .to_lowercase();
        let mut slots = Slots::get_copy(cx).slots;
        slots.sort_by(|a, b| a.accessed.cmp(&b.accessed));
        // println!("{:?}", slots);

        v_flex()
            .size_full()
            .child(Input::new(&self.slot_search).prefix(Icon::new(IconName::Search)))
            .child(
                v_flex()
                    .size_full()
                    .id("SlotList")
                    .overflow_y_scroll()
                    .overflow_y_scrollbar()
                    .children(
                        slots
                            .iter()
                            .filter(|slot| {
                                slot.name.to_lowercase().contains(&search_value)
                                    || slot.alias.to_lowercase().contains(&search_value)
                            })
                            .map(|slot| cx.new(|cx| slot.clone())),
                    ),
            )
            .child(
                h_flex()
                    .w_full()
                    .border_2()
                    .child(
                        v_flex()
                            .w_full()
                            .child(Input::new(&self.slot_server).w_full())
                            .child(Input::new(&self.slot_name).w_full()),
                    )
                    .child(
                        Button::new("Connect")
                            .label("Connect")
                            .h_full()
                            .on_click(cx.listener(|this, ev, win, cx| {
                                let server =
                                    this.slot_server.read_with(cx, |server, _| server.value());
                                let name = this.slot_name.read_with(cx, |name, _| name.value());
                                println!("Attempting connection to {} with {} slot", server, name);
                                Slots::get_mut(cx).slots.push(Slot {
                                    server: server.to_string(),
                                    name: name.to_string(),
                                    ..Default::default()
                                });
                            })),
                    ),
            )
    }
}
impl Render for Slot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Button::new("slot-entry")
            .h(px(48.))
            .child(
                v_flex()
                    .child(
                        Label::new(match self.alias.is_empty() {
                            true => self.name.clone(),
                            false => format!("({}) {}", self.alias, self.name),
                        })
                        .text_xl(),
                    )
                    .child(Label::new(self.server.clone()).italic()),
            )
            .child(
                div()
                    .absolute()
                    .when_else(
                        // TODO: THEMING
                        self.get_completion_percent() == 1.,
                        |this| this.bg(green_950()),
                        |this| this.bg(cyan_950()),
                    )
                    .top_0()
                    .left_0()
                    .h_full()
                    .w(Length::Definite(DefiniteLength::Fraction(
                        self.get_completion_percent() as f32,
                    ))),
            )
            .on_click(cx.listener(|this, ev, win, cx| {
                win.remove_window();
            }))
    }
}
