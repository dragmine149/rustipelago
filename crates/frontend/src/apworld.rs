use gpui::{
    App, AppContext, Context, Entity, Image, ImageSource, IntoElement, ObjectFit, ParentElement,
    Render, RenderImage, Styled, StyledImage, SvgRenderer, Window, div, img,
};
use gpui_component::{Icon, IconName, Sizable, button::Button, label::Label, v_flex};
use rustipelago_apworlds::APWorld;

use crate::{Assets, client::open_client};

pub(crate) struct APWorldCard {
    pub world_info: APWorld,
}
impl APWorldCard {
    pub fn view(world: APWorld, window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(world, window, cx))
    }
    fn new(world: APWorld, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self { world_info: world }
    }
}

impl Render for APWorldCard {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Button::new("ap-world")
            .child(
                v_flex()
                    .child(
                        Icon::default()
                            .path("ArchipelagoIcon.svg")
                            .size_full()
                            .pb_10(),
                        // img(self
                        //     .world_info
                        //     .icon
                        //     .clone()
                        //     .unwrap_or("assets/archipelago_icon.png".to_string()))
                        // .with_fallback(|| div().child("Failed loading image").into_any_element())
                        // .object_fit(ObjectFit::Contain),
                    )
                    .child(
                        Label::new(self.world_info.name.clone())
                            .text_xl()
                            .text_center(),
                    )
                    .child(Label::new(self.world_info.description.clone()).text_center()),
            )
            .on_click(cx.listener(|this, ev, win, cx| {
                println!("Clicked world: {}", this.world_info.name);
                open_client(this.world_info.clone(), win, cx).unwrap();
            }))
            .size_40()
        // .aspect_square()
    }
}
