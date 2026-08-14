use gpui::SharedString;
use gpui::{AppContext, Application, WindowOptions};
use gpui_component::{Theme, ThemeRegistry};
use std::path::PathBuf;

// ToDo: Discuss Modules!
mod root_view;
mod tabs;
use crate::root_view::RootView;
use crate::tabs::{MainTabs, TabsWithContent};

fn main() {
    Application::new().run(|app| {
        gpui_component::init(app);

        // Syncs system to light or dark
        Theme::sync_system_appearance(None, app);

        // Set up directory monitor tracking your local themes path
        let themes_dir = PathBuf::from("./themes");
        let active_theme_name = SharedString::from("Molokai Light"); // Set Theme Name

        if !themes_dir.exists() {
            let _ = std::fs::create_dir_all(&themes_dir);
        }

        // Initialize filesystem watcher: See GPUI Component::Theme
        if let Err(err) = ThemeRegistry::watch_dir(themes_dir, app, move |app| {
            // Triggered when a file is edited inside ./themes
            if let Some(config) = ThemeRegistry::global(app)
                .themes()
                .get(&active_theme_name)
                .cloned()
            {
                // Mutate the active configuration framework allocation
                Theme::global_mut(app).apply_config(&config);
                // Invalidate drawing caches to render style updates on screen
                app.refresh_windows();
            }
        }) {
            eprintln!("Failed to bind themes file monitor: {:?}", err);
        }

        // Spin up UI context window
        app.open_window(WindowOptions::default(), |_window, app| {
            app.new(|cx| RootView {
                tabs_content: cx.new(|_| TabsWithContent {
                    active_tab: MainTabs::LevelingTab,
                    count: 0,
                }),
            })
        })
        .unwrap(); // ToDo: Discuss `.unwrap()`
    });
}
