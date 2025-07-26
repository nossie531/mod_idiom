# mod_items

Module item importer.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?

This crate provides a macro for common usage of `mod` and `use`.

## Background

In Rust, when dividing code into multiple files, first treat the files as
modules with `mod`, and then access each item inside the module with `use`.
This approach is very flexible.

However, this approach will be redundant, if you just want to separate
items into files and do not plan to access them as modules from multiple
locations. In this case, file name is required twice.

## Examples

Without this crate.

```rust
mod my_items;
pub use my_items::*;
```

With this crate.

```rust
mi::items!(pub my_items::*);
```

Note: `mi` is [shorthand](#tips-shorthand-name) for this crate.

## Other options

There are many crates that can do the same thing.

Followings are some of them.

📦 [`mod_use`] - Supports bulk targeting.  
📦 [`moddef`] - Supports bulk targeting, public level adjustment, etc.  
📦 [`export-magic`] - Supports public level adjustment.

[`mod_use`]: https://crates.io/crates/mod_use/0.2.3
[`moddef`]: https://crates.io/crates/moddef/0.3.0
[`export-magic`]: https://crates.io/crates/export-magic/0.3.6

## Highlights

This crate has **no** functional advantages compared to other crates.  
(Furthermore, this crate does not support batch targeting.)

Therefore all that remains is the name and notation.  
In other words, it's just a matter of style preference.  
I hope this crate fits your style...

## TIPS (shorthand name)

Item imports are boilerplate, so you'll want to keep them short.
This can be achieved by changing crate name when introducing it in
`cargo.toml` as follows.

```toml
[dependencies]
mi = { package = "mod_items", version = "0.1" }
```

## History

See [CHANGELOG](CHANGELOG.md).
