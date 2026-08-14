# Shared Scratchpad

## Self (Rust) vs This (C#)
* In rust, `this` becomes `self`
* Explicit vs. Implicit: C# hides `this` implicitly. Rust requires you to pass self as the explicit first parameter of every instance method.
* Capital `Self`: A type shortcut meaning "the current struct/type"
* `&self`: Read-only access to the object's data
* `&mut self`: Write access to change properties
* `self` (No Ampersand): Consumes and permanently destroys the instance. The caller can never use that variable again.


## `#[ ]` usage 
In Rust, #[ ] denotes an outer attribute used to apply metadata to the item directly below it.

### Key Uses
* #[derive(...)]: Automatically implements standard traits like Debug, Clone, or PartialEq to eliminate boilerplate.
* #[cfg(...)]: Enables conditional compilation based on flags, features, or target operating systems.
* #[test]: Marks a function as a unit test to be executed during cargo test.
* #[allow(...)]: Modifies compiler lint levels to suppress or enforce specific warnings.
* #[inline]: Suggests that the compiler should integrate the function's code directly into the calling site.

### Syntax Variant
* #![ ] (Inner Attribute): Applies to the item it is inside of, typically used at the top of a file for crate-wide settings.
