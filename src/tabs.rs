use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div};

use gpui_component::{
    tab::{Tab, TabBar},
    v_flex,
};

// Discuss Macros
// - Useful
// - Reason to avoid creating macros: Debugging
// #[derive(TryFromPrimitive, IntoPrimitive)] via num_enum crate?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
// Assigns numbers in order to cleanly interface with TabBar component
pub enum MainTabs {
    LevelingTab = 0,
    CharacterSheetTab = 1,
    ClassInfoTab = 2,
    SettingsTab = 3,
}

impl MainTabs {
    fn label(&self) -> &str {
        match self {
            Self::LevelingTab => "Leveling",
            Self::CharacterSheetTab => "Character Sheet",
            Self::ClassInfoTab => "Class Info",
            Self::SettingsTab => "Settings",
        }
    }
}

// Builtin Trait
impl TryFrom<usize> for MainTabs {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::LevelingTab),
            1 => Ok(Self::CharacterSheetTab),
            2 => Ok(Self::ClassInfoTab),
            3 => Ok(Self::SettingsTab),
            _ => Err(()),
        }
    }
}

pub struct TabsWithContent {
    pub active_tab: MainTabs,
}

impl TabsWithContent {
    fn render_tab_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        match self.active_tab {
            MainTabs::LevelingTab => div().child(MainTabs::LevelingTab.label()),
            MainTabs::CharacterSheetTab => div().child("Character sheet content"),
            MainTabs::ClassInfoTab => div().child("Class info content"),
            MainTabs::SettingsTab => div().child("Settings content"),
        }
    }
}

impl Render for TabsWithContent {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // v_flex ensures they are stacked vertically
        v_flex()
            .child(
                TabBar::new("content-tabs")
                    .underline()
                    .selected_index(self.active_tab as usize)
                    .on_click(cx.listener(|view, &index, _, cx| {
                        if let Ok(valid_tab) = MainTabs::try_from(index) {
                            view.active_tab = valid_tab;
                            cx.notify();
                        }
                    }))
                    .child(Tab::new().label(MainTabs::LevelingTab.label()))
                    .child(Tab::new().label(MainTabs::CharacterSheetTab.label()))
                    .child(Tab::new().label(MainTabs::ClassInfoTab.label()))
                    .child(Tab::new().label(MainTabs::SettingsTab.label())),
            )
            .child(div().flex_1().p_4().child(self.render_tab_content(cx)))
    }
}
