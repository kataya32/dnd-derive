use gpui::{
    Context, Entity, EventEmitter, IntoElement, ParentElement, Render, Styled, Window, div,
    prelude::FluentBuilder,
};
use gpui_component::sidebar::{
    Sidebar, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem,
};
use gpui_component::{ActiveTheme, Icon, IconName, TitleBar, button::Button, h_flex, v_flex};

pub struct MainView {
    sidebar_collapsed: bool,
    main_content: Entity<MainContent>,
}

impl MainView {
    pub fn new(main_content: Entity<MainContent>, cx: &mut Context<Self>) -> Self {
        Self {
            sidebar_collapsed: false,
            main_content: main_content,
        }
    }

    pub fn toggle_sidebar(&mut self, cx: &mut Context<Self>) {
        self.sidebar_collapsed = !self.sidebar_collapsed;
        cx.notify();
    }
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .h_full()
            .child(
                TitleBar::new()
                    .bg(cx.theme().background)
                    .border_color(cx.theme().border)
                    .child(div().text_color(cx.theme().foreground).child("Rust Vault")),
            )
            .child(
                h_flex()
                    .h_full()
                    .child(
                        Sidebar::new("left-sidebar")
                            .collapsible(true)
                            .collapsed(self.sidebar_collapsed)
                            .header(
                                SidebarHeader::new().child(
                                    h_flex()
                                        .child(Icon::new(IconName::Building2))
                                        .when(!self.sidebar_collapsed, |this| this.child("Home")),
                                ),
                            )
                            .child(
                                SidebarGroup::new("Navigation").child(
                                    SidebarMenu::new()
                                        .child(
                                            SidebarMenuItem::new("Dashboard")
                                                .icon(IconName::LayoutDashboard)
                                                .on_click(|_, _, _| println!("Dashboard clicked")),
                                        )
                                        .child(
                                            SidebarMenuItem::new("Settings")
                                                .icon(IconName::Settings)
                                                .on_click(|_, _, _| println!("Settings clicked")),
                                        ),
                                ),
                            )
                            .footer(SidebarFooter::new().child("User Profile")),
                    )
                    .child(self.main_content.clone()),
            )
    }
}

pub enum MainContentEvent {
    ToggleSidebar,
}

pub struct MainContent;

impl EventEmitter<MainContentEvent> for MainContent {}

impl MainContent {
    pub fn new() -> Self {
        Self
    }
}

impl Render for MainContent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("Hello, World!")
            .child(
                Button::new("sidebar-toggle")
                    .label("Goodbye Cruel World")
                    .on_click(cx.listener(|_this, _event, _window, cx| {
                        cx.emit(MainContentEvent::ToggleSidebar);
                    })),
            )
    }
}
