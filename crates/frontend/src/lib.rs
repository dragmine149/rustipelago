use crate::{
    home::Home,
    writer::{Writer, config::Config},
};
use anyhow::anyhow;
use gpui::{
    App, AppContext, AssetSource, Bounds, Context, Entity, KeyBinding, SharedString,
    TitlebarOptions, Window, WindowBounds, WindowOptions, actions, px, size,
};
use gpui_component::Root;
use rust_embed::RustEmbed;
use std::{fs, path::PathBuf};
pub(crate) mod apworld;
pub(crate) mod client;
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

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "*"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<std::borrow::Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }
        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow!("could not find asset at path \"{path}\""))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}

actions!([Quit]);

pub fn main(config_dir: PathBuf, internal_dir: PathBuf) {
    gpui_platform::application()
        .with_assets(gpui_component_assets::Assets)
        .with_assets(Assets)
        .run(move |cx| {
            gpui_component::init(cx);
            // TODO: Sort out themes.
            gpui_component::Theme::change(gpui_component::ThemeMode::Dark, None, cx);

            writer::init_writers(cx, &config_dir);

            let theme_folder = internal_dir.join("Themes");
            if !theme_folder.exists() {
                let _ = fs::create_dir(&theme_folder);
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

            let bounds = Bounds::centered(None, size(px(800.), px(600.)), cx);
            cx.open_window(
                WindowOptions {
                    app_id: Some(env!("CARGO_PKG_NAME").to_string()),
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some(format!("Rustipelago {}", env!("CARGO_PKG_VERSION")).into()),
                        ..Default::default()
                    }),

                    tabbing_identifier: Some(env!("CARGO_PKG_NAME").to_string()),
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
