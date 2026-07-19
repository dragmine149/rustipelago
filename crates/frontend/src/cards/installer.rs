use crate::{GPUIStructHelper, cards::CardHandler};
use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div};
use gpui_component::{button::Button, v_flex};

/// Custom UI for installing apworlds from various sources.
pub struct Installer {}
impl CardHandler for Installer {
    fn get_name() -> impl Into<gpui::SharedString> {
        "Apworld Installer"
    }
}
impl GPUIStructHelper for Installer {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {}
    }
}
impl Render for Installer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child("Install APWorld from...").child(
            v_flex()
                .size_full()
                .child(
                    Button::new("Github Releases")
                        .label("Github Releases")
                        .tooltip(
                            "
Download from the latest github release for that world.
Supports auto updating",
                        )
                        .size_full(),
                )
                .child(
                    Button::new("Github Ref")
                        .label("Github Ref")
                        .tooltip(
                            "
Download from the latest github ref for that world.
Might be more broken than releases.
Supports auto updating",
                        )
                        .size_full(),
                )
                .child(
                    Button::new("From File")
                        .label("From File")
                        .tooltip(
                            "
Install a world from a pre-downloaded file.
Doesn't support auto updating",
                        )
                        .size_full(),
                ),
        )
    }
}
