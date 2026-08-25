use gpui::{App, AppContext, WindowOptions};
use gpui_component::{Root, Theme, ThemeRegistry};
use std::path::PathBuf;

// ToDo: Discuss Modules!
mod app_routes;
mod character;
mod main_content;
mod main_view;

use crate::character::CharacterStore;
use crate::main_content::MainContent;
use crate::main_view::MainView;

fn main() {
    gpui_platform::application()
        .with_assets(gpui_component_assets::Assets)
        .run(|app| {
            gpui_component::init(app);

            load_and_watch_themes(app);

            // Standard GPUI practice
            app.spawn(async move |cx| {
                cx.open_window(WindowOptions::default(), |window, cx| {
                    let character_store = cx.new(|_| CharacterStore::new());
                    let main_content = cx.new(|cx| MainContent::new(character_store.clone(), cx));
                    let main_view =
                        cx.new(|cx| MainView::new(character_store, main_content, window, cx));

                    // This first level on the window, should be a Root.
                    cx.new(|cx| Root::new(main_view, window, cx))
                })
                .expect("Failed to open window");
            })
            .detach();
        });
}

fn load_and_watch_themes(cx: &mut App) {
    let themes_dir = PathBuf::from("./themes");
    if !themes_dir.exists() {
        let _ = std::fs::create_dir_all(&themes_dir);
    }

    // Load + watch. Closure runs after initial load and on every change.
    if let Err(err) = ThemeRegistry::watch_dir(themes_dir, cx, move |cx| {
        let (light, dark) = {
            let registry = ThemeRegistry::global(cx);
            (
                registry.themes().get("Molokai Light").cloned(),
                registry.themes().get("Molokai Dark").cloned(),
            )
        };

        if let Some(light) = light {
            Theme::global_mut(cx).light_theme = light;
        }
        if let Some(dark) = dark {
            Theme::global_mut(cx).dark_theme = dark;
        }

        Theme::sync_system_appearance(None, cx);
        cx.refresh_windows();
    }) {
        tracing::error!(?err, "failed to bind themes file monitor");
    }
}
