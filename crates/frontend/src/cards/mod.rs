use crate::GPUIStructHelper;
use gpui::{
    App, AppContext, Bounds, Render, SharedString, TitlebarOptions, Window, WindowBounds,
    WindowHandle, WindowOptions, px, size,
};
use gpui_component::Root;

pub(crate) mod cards;
pub(crate) mod installer;

/// Helper trait for opening new windows
pub(crate) trait CardHandler: GPUIStructHelper
where
    Self: Render,
{
    fn get_name() -> impl Into<SharedString>;

    fn open_window(cx: &mut App) -> Result<WindowHandle<Root>, anyhow::Error> {
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some(Self::get_name().into()),
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
