use crate::GPUIStructHelper;
use gpui::{
    App, AppContext, Bounds, Render, TitlebarOptions, Window, WindowBounds, WindowHandle,
    WindowOptions, px, size,
};
use gpui_component::Root;

pub(crate) mod installer;

pub(crate) trait CardHandler: GPUIStructHelper
where
    Self: Render,
{
    fn open_window(cx: &mut App) -> Result<WindowHandle<Root>, anyhow::Error> {
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Slot Manager".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(800.), px(600.)),
                    cx,
                ))),
                app_id: Some(env!("CARGO_PKG_NAME").to_string()),
                tabbing_identifier: Some(env!("CARGO_PKG_NAME").to_string()),
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| {
                    let new = Self::view(window, cx);
                    Root::new(new, window, cx)
                })
            },
        )
    }
}
