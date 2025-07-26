//! `mod` and `use` idiom.
//!
//! *The author of this crate is not good at English.*  
//! *Forgive me if the document is hard to read.*

#![no_std]
#![warn(missing_docs)]

pub mod prelude;

/// A macro combining `mod` and `use`.
///
/// # Examples
///
/// File `main.rs`
///
/// ```ignore
/// mod util;
///
/// fn main() {
///     my_func1();
///     my_func2();
///     my_func3();
/// }
/// ```
///
/// File `util/mod.rs`:
///
/// ```ignore
/// # use mod_idiom::prelude::*;
/// u!(pub my_items);
/// ```
///
/// File `util/my_items.rs`:
///
/// ```ignore
/// pub fn my_func1() {}
/// pub fn my_func2() {}
/// pub fn my_func3() {}
/// ```
#[macro_export]
macro_rules! u {
    ($vis:vis mod $module:ident) => {
        mod $module;
        $vis use $module::*;
    };
}
