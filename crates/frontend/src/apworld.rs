//! Everything to do with rendering the cards in the UI.
use crate::client::slot_management;
use gpui::{
    App, AppContext, Context, Entity, ImageSource, InteractiveElement, IntoElement, ParentElement,
    Render, StatefulInteractiveElement, Styled, Window, img, prelude::FluentBuilder,
};
use gpui_component::{button::Button, label::Label, tooltip::Tooltip, v_flex};
use rustipelago_schema::archipelago::ApCard;

pub(crate) struct APWorldCard {
    pub world_info: ApCard,
}
impl APWorldCard {
    pub fn view(world: ApCard, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(world, cx))
    }
    fn new(world: ApCard, _cx: &mut Context<Self>) -> Self {
        Self { world_info: world }
    }
}

impl Render for APWorldCard {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let description = self.world_info.description.clone();
        Button::new("ap-world")
            .child(
                v_flex()
                    .id("apcard")
                    .child(
                        img(ImageSource::from("images/ArchipelagoIcon.png"))
                            .size_20()
                            .self_center(),
                    )
                    .child(
                        Label::new(self.world_info.name.clone())
                            .text_xl()
                            .text_center(),
                    )
                    .when(!description.is_empty(), |this| {
                        this.tooltip(move |window, cx| {
                            Tooltip::new(description.clone()).build(window, cx)
                        })
                    }),
            )
            .on_click(cx.listener(|this, _, win, cx| {
                println!("Clicked world: {}", this.world_info.name);
                slot_management(win, cx).unwrap();
            }))
            .size_40()
        // .aspect_square()
    }
}
