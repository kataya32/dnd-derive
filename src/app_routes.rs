use crate::character::CharacterId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    Home,                        // list of characters + "Create New"
    CharacterSheet(CharacterId), // view existing character
    CharacterCreation,           // character creation wizard
    Settings,
}
