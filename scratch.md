# Plan
D&D Character Management System
## Views:
- **Main View:** Sidebar | Main Content Window
- **Character Creation Wizard:** Fullscreen, breadcrumbs, step by step
- **Settings Page:** GPUI component settings component

## On Application Launch User Sees (Home Page):
- List of Existing Characters
- Create New Character Button

## On Character Click:
- Character Sheet
  - Stats
  - Proficiency Bonus
  -  Speed
  - Saving Throws
  - Passive Skills
  - Proficiencies
  - Languages
  - Skills
  - Actions (PlaceHolder) | Spells | Inventory | Features & Traits | Background | Notes | Extras
- Dice Rolling

## On Create New Character:
- Character Name
- Preferences:
  - Select Sources (Player Handbook, etc.)
  - Advancement Type (Milestone | XP )
  - Prerequisites (Feats, Multiclass Requirements)
  - Encumberance Type (Does weight affect you)
  - Ignore Coin Weight (Togleable)
- Class Selector
  - Info Button for each class
- Level Based Character Config
  - Known Spells (Filtered by level)
  - Multiclass Option (add a second class)
- Background (Defaults)
  - Alignment
  - Faith
  - Lifestyle
  - Custom Character Traits
  - Ability Scores
- Race Selctor
  - Info Button for race
- Abilities Generation Method
  - Manually Rolled (Defalt, maybe more later)
  - Choose which roll goes to what Stat
  - Override (optional)
- Equipment
  - Starting Equipment


# Shared Scratchpad

## Self
- `&self`: Read-only access to the object's data
- `&mut self`: Write access to change properties
- `self` (No Ampersand): Consumes and permanently destroys the instance. The caller can never use that variable again.

## Rusult & Option Types (Per ClaudeAI)
### **Option<T>** — value may or may not exist
- `enum Option<T> { Some(T), None }`

### **Result<T, E>** — operation succeeds or fails, with a reason
- `enum Result<T, E> { Ok(T), Err(E) }`

## ToDo: Message Passing
- `https://refactoring.guru/design-patterns/observer`
- Message Passing: James Helfrich

### Claude AI: Where subscribe + EventEmitter complicate the picture slightly
observe fires on any state change (generic "something happened"). subscribe requires the emitter to implement EventEmitter<T> and fires with a typed payload — closer to a discriminated event than a bare notification. This is sometimes called an "event emitter" pattern rather than pure Observer, since it carries structured data instead of just "go re-read my state." But it's still not pub-sub: there's no topic string, no broker, no decoupled many-to-many routing — you still need the concrete &Entity<Emitter> to call cx.subscribe on it in the first place.
