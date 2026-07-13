use crate::{
    home::Home,
    writer::{Writer, config::Config},
};
use gpui::{
    App, AppContext, Bounds, Context, Entity, KeyBinding, SharedString, TitlebarOptions, Window,
    WindowBounds, WindowOptions, actions, px, size,
};
use gpui_component::Root;
use std::fs;
pub(crate) mod home;
pub(crate) mod writer;

pub(crate) trait GPUIStructHelper
where
    Self: 'static + Sized,
{
    fn view(window: &mut Window, cx: &mut App) -> Entity<Self>
    where
        Self: Sized,
    {
        cx.new(|cx| Self::new(window, cx))
    }
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self;
}

actions!([Quit]);

pub fn main() {
    gpui_platform::application()
        .with_assets(gpui_component_assets::Assets)
        .run(move |cx| {
            gpui_component::init(cx);
            // TODO: Sort out themes.
            gpui_component::Theme::change(gpui_component::ThemeMode::Dark, None, cx);

            // we can... just "borrow" that dir as all the config is there anyway.
            let config_dir = dirs::data_dir()
                .expect("Failed to find data dir.")
                .join("Archipelago");
            writer::init_writers(cx, &config_dir);

            let theme_folder = config_dir.join("Themes");
            if !theme_folder.exists() {
                // TODO: Should we not error here?
                // TODO: Add info file.
                fs::create_dir(&theme_folder).expect("Faailed to create theme folder");
                // fs::write(theme_folder.join("rustipelago.readme"), "")
                //     .expect("Failed to write hello message");
            }

            _ = gpui_component::ThemeRegistry::watch_dir(theme_folder.clone(), cx, move |cx| {
                let theme_name = Config::get(cx).active_theme.clone();
                if theme_name.is_empty() {
                    return;
                }
                let Some(theme) = gpui_component::ThemeRegistry::global(cx)
                    .themes()
                    .get(&SharedString::new(theme_name.trim_ascii()))
                    .cloned()
                else {
                    return;
                };
                gpui_component::Theme::global_mut(cx).apply_config(&theme);
            });

            cx.on_app_quit(|cx| {
                Config::force_save(cx);
                async {}
            })
            .detach();
            cx.bind_keys([KeyBinding::new("secondary-q", Quit, None)]);
            cx.on_action(|_: &Quit, cx| {
                for window in cx.windows() {
                    _ = window.update(cx, |_, window, _| {
                        window.remove_window();
                    })
                }
            });

            let bounds = Bounds::centered(None, size(px(1000.), px(1000.)), cx);
            cx.open_window(
                WindowOptions {
                    app_id: Some("Rustipelago".into()),
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some(format!("Rustipelago {}", env!("CARGO_PKG_VERSION")).into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |window, cx| {
                    let home = Home::view(window, cx);
                    cx.new(|cx| Root::new(home, window, cx))
                },
            )
            .unwrap();
        });
}
