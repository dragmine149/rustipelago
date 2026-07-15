use gpui::{
    App, AppContext, Bounds, TitlebarOptions, Window, WindowBounds, WindowOptions, px, size,
};
use gpui_component::Root;

use crate::{GPUIStructHelper, client::slots::SlotRender};

pub(crate) mod commands;
pub(crate) mod slots;
pub(crate) mod ui;

///
pub(crate) fn slot_management(
    window: &mut Window,
    cx: &mut App,
) -> Result<gpui::WindowHandle<Root>, anyhow::Error> {
    let slot_render = SlotRender::view(window, cx);
    cx.open_window(
        WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some("Slot Manager".into()),
                ..Default::default()
            }),
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                Some(window.display(cx).unwrap().id()),
                size(px(800.), px(600.)),
                cx,
            ))),
            app_id: Some(env!("CARGO_PKG_NAME").to_string()),
            tabbing_identifier: Some(env!("CARGO_PKG_NAME").to_string()),
            ..Default::default()
        },
        |window, cx| cx.new(|cx| Root::new(slot_render, window, cx)),
    )
}
