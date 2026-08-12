use gpui::{AppContext, Application, WindowOptions};

mod root_view;
use crate::root_view::RootView;

fn main() {
    Application::new().run(|app| {
        gpui_component::init(app);
        app.open_window(WindowOptions::default(), |_window, app| {
            app.new(|_cx| RootView { count: 0 })
        })
        .unwrap();
    });
}
