use gpui::{
    Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, Window, div,
};

// ToDo: Consider consolidating these!
use gpui_component::Theme;

// Internal Crate
use crate::tabs::TabsWithContent;

pub struct MainView {
    pub tabs_content: Entity<TabsWithContent>,
    _appearance_subscription: Subscription, // ToDo: Discuss Message Passing both pros and cons
                                            // Count Moved
}

impl MainView {
    pub fn new(
        tabs_content: Entity<TabsWithContent>,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Self {
        let subscription = window.observe_window_appearance(|window, cx| {
            Theme::sync_system_appearance(Some(window), cx);
            cx.refresh_windows();
        });

        Self {
            tabs_content,
            _appearance_subscription: subscription,
        }
    }
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Get access to theme
        let theme = Theme::global_mut(cx);

        div()
            .size_full()
            .bg(theme.background)
            // ToDo: Discuss `self`. See scratch.md
            .child(self.tabs_content.clone())
    }
}
