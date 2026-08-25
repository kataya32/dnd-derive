use gpui::{Context, SharedString};

#[derive(Clone, PartialEq, Eq, Default)]
pub enum AdvancementType {
    #[default]
    Milestone,
    Xp,
}

#[derive(Clone, PartialEq, Eq, Default)]
pub enum EncumbranceType {
    #[default]
    Standard,
    Variant,
    None,
}

#[derive(Clone, PartialEq, Eq, Default)]
pub enum AbilityGenMethod {
    #[default]
    ManualRoll,
    // PointBuy, StandardArray, etc. later
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CharacterId(usize);

impl CharacterId {
    pub fn to_usize(&self) -> usize {
        self.0
    }
}

/// Shared Character Store
pub struct CharacterStore {
    characters: Vec<CharacterSummary>,
}

impl CharacterStore {
    pub fn new() -> Self {
        Self {
            characters: vec![
                CharacterSummary {
                    id: CharacterId(0),
                    name: "Aragorn".into(),
                    class: "Ranger".into(),
                    level: 5,
                },
                CharacterSummary {
                    id: CharacterId(1),
                    name: "Gandalf".into(),
                    class: "Wizard".into(),
                    level: 12,
                },
            ],
        }
    }

    pub fn characters(&self) -> &[CharacterSummary] {
        &self.characters
    }

    pub fn add(&mut self, character: CharacterSummary, cx: &mut Context<Self>) {
        self.characters.push(character);
        cx.notify();
    }

    pub fn get(&self, id: CharacterId) -> Option<&CharacterSummary> {
        self.characters.iter().find(|c| c.id == id)
    }
}

#[derive(Clone)]
pub struct CharacterSummary {
    pub id: CharacterId,
    pub name: SharedString,
    pub class: SharedString,
    pub level: u8,
    // add race, portrait, etc. later
}

#[derive(Clone, Default)]
pub struct CharacterCreationState {
    pub name: String,
    pub sources: Vec<String>, // PHB, Xanathar's, etc.
    pub advancement: AdvancementType,
    pub encumbrance: EncumbranceType,
    pub ignore_coin_weight: bool,
    pub selected_class: Option<String>,
    pub level: u8,
    pub background: Option<String>,
    pub race: Option<String>,
    pub ability_method: AbilityGenMethod,
    // ... spells, equipment, etc.
}
