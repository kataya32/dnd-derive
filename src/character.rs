use gpui::{Context, SharedString};
use gpui_component::plot::scale::ScaleOrdinal;

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
                    level: 5,
                },
                CharacterSheet {
                    id: CharacterId(1),
                    character_name: "Gandalf".into(),
                    base_class: Class::Wizard,
                    level: 12,
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

#[derive(Clone)]
enum Background {
    Acolyte,
    Custom,
}

// enum Skill {
//     Acrobatics(stat: StatType, proficient: bool, expertise: bool),
//     Animal Handling(stat: StatType, proficient: bool, expertise: bool),
//     Arcana(stat: StatType, proficient: bool, expertise: bool),
//     Athletics(stat: StatType, proficient: bool, expertise: bool),
//     Deception(stat: StatType, proficient: bool, expertise: bool),
//     History(stat: StatType, proficient: bool, expertise: bool),
//     Insight(stat: StatType, proficient: bool, expertise: bool),
//     Intimidation(stat: StatType, proficient: bool, expertise: bool),
//     Investigation(stat: StatType, proficient: bool, expertise: bool),
//     Medicine(stat: StatType, proficient: bool, expertise: bool),
//     Nature(stat: StatType, proficient: bool, expertise: bool),
//     Perception(stat: StatType, proficient: bool, expertise: bool),
//     Performance(stat: StatType, proficient: bool, expertise: bool),
//     Persuasion(stat: StatType, proficient: bool, expertise: bool),
//     Religion(stat: StatType, proficient: bool, expertise: bool),
//     SleightOfHand(stat: StatType, proficient: bool, expertise: bool),
//     Stealth(stat: StatType, proficient: bool, expertise: bool),
//     Survival(stat: StatType, proficient: bool, expertise: bool),
// }

enum StatType {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Clone)]
struct Stat {
    score: u8,
    modifier: i8,
}

impl Stat {
    fn new(&mut self, score: u8) -> Stat {
        let stat = Self {
            score: 0,
            modifier: 0,
        };
        self.set(score);
        stat
    }

    fn set(&mut self, score: u8) {
        self.score = score;
        let mut modifier: i8 = 0;
        match score {
            1 => modifier = -5,
            2 | 3 => modifier = -4,
            4 | 5 => modifier = -3,
            6 | 7 => modifier = -2,
            8 | 9 => modifier = -1,
            10 | 11 => modifier = 0,
            12 | 13 => modifier = 1,
            14 | 15 => modifier = 2,
            16 | 17 => modifier = 3,
            18 | 19 => modifier = 4,
            20 | 21 => modifier = 5,
            22 | 23 => modifier = 6,
            24 | 25 => modifier = 7,
            26 | 27 => modifier = 8,
            28 | 29 => modifier = 9,
            30 => modifier = 10,
            _ => modifier = 0,
        }
        self.modifier = modifier
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
    pub strength: Stat,
    pub dexterity: Stat,
    pub constitution: Stat,
    pub intelligence: Stat,
    pub wisdom: Stat,
    pub charisma: Stat,
    pub inspiration: bool,
    pub background: Background,

    // add race, portrait, etc. later
}

impl CharacterSheet {
    fn proficiency_bonus(&self) -> u8 {
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
    fn saving_throw(&self, stat_type: StatType) -> i8 {
        0
    }

    // ToDo: Needs Implemented
    fn saving_throw_proficiency(&self) -> i8 {
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

#[derive(Clone)]
pub enum Class {
    Barbarian,
    Artificer,
    Ranger,
    Wizard,
    // ToDo: Flush this out
}
