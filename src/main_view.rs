use crate::app_routes::AppRoute;
use crate::character::CharacterStore;
use crate::main_content::{MainContent, MainContentEvent};
use gpui::{
    Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled, Subscription,
    Window, div, prelude::FluentBuilder,
};

use gpui_component::sidebar::{
    Sidebar, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem,
};
use gpui_component::{
    ActiveTheme, Icon, IconName, Theme, TitleBar, button::Button, h_flex, v_flex,
};

/// MainView (Sidebar + TitleBar + Content)
pub struct MainView {
    app_title: SharedString,
    sidebar_collapsed: bool,
    character_store: Entity<CharacterStore>,
    main_content: Entity<MainContent>,
    _subscriptions: Vec<Subscription>,
}

impl MainView {
    pub fn new(
        character_store: Entity<CharacterStore>,
        main_content: Entity<MainContent>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut subscriptions = Vec::new();
        // React when the character list changes so the sidebar re-renders
        let character_list_subscription = cx.observe(&character_store, |_, _, cx| {
            cx.notify();
        });
        // React to events from MainContent
        let main_content_event_subscription = cx.subscribe(
            &main_content,
            |this, _emitter, event: &MainContentEvent, cx| match event {
                MainContentEvent::ToggleSidebar => {
                    this.toggle_sidebar(cx);
                }
                MainContentEvent::Navigate(_) | MainContentEvent::SelectCharacter(_) => {
                    cx.notify();
                }
            },
        );
        // React when theme mode changes
        let theme_subscription = window.observe_window_appearance(|window, cx| {
            Theme::sync_system_appearance(Some(window), cx);
            cx.refresh_windows();
        });

        subscriptions.push(character_list_subscription);
        subscriptions.push(main_content_event_subscription);
        subscriptions.push(theme_subscription);

        Self {
            app_title: "D&D Derive".into(),
            sidebar_collapsed: false,
            character_store,
            main_content,
            _subscriptions: subscriptions,
        }
    }

    pub fn toggle_sidebar(&mut self, cx: &mut Context<Self>) {
        self.sidebar_collapsed = !self.sidebar_collapsed;
        cx.notify();
    }
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let collapsed = self.sidebar_collapsed;
        let characters = self.character_store.read(cx).characters().to_vec();

        v_flex()
            .h_full()
            .child(
                TitleBar::new()
                    .bg(cx.theme().background)
                    .border_color(cx.theme().border)
                    .child(div().child(self.app_title.clone())),
            )
            .child(
                h_flex()
                    .h_full()
                    .child(
                        Sidebar::new("left-sidebar")
                            .collapsible(true)
                            .collapsed(collapsed)
                            .header(SidebarHeader::new().child(
                                h_flex().gap_2().child(Icon::new(IconName::User)).when(
                                    !collapsed,
                                    |this| {
                                        this.child(
                                            v_flex().child("John Doe").child(
                                                div()
                                                    .text_xs()
                                                    .text_color(cx.theme().muted_foreground)
                                                    .child("john@example.com"),
                                            ),
                                        )
                                    },
                                ),
                            ))
                            .child(SidebarGroup::new("Characters").child(
                                SidebarMenu::new().children(characters.iter().map(|c| {
                                    let id = c.id;
                                    SidebarMenuItem::new(c.name.clone())
                                        .icon(IconName::User)
                                        .on_click(cx.listener(move |this, _, _, cx| {
                                            this.main_content.update(cx, |content, cx| {
                                                content.navigate(AppRoute::CharacterSheet(id), cx);
                                            });
                                        }))
                                })),
                            ))
                            .footer(
                                SidebarFooter::new().child(
                                    v_flex()
                                        .size_full()
                                        .gap_2()
                                        .child(
                                            Button::new("settings")
                                                .w_full()
                                                .icon(IconName::Settings)
                                                .when(!collapsed, |this| this.label("Settings"))
                                                .on_click(cx.listener(|this, _, _, cx| {
                                                    this.main_content.update(cx, |content, cx| {
                                                        content.navigate(AppRoute::Settings, cx);
                                                    });
                                                })),
                                        )
                                        .child(
                                            Button::new("toggle-sidebar")
                                                .w_full()
                                                .icon(if collapsed {
                                                    IconName::ChevronRight
                                                } else {
                                                    IconName::ChevronLeft
                                                })
                                                .when(!collapsed, |this| {
                                                    this.label("Toggle Sidebar")
                                                })
                                                .on_click(cx.listener(|this, _, _, cx| {
                                                    this.toggle_sidebar(cx);
                                                })),
                                        ),
                                ),
                            ),
                    )
                    .child(self.main_content.clone()),
            )
    }
}
