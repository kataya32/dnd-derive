use crate::app_routes::AppRoute;
use crate::character::{CharacterCreationState, CharacterId, CharacterStore};
use gpui::{
    App, Context, Div, Entity, EventEmitter, InteractiveElement, IntoElement, ParentElement,
    Render, SharedString, StatefulInteractiveElement, Styled, Subscription, Window, div,
};
use gpui_component::{
    ActiveTheme, StyledExt,
    button::{Button, ButtonVariants},
    h_flex, v_flex,
};
use gpui_component::{Icon, IconName};

pub enum MainContentEvent {
    ToggleSidebar,
    Navigate(AppRoute),
    SelectCharacter(usize), // character index / id
}

/// MainContent – owns current route + creation state
pub struct MainContent {
    current_route: AppRoute,
    character_store: Entity<CharacterStore>,
    creation: CharacterCreationState,
    _subscription: Subscription,
}

impl EventEmitter<MainContentEvent> for MainContent {}

impl MainContent {
    pub fn new(character_store: Entity<CharacterStore>, cx: &mut Context<Self>) -> Self {
        let subscription = cx.observe(&character_store, |_, _, cx| {
            cx.notify();
        });

        Self {
            current_route: AppRoute::Home,
            character_store,
            creation: CharacterCreationState::default(),
            _subscription: subscription,
        }
    }

    pub fn navigate(&mut self, route: AppRoute, cx: &mut Context<Self>) {
        self.current_route = route.clone();
        cx.emit(MainContentEvent::Navigate(route));
        cx.notify();
    }

    fn render_home(&self, cx: &mut Context<Self>) -> Div {
        let characters = self.character_store.read(cx).characters().to_vec();
        v_flex()
            .size_full()
            .p_6()
            .gap_4()
            .child(div().text_2xl().font_bold().child("Characters"))
            .child(
                div()
                    .flex()
                    .w_full()
                    .gap_4()
                    .children(characters.iter().map(|c| {
                        let id = c.id;
                        div()
                            .id(("character-card", id.to_usize()))
                            .w_full()
                            .p_4()
                            .border_1()
                            .border_color(cx.theme().accent)
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|s| s.bg(cx.theme().accent.opacity(0.1)))
                            .child(
                                h_flex()
                                    .gap_4()
                                    .text_2xl()
                                    .font_bold()
                                    .child(Icon::new(IconName::User).size_10())
                                    .child(c.name.clone()),
                            )
                            .child(format!("{} -- (Lvl {})", c.class, c.level))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.navigate(AppRoute::CharacterSheet(id), cx);
                            }))
                    })),
            )
            .child(
                Button::new("create-character")
                    .label("+ Create New Character")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.navigate(AppRoute::CharacterCreation, cx);
                    })),
            )
    }

    fn render_character_sheet(&self, id: CharacterId, cx: &mut Context<Self>) -> Div {
        let character = self.character_store.read(cx).get(id);
        v_flex()
            .size_full()
            .p_6()
            .gap_4()
            .child(
                h_flex()
                    .gap_4()
                    .child(
                        Button::new("back-home")
                            .ghost()
                            .icon(IconName::ArrowLeft)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.navigate(AppRoute::Home, cx);
                            })),
                    )
                    .child(
                        div().text_2xl().font_bold().child(
                            character
                                .map(|c| c.name.clone())
                                .unwrap_or_else(|| "Unknown".into()),
                        ),
                    ),
            )
            .child(section_header("Stats & Core"))
            .child(placeholder_box(
                "Ability Scores • Proficiency Bonus • Speed • HP",
                cx,
            ))
            .child(section_header("Saving Throws & Skills"))
            .child(placeholder_box(
                "Saving Throws • Skills • Passive Perception",
                cx,
            ))
            .child(section_header("Proficiencies & Languages"))
            .child(placeholder_box("Armor • Weapons • Tools • Languages", cx))
            .child(section_header("Tabs"))
            .child(placeholder_box(
                "Actions | Spells | Inventory | Features & Traits | Background | Notes | Extras",
                cx,
            ))
            .child(section_header("Dice Rolling"))
            .child(placeholder_box("Quick dice roller UI goes here", cx))
    }

    fn render_character_creation(&self, cx: &mut Context<Self>) -> Div {
        v_flex()
            .size_full()
            .p_6()
            .gap_4()
            .child(
                h_flex()
                    .justify_between()
                    .child(div().text_2xl().font_bold().child("Create Character"))
                    .child(
                        Button::new("cancel-creation")
                            .label("Cancel")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.navigate(AppRoute::Home, cx);
                            })),
                    ),
            )
            .child(placeholder_box(
                "Breadcrumbs: Name → Preferences → Class → Level → Background → Race → Abilities → Equipment",
                cx,
            ))
            .child(section_header("1. Character Name"))
            .child(placeholder_box("Text input for name", cx))
            .child(section_header("2. Preferences"))
            .child(placeholder_box(
                "Sources • Advancement (Milestone/XP) • Prerequisites • Encumbrance • Ignore Coin Weight",
                cx,
            ))
            .child(section_header("3. Class Selector"))
            .child(placeholder_box("Class list + Info button per class", cx))
            .child(section_header("4. Level-based Config"))
            .child(placeholder_box("Known Spells (filtered) • Multiclass option", cx))
            .child(section_header("5. Background"))
            .child(placeholder_box(
                "Alignment • Faith • Lifestyle • Traits • Ability Scores defaults",
                cx,
            ))
            .child(section_header("6. Race Selector"))
            .child(placeholder_box("Race list + Info button", cx))
            .child(section_header("7. Ability Generation"))
            .child(placeholder_box(
                "Manual Roll (default) • Assign rolls to stats • Override option",
                cx,
            ))
            .child(section_header("8. Equipment"))
            .child(placeholder_box("Starting equipment choices", cx))
            .child(
                Button::new("finish-creation")
                    .label("Finish & Save (placeholder)")
                    .on_click(|_, _, _| println!("Save character – implement later")),
            )
    }

    fn render_settings(&self, cx: &mut Context<Self>) -> Div {
        v_flex()
            .size_full()
            .p_6()
            .gap_4()
            .child(
                h_flex()
                    .justify_between()
                    .child(div().text_2xl().font_bold().child("Settings"))
                    .child(Button::new("back-from-settings").label("← Back").on_click(
                        cx.listener(|this, _, _, cx| {
                            this.navigate(AppRoute::Home, cx);
                        }),
                    )),
            )
            .child(placeholder_box(
                "GPUI theme / appearance settings component goes here",
                cx,
            ))
            .child(placeholder_box("Other app preferences…", cx))
    }
}

impl Render for MainContent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match &self.current_route {
            AppRoute::Home => self.render_home(cx),
            AppRoute::CharacterSheet(id) => self.render_character_sheet(*id, cx),
            AppRoute::CharacterCreation => self.render_character_creation(cx),
            AppRoute::Settings => self.render_settings(cx),
        }
    }
}

/// Helper
fn section_header(text: impl Into<SharedString>) -> impl IntoElement {
    div().mt_4().text_lg().font_semibold().child(text.into())
}

/// Helper
fn placeholder_box(text: impl Into<SharedString>, cx: &App) -> impl IntoElement {
    div()
        .p_4()
        .rounded_md()
        .border_1()
        .border_color(cx.theme().border)
        .text_color(cx.theme().muted_foreground)
        .child(text.into())
}
