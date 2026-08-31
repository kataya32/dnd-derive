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
    characters: Vec<CharacterSheet>,
}

impl CharacterStore {
    pub fn new() -> Self {
        Self {
            characters: vec![
                CharacterSheet {
                    id: CharacterId(0),
                    character_name: "Aragorn".into(),
                    base_class: Class::Ranger,
                    sub_classes: Vec::new(),
                    multiclasses: Vec::new(),
                    level: 5,
                    strength: Stat::new(16),
                    dexterity: Stat::new(14),
                    constitution: Stat::new(14),
                    intelligence: Stat::new(12),
                    wisdom: Stat::new(14),
                    charisma: Stat::new(14),
                    inspiration: false,
                    background: Background::Custom,
                },
                CharacterSheet {
                    id: CharacterId(1),
                    character_name: "Gandalf".into(),
                    base_class: Class::Wizard,
                    sub_classes: Vec::new(),
                    multiclasses: Vec::new(),
                    level: 12,
                    strength: Stat::new(10),
                    dexterity: Stat::new(12),
                    constitution: Stat::new(14),
                    intelligence: Stat::new(18),
                    wisdom: Stat::new(16),
                    charisma: Stat::new(14),
                    inspiration: false,
                    background: Background::Acolyte,
                },
            ],
        }
    }

    pub fn characters(&self) -> &[CharacterSheet] {
        &self.characters
    }

    pub fn add(&mut self, character: CharacterSheet, cx: &mut Context<Self>) {
        self.characters.push(character);
        cx.notify();
    }

    pub fn get(&self, id: CharacterId) -> Option<&CharacterSheet> {
        self.characters.iter().find(|c| c.id == id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Background {
    Acolyte,
    Custom,
}

struct Skill {
    stat: StatType,
    proficient: bool,
    expertise: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatType {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Stat {
    pub score: u8,
    pub modifier: i8,
}

impl Stat {
    pub fn new(score: u8) -> Stat {
        let mut stat = Self {
            score: 0,
            modifier: 0,
        };
        stat.set(score);
        stat
    }

    pub fn set(&mut self, score: u8) {
        self.score = score;
        let modifier = match score {
            1 => -5,
            2 | 3 => -4,
            4 | 5 => -3,
            6 | 7 => -2,
            8 | 9 => -1,
            10 | 11 => 0,
            12 | 13 => 1,
            14 | 15 => 2,
            16 | 17 => 3,
            18 | 19 => 4,
            20 | 21 => 5,
            22 | 23 => 6,
            24 | 25 => 7,
            26 | 27 => 8,
            28 | 29 => 9,
            30 => 10,
            _ => 0,
        };
        self.modifier = modifier;
    }
}

#[derive(Clone)]
pub struct CharacterSheet {
    pub id: CharacterId,
    pub character_name: SharedString,
    pub base_class: Class,
    pub sub_classes: Vec<Class>,
    pub multiclasses: Vec<Class>,
    pub level: u8,
    // Stats
    pub strength: Stat,
    pub dexterity: Stat,
    pub constitution: Stat,
    pub intelligence: Stat,
    pub wisdom: Stat,
    pub charisma: Stat,
    //---
    pub inspiration: bool,
    pub background: Background,
    // Skills
    pub acrobatics: Skill,
    // ---
    // add race, portrait, etc. later
}

impl CharacterSheet {
    pub fn proficiency_bonus(&self) -> u8 {
        match self.level {
            1 | 2 | 3 | 4 => 2,
            5 | 6 | 7 | 8 => 3,
            9 | 10 | 11 | 12 => 4,
            13 | 14 | 15 | 16 => 5,
            17 | 18 | 19 | 20 => 6,
            21 | 22 | 23 | 24 => 7,
            25 | 26 | 27 | 28 => 8,
            29 | 30 => 9,
            _ => 0,
        }
    }

    // ToDo: Needs implemented
    pub fn saving_throw(&self, _stat_type: StatType) -> i8 {
        0
    }

    // ToDo: Needs Implemented
    pub fn saving_throw_proficiency(&self) -> i8 {
        0
    }
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    Barbarian,
    Artificer,
    Ranger,
    Wizard,
    // ToDo: Flush this out
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Class::Barbarian => write!(f, "Barbarian"),
            Class::Artificer => write!(f, "Artificer"),
            Class::Ranger => write!(f, "Ranger"),
            Class::Wizard => write!(f, "Wizard"),
        }
    }
}
