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

## GrokAI recommended fix
Add a `label` helper on the enum, then use it when building the `TabBar`.

### 1. On `MainTabs` (in `tabs.rs`)

```rust
impl MainTabs {
    pub fn label(self) -> &'static str {
        match self {
            Self::LevelingTab => "Leveling",
            Self::CharacterSheetTab => "Character",
            Self::ClassInfoTab => "Class Info",
            Self::SettingsTab => "Settings",
            Self::CounterTab => "Counter",
        }
    }
}
```

### 2. In `TabsWithContent::render`

```rust
impl Render for TabsWithContent {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                    .child(Tab::new().label(MainTabs::SettingsTab.label()))
                    .child(Tab::new().label(MainTabs::CounterTab.label())),
            )
            .child(div().flex_1().p_4().child(self.render_tab_content(cx)))
    }
}
```

*Reasoning: Labels live in one place; changing a tab name only requires updating `MainTabs::label`.*

## ToDo: Message Passing
- `https://refactoring.guru/design-patterns/observer`
- Message Passing: James Helfrich

### Claude AI: Where subscribe + EventEmitter complicate the picture slightly
observe fires on any state change (generic "something happened"). subscribe requires the emitter to implement EventEmitter<T> and fires with a typed payload — closer to a discriminated event than a bare notification. This is sometimes called an "event emitter" pattern rather than pure Observer, since it carries structured data instead of just "go re-read my state." But it's still not pub-sub: there's no topic string, no broker, no decoupled many-to-many routing — you still need the concrete &Entity<Emitter> to call cx.subscribe on it in the first place.



## ToDo: Replace Custom Variant Buttons
From:
```rs
Button::new("decrement_button")
    .custom(ButtonCustomVariant::new(cx).border(black()).hover(red()))
    .label("-")
    .on_click(cx.listener(Self::decrement)),
```

To:
```rs
// No Hardcoded Colors
Button::new("decrement_button")
    .danger()
    .outline()
    .label("-")
    .on_click(cx.listener(Self::decrement)),
```
