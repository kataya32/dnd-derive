use gpui::{
    ClickEvent, Context, Div, IntoElement, ParentElement, Render, Styled, Window, black, div,
    green, red, white,
};

// ToDo: Consider consolidating these!
use gpui_component::button::{Button, ButtonCustomVariant, ButtonVariants};
use gpui_component::tab::{Tab, TabBar};
use gpui_component::v_flex;

// Discuss Macros
// - Useful
// - Reason to avoid creating macros: Debugging
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
// Assigns numbers in order to cleanly interface with TabBar component
pub enum MainTabs {
    LevelingTab = 0,
    CharacterSheetTab = 1,
    ClassInfoTab = 2,
    SettingsTab = 3,
    CounterTab = 4,
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
            4 => Ok(Self::CounterTab),
            _ => Err(()),
        }
    }
}

pub struct TabsWithContent {
    pub active_tab: MainTabs,
    pub count: isize,
}

impl TabsWithContent {
    // AI Note: Returning `Div` explicitly here ensures the `match` statement in
    // `render_tab_content` resolves to a single consistent type.
    fn render_counter(&self, cx: &mut Context<Self>) -> Div {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .gap_5()
            .bg(white())
            .child(
                Button::new("decrement_button")
                    .custom(ButtonCustomVariant::new(cx).border(black()).hover(red()))
                    .label("-")
                    .on_click(cx.listener(Self::decrement)),
            )
            .child(
                div()
                    .min_w_16()
                    .text_3xl()
                    .text_center()
                    .child(self.count.to_string()),
            )
            .child(
                Button::new("increment_button")
                    .custom(ButtonCustomVariant::new(cx).border(black()).hover(green()))
                    .label("+")
                    .on_click(cx.listener(Self::increment)),
            )
    }

    fn increment(&mut self, _event: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.count += 1;
        cx.notify();
    }

    fn decrement(&mut self, _event: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.count -= 1;
        cx.notify();
    }

    fn render_tab_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        match self.active_tab {
            MainTabs::LevelingTab => div().child("Leveling content"),
            MainTabs::CharacterSheetTab => div().child("Character sheet content"),
            MainTabs::ClassInfoTab => div().child("Class info content"),
            MainTabs::SettingsTab => div().child("Settings content"),
            MainTabs::CounterTab => self.render_counter(cx),
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
                    .child(Tab::new().label("Leveling"))
                    .child(Tab::new().label("Character"))
                    .child(Tab::new().label("Class Info"))
                    .child(Tab::new().label("Settings"))
                    .child(Tab::new().label("Counter")),
            )
            .child(div().flex_1().p_4().child(self.render_tab_content(cx)))
    }
}
