use crate::{GPUIStructHelper, cards::CardHandler};
use gpui::{Context, IntoElement, ParentElement, Render, Window, div};
use gpui_component::{button::Button, h_flex};

pub struct Installer {}
impl CardHandler for Installer {}
impl GPUIStructHelper for Installer {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {}
    }
}
impl Render for Installer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child("Install APWorld from...").child(
            h_flex()
                .child(
                    Button::new("Github Releases")
                        .label("Github Releases")
                        .tooltip(
                            "
Download from the latest github release for that world.
Supports auto updating",
                        ),
                )
                .child(Button::new("Github Ref").label("Github Ref").tooltip(
                    "
Download from the latest github ref for that world.
Might be more broken than releases.
Supports auto updating",
                ))
                .child(Button::new("From File").label("From File").tooltip(
                    "
Install a world from a pre-downloaded file.
Doesn't support auto updating",
                )),
        )
    }
}
