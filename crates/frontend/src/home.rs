use gpui::{
    AnyWindowHandle, App, AppContext, Context, Entity, InteractiveElement, IntoElement,
    ParentElement, Render, StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder,
    px,
};
use gpui_component::{
    ActiveTheme, Icon, IconName, Root, Sizable, WindowExt,
    button::{Button, ButtonVariants},
    h_flex,
    input::{Input, InputState},
    label::Label,
    notification::Notification,
    scroll::ScrollableElement,
};
use rustipelago_bridge::{BackendSender, FrontendReceiver, MessageToBackend};
use rustipelago_schema::archipelago::CardType;
use strum::IntoEnumIterator;

use crate::{GPUIStructHelper, apworld::APWorldCard, thread_to_main};

pub(crate) struct Home {
    search: Entity<InputState>,
    cards: Vec<Entity<APWorldCard>>,
    filter: Option<CardType>,

    backend_sender: BackendSender,
    main_window: AnyWindowHandle,
}

impl Home {
    pub fn view(
        window: &mut Window,
        cx: &mut App,
        frontend_receiver: FrontendReceiver,
        backend_sender: BackendSender,
    ) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx, frontend_receiver, backend_sender))
    }
    fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        frontend_receiver: FrontendReceiver,
        backend_sender: BackendSender,
    ) -> Self {
        thread_to_main(cx, frontend_receiver.receiver, async |this, cx, rx| {
            println!("Waiting for message");
            while let Ok(msg) = rx.recv().await {
                match msg {
                    rustipelago_bridge::MessageToFrontend::ReadFailed { path, error } => todo!(),
                    rustipelago_bridge::MessageToFrontend::ReqwestFailed { url, error } => todo!(),
                    rustipelago_bridge::MessageToFrontend::LauncherUpdate { new_version } => {
                        if let Some(version) = new_version {
                            this.update(cx, |this, cx| {
                                cx.update_window(this.main_window, |_, win, cx| {
                                    let noti = Notification::new()
                                        .title("Update")
                                        .message(format!(
                                            "New update available! Version: {}",
                                            version
                                        ))
                                        .with_type(
                                            gpui_component::notification::NotificationType::Info,
                                        )
                                        .autohide(false)
                                        .action(|_, win, cx| {
                                            Button::new("Update")
                                                .primary()
                                                .label("Update")
                                                .on_click(cx.listener(|this, _, win, cx| {
                                                    print!("Start updating launcher somehow");
                                                }))
                                        });
                                    // let notify_msg = format!("New update available!\nCurrent version: {}, New version: {}", env!("CARGO_PKG_VERSION"), version);
                                    win.push_notification(noti, cx);
                                });
                            });
                            // cx.
                            // cx.update(|cx| {
                            //     window.push_notification(
                            //         format!("New update available! Version {}", version),
                            //         cx,
                            //     )
                            // });
                            println!("Update!");
                        }
                    }
                }
            }
        })
        .detach();

        println!("sending update check");
        backend_sender.send(MessageToBackend::CheckLauncherUpdate);
        println!("update check sent");

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

            backend_sender,
            main_window: window.window_handle(),
        }
    }
}

impl Render for Home {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let search_value = self
            .search
            .read_with(cx, |search, _| search.value())
            .to_string();

        let notifications = Root::render_notification_layer(window, cx);
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
            .children(notifications)
    }
}
