use gpui::{Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div};

// ToDo: Consider consolidating these!
use gpui_component::Theme;

// Internal Crate
use crate::tabs::TabsWithContent;

pub struct RootView {
    pub tabs_content: Entity<TabsWithContent>,
    // Count Moved
}

impl Render for RootView {
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
