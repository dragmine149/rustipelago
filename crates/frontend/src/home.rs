use crate::{apworld::APWorldCard, percent, thread_to_main};
use gpui::{
    AnyWindowHandle, App, AppContext, AsyncApp, Context, Entity, ImageSource, InteractiveElement,
    IntoElement, ParentElement, Render, StatefulInteractiveElement, Styled, WeakEntity, Window,
    div, img, prelude::FluentBuilder, px,
};
use gpui_component::{
    ActiveTheme, Icon, IconName, Root, Sizable, StyledExt, WindowExt,
    button::{Button, ButtonGroup, ButtonVariants},
    h_flex,
    input::{Input, InputState},
    label::Label,
    notification::{Notification, NotificationType},
    scroll::ScrollableElement,
    tooltip::Tooltip,
    v_flex,
};
use rustipelago_bridge::messages::{MessageToBackend, MessageToFrontend};
use rustipelago_schema::archipelago::{ApCard, CardType};
use std::{
    path::PathBuf,
    sync::mpsc::{Receiver, Sender},
};
use strum::IntoEnumIterator;

/// Main GPUI page.
pub(crate) struct Home {
    search: Entity<InputState>,
    cards: Vec<ApCard>,
    filter: Option<CardType>,

    backend_sender: Sender<MessageToBackend>,
    /// Window handler so that we can use it in AsyncApp
    main_window: AnyWindowHandle,
    view_mode: ViewMode,
}

#[derive(Default, PartialEq, Eq)]
enum ViewMode {
    List,
    #[default]
    Grid,
}

impl Home {
    pub fn view(
        window: &mut Window,
        cx: &mut App,
        frontend_receiver: Receiver<MessageToFrontend>,
        backend_sender: Sender<MessageToBackend>,
    ) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx, frontend_receiver, backend_sender))
    }
    fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        frontend_receiver: Receiver<MessageToFrontend>,
        backend_sender: Sender<MessageToBackend>,
    ) -> Self {
        // send the receiver off to a detached thread which allows it to be non-blocking.
        // Does mean we have a bit more trouble with cx but nothing too major.
        thread_to_main(cx, frontend_receiver, async |this, cx, rx| {
            println!("Waiting for message");
            while let Ok(msg) = rx.recv().await {
                match msg {
                    MessageToFrontend::ReadFailed { path, error } => {
                        let _ = Self::weak_notify(
                            &this,
                            Notification::new()
                                .title(format!("Read `{}` error", path.display()))
                                .message(error.to_string())
                                .autohide(false)
                                .with_type(NotificationType::Error),
                            cx,
                        );
                    }
                    MessageToFrontend::ReqwestFailed { url, error } => {
                        let _ = Self::weak_notify(
                            &this,
                            Notification::new()
                                .title(format!("Fetch Failed `{}`", url))
                                .with_type(NotificationType::Warning)
                                .message(error.to_string()),
                            cx,
                        );
                    }
                    MessageToFrontend::LauncherUpdate { new_version } => {
                        if let Some(version) = new_version {
                            let _ = Self::weak_notify(
                                &this,
                                Notification::new()
                                    .title("Update")
                                    .message(format!("New update available! Version: {}", version))
                                    .with_type(gpui_component::notification::NotificationType::Info)
                                    .autohide(false)
                                    .action(|_, _, cx| {
                                        Button::new("Update").primary().label("Update").on_click(
                                            cx.listener(|this, _, win, cx| {
                                                print!("Start updating launcher somehow");
                                            }),
                                        )
                                    }),
                                cx,
                            );

                            println!("Update!");
                        }
                    }
                    MessageToFrontend::CardsLoaded { cards } => {
                        let _ = this.update(cx, |this, cx| this.cards = cards);
                    }
                }
            }
        })
        .detach();

        // on app start, update check bc why not.
        println!("sending update check");
        let _ = backend_sender.send(MessageToBackend::CheckLauncherUpdate);
        println!("update check sent");
        let _ = backend_sender.send(MessageToBackend::FetchCards);

        Self {
            search: cx.new(|cx| {
                let is = InputState::new(window, cx).placeholder("Search");
                is.focus(window, cx);
                is
            }),
            cards: vec![],
            filter: None,

            backend_sender,
            main_window: window.window_handle(),
            view_mode: ViewMode::Grid,
        }
    }
}
impl Home {
    /// even even more shorthand for notification.
    ///
    /// # Usage
    /// ```rs
    /// let _ = Self::weak_notify(this, Notification::new(), cx);
    /// ```
    fn weak_notify(
        this: &WeakEntity<Self>,
        notification: Notification,
        cx: &mut AsyncApp,
    ) -> anyhow::Result<()> {
        this.update(cx, |this, cx| this.notify(notification, cx))?
    }

