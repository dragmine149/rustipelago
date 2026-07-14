use gpui::{
    AppContext, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder, px,
};
use gpui_component::{
    ActiveTheme, Icon, IconName, Sizable,
    button::Button,
    h_flex,
    input::{Input, InputState},
    label::Label,
    scroll::ScrollableElement,
};
use rustipelago_apworlds::{CardType, CardTypeIter};
use strum::IntoEnumIterator;

use crate::{GPUIStructHelper, apworld::APWorldCard};

pub(crate) struct Home {
    search: Entity<InputState>,
    cards: Vec<Entity<APWorldCard>>,
    filter: Option<CardType>,
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
            filter: None,
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
            .size_full()
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
                                            // c.world_info.favourite &&
                                            c.world_info.name.contains(&search_value) && false
                                        })
                                    })
                                    .cloned(),
                            ),
                    )
                    .pb_5(),
            )
            .child(
                div()
                    .w_full()
                    .child(Label::new("Cards"))
                    .child(
                        h_flex()
                            .p_2()
                            .gap_1()
                            .w_full()
                            .child(Button::new("all").label("All").rounded_lg().on_click(
                                cx.listener(|this, _, _, _| {
                                    this.filter = None;
                                }),
                            ))
                            .children(CardType::iter().map(|card| {
                                Button::new(card.to_string())
                                    .label(card.to_string())
                                    .rounded_lg()
                                    .on_click(cx.listener(move |this, _, _, _| {
                                        this.filter = Some(card.clone());
                                    }))
                            })),
                    )
                    .child(
                        h_flex()
                            .id("card_list")
                            .children(
                                self.cards
                                    .iter()
                                    .filter(|card| {
                                        card.read_with(cx, |c, _| {
                                            c.world_info.name.contains(&search_value)
                                                && self.filter.as_ref().map_or(true, |filter| {
                                                    &c.world_info.card_type == filter
                                                })
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
