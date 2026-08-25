# Shared Scratchpad

## Self (Rust) vs This (C#)
- In rust, `this` becomes `self`
- Explicit vs. Implicit: C# hides `this` implicitly. Rust requires you to pass self as the explicit first parameter of every instance method.
- Capital `Self`: A type shortcut meaning "the current struct/type"
- `&self`: Read-only access to the object's data
- `&mut self`: Write access to change properties
- `self` (No Ampersand): Consumes and permanently destroys the instance. The caller can never use that variable again.


## `#[ ]` usage 
In Rust, #[ ] denotes an outer attribute used to apply metadata to the item directly below it.

### Key Uses
- #[derive(...)]: Automatically implements standard traits like Debug, Clone, or PartialEq to eliminate boilerplate.
- #[cfg(...)]: Enables conditional compilation based on flags, features, or target operating systems.
- #[test]: Marks a function as a unit test to be executed during cargo test.
- #[allow(...)]: Modifies compiler lint levels to suppress or enforce specific warnings.
- #[inline]: Suggests that the compiler should integrate the function's code directly into the calling site.

### Syntax Variant
- #![ ] (Inner Attribute): Applies to the item it is inside of, typically used at the top of a file for crate-wide settings.

## Rusult & Option Types (Per ClaudeAI)
### **Option<T>** — value may or may not exist
- `enum Option<T> { Some(T), None }`
- Replaces `null`; absence is in the type signature
- Must unwrap (`match`, `if let`, `.unwrap()`, `.map()`) before use — compiler enforces it
- C# analog: `int?`, but generalized to any type and actually enforced

### **Result<T, E>** — operation succeeds or fails, with a reason
- `enum Result<T, E> { Ok(T), Err(E) }`
- Replaces exceptions for expected/recoverable failure
- `E` carries *why* it failed (vs `Option`'s bare nothing)
- C# analog: `TryParse` pattern, generalized — but visible in the return type instead of hidden like a `throw`

### **Shared mechanics**
- Both are plain enums — no special language magic
- Handle via `match` (exhaustive), `if let` (one case), or combinators (`.map()`, `.unwrap_or()`, `.and_then()`)
- `?` operator propagates `Err`/`None` early up the call stack
- `.unwrap()`/`.expect("msg")` = deliberate panic if wrong — explicit opt-in crash, not an ambient risk

### **Core pitch**
- Failure/absence is baked into the type, not a runtime surprise
- Can't compile code that ignores the failure case

## Rust Closures VS C# lambdas/delegates
Rust: `.on_click(cx.listener(Self::increment))`
C#: `button.Click += Increment;`
Conceptually similar (method group → event handler), but worth calling out that Rust closures capturing &mut self are enforcing exclusive access at compile time — there's no way two handlers could race on self.count the way you could accidentally get a threading bug in C# without locks. This is a good "here's what the borrow checker is protecting you from" moment.

## Match VS Switch (Exhuastiveness)


## ToDo: Discuss how much space rust projects use.

## ToDo: Discuss unwraps and why to avoid the default `.unwrap()`



## ToDo: Message Passing
- `https://refactoring.guru/design-patterns/observer`
- Message Passing: James Helfrich

### Claude AI: Where subscribe + EventEmitter complicate the picture slightly
observe fires on any state change (generic "something happened"). subscribe requires the emitter to implement EventEmitter<T> and fires with a typed payload — closer to a discriminated event than a bare notification. This is sometimes called an "event emitter" pattern rather than pure Observer, since it carries structured data instead of just "go re-read my state." But it's still not pub-sub: there's no topic string, no broker, no decoupled many-to-many routing — you still need the concrete &Entity<Emitter> to call cx.subscribe on it in the first place.



On Application Launch User Sees (Home Page):
- List of Existing Characters
- Create New Character Button

On Character Click:
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

On Create New Character:
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

D&D Character Management System:
Views:
- **Main View:** Sidebar | Main Content Window
- **Character Creation Wizard**
- **Settings Page**