    /// Shorthand for notification, saves repeating it a bit.
    fn notify(&mut self, notification: Notification, cx: &mut Context<Self>) -> anyhow::Result<()> {
        cx.update_window(self.main_window, |_, win, cx| {
            win.push_notification(notification, cx);
        })
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
                        .h_auto()
                        .prefix(Icon::new(IconName::Search)),
                ),
            )
            // .child(
            //     div()
            //         .child(Label::new("Favourites"))
            //         .child(
            //             h_flex()
            //                 .id("favuorites_list")
            //                 .w_full()
            //                 .overflow_y_scroll()
            //                 .overflow_y_scrollbar()
            //                 .items_center()
            //                 .border_1()
            //                 .border_color(cx.theme().border)
            //                 .children(
            //                     self.cards
            //                         .iter()
            //                         .filter(|card| {
            //                             // card.read_with(cx, |c, _| {
            //                             // c.world_info.favourite &&
            //                             // c.world_info.name.contains(&search_value)
            //                             false
            //                             // })
            //                         })
            //                         .cloned(),
            //                 ),
            //         )
            //         .pb_5(),
            // )
            .child(
                div()
                    .size_full()
                    .child(
                        h_flex()
                            .w_full()
                            .child(Label::new("Cards").text_3xl())
                            .child(
                                ButtonGroup::new("view_mode")
                                    .child(Button::new("grid").label("Grid").on_click(cx.listener(
                                        |this, _, _, cx| {
                                            this.view_mode = ViewMode::Grid;
                                        },
                                    )))
                                    .child(Button::new("list").label("List").on_click(cx.listener(
                                        |this, _, _, cx| {
                                            this.view_mode = ViewMode::List;
                                        },
                                    )))
                                    .self_end(),
                            ),
                    )
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
                        div()
                            .id("card_list")
                            .w_full()
                            .h(percent(80.))
                            // .h_auto()
                            .children(
                                self.cards
                                    .iter()
                                    .filter(|card| {
                                        card.name.contains(&search_value)
                                            && self
                                                .filter
                                                .as_ref()
                                                .is_none_or(|filter| &card.card_type == filter)
                                    })
                                    .map(|card| {
                                        let name = card.name.clone();
                                        Button::new(format!("ap-world-{}", name))
                                            .when(self.view_mode == ViewMode::Grid, |this| {
                                                this.v_flex().size_40()
                                            })
                                            .when(self.view_mode == ViewMode::List, |this| {
                                                this.v_flex().h_40().w_full()
                                            })
                                            .child(
                                                img(ImageSource::from(
                                                    card.icon
                                                        .as_ref()
                                                        .and_then(|v| v.is_empty().then_some(v))
                                                        .unwrap_or(&String::from(
                                                            "images/ArchipelagoIcon.png",
                                                        ))
                                                        .to_string(),
                                                ))
                                                .size_20()
                                                .self_center(),
                                            )
                                            .child(
                                                div()
                                                    .size_full()
                                                    .child(
                                                        Label::new(card.name.clone())
                                                            .text_lg()
                                                            .when(
                                                                self.view_mode == ViewMode::List,
                                                                |this| this.text_2xl(),
                                                            )
                                                            .text_center(),
                                                    )
                                                    .when(
                                                        self.view_mode == ViewMode::List,
                                                        |this| {
                                                            this.child(format!(
                                                                "\n{}",
                                                                card.description.clone()
                                                            ))
                                                        },
                                                    ),
                                            )
                                            .when(
                                                !card.description.is_empty()
                                                    && self.view_mode == ViewMode::Grid,
                                                |this| this.tooltip(card.description.clone()),
                                            )
                                            .on_click(cx.listener(move |this, _, _, _| {
                                                let _ = this.backend_sender.send(
                                                    MessageToBackend::OpenCard {
                                                        card_name: name.clone(),
                                                    },
                                                );
                                            }))
                                    }),
                            )
                            .items_center()
                            .overflow_scroll()
                            .overflow_scrollbar()
                            .when(self.view_mode == ViewMode::Grid, |this| this.h_flex())
                            .when(self.view_mode == ViewMode::List, |this| this.v_flex()),
                    ),
            )
            .children(notifications)
    }
}
