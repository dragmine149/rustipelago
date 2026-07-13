use gpui::{
    AppContext, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder, px,
};
use gpui_component::{
    ActiveTheme, Icon, IconName, Sizable, h_flex,
    input::{Input, InputState},
    label::Label,
    scroll::ScrollableElement,
};

use crate::{GPUIStructHelper, apworld::APWorldCard};

pub(crate) struct Home {
    search: Entity<InputState>,
    cards: Vec<Entity<APWorldCard>>,
}

impl GPUIStructHelper for Home {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            search: cx.new(|cx| {
                let is = InputState::new(window, cx).placeholder("Search");
                is.focus(window, cx);
                is
            }),
            cards: rustipelago_apworlds::load_dummy_worlds()
                .iter()
                .map(|world| APWorldCard::view(world.clone(), window, cx))
                .collect(),
        }
    }
}

impl Render for Home {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let search_value = self
            .search
            .read_with(cx, |search, _| search.value())
            .to_string();

        div()
            .child(
                div().h(px(50.)).child(
                    Input::new(&self.search)
                        .w_full()
                        .large()
                        // .with_size(px(100.))
                        // .h_20()
                        .h_auto()
                        // .p_0()
                        // .text_2xl()
                        .prefix(Icon::new(IconName::Search)),
                ),
            )
            .child(
                div()
                    .child(Label::new("Favourites"))
                    .child(
                        h_flex()
                            .id("favuorites_list")
                            .w_full()
                            .overflow_y_scroll()
                            .overflow_y_scrollbar()
                            .items_center()
                            // .when_else(
                            //     self.cards
                            //         .iter()
                            //         .any(|card| card.read_with(cx, |c, _| c.world_info.favourite)),
                            //     |this| this.block(),
                            //     |this| this.hidden(),
                            // )
                            .border_1()
                            .border_color(cx.theme().border)
                            .children(
                                self.cards
                                    .iter()
                                    .filter(|card| {
                                        card.read_with(cx, |c, _| {
                                            c.world_info.favourite
                                                && c.world_info.name.contains(&search_value)
                                        })
                                    })
                                    .cloned(),
                            ),
                    )
                    .pb_5(),
            )
            .child(
                div().child(Label::new("Worlds")).child(
                    h_flex()
                        .id("apworld_list")
                        .children(
                            self.cards
                                .iter()
                                .filter(|card| {
                                    card.read_with(cx, |c, _| {
                                        c.world_info.name.contains(&search_value)
                                    })
                                })
                                .cloned(),
                        )
                        .items_center()
                        .size_full()
                        .overflow_scroll()
                        .overflow_scrollbar(),
                ),
            )
    }
}
