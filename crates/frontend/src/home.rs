use gpui::{AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div};
use gpui_component::{
    input::{Input, InputState},
    label::Label,
};

use crate::GPUIStructHelper;

pub(crate) struct Home {
    search: Entity<InputState>,
}

impl GPUIStructHelper for Home {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            search: cx.new(|cx| {
                let is = InputState::new(window, cx);
                is.focus(window, cx);
                is
            }),
        }
    }
}

impl Render for Home {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child(Input::new(&self.search).w_full()).child(div())
    }
}
