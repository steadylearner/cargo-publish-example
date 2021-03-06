// https://doc.rust-lang.org/1.7.0/book/using-rust-without-the-standard-library.html
#![no_std]

#![doc(html_favicon_url = "https://avatars0.githubusercontent.com/u/32325099?s=460&u=cd848fc83d9739939a4ea2d38108c8bcee199109&v=4")]
#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/32325099?s=460&u=cd848fc83d9739939a4ea2d38108c8bcee199109&v=4")]

#![warn(missing_docs)]
// #![warn(missing_doc_code_examples)] // Use it if necessary.

//! [trybuild]: https://github.com/dtolnay/trybuild
//! [macrotest]: https://github.com/eupn/macrotest
//!
//! Publish(Struct, Enum)
//! =============
//!
//! It provides functional macros to reuse fields from [Struct](https://doc.rust-lang.org/std/keyword.struct.html) and [Enum](https://doc.rust-lang.org/std/keyword.enum.html) definition.
//!
//! ```toml
//! [dependencies]
//! publish = "0.0.0"
//! ```
//!
//! <br>
//!
//! ## Example
//!
//! ### Struct
//!
//! ```
//! #[macro_use] extern crate publish;
//!
//! fn main() {
//!     public_struct!(
//!         // pub is required before 'struct' when you use public_struct!
//!         pub struct MessageBase {
//!             pub text: String
//!             // text: String // pub is optional in fields.
//!         }
//!     ); // It is lazy. Nothing is made yet.
//!
//!     MessageBase!(); // You have to call it to use the struct.
//!     let message = MessageBase {
//!         text: "First Message".into(),
//!     };
//!     println!("{}", &message.text);
//! }
//! ```
//!
//! ### Enum
//!
//! ```
//! #[macro_use] extern crate publish;
//!
//! fn main() {
//!     public_enum!(
//!         // pub is required before 'enum' when you use public_enum!
//!         pub enum WebEventBase {
//!             PageLoad,
//!             PageUnload, // , here is required if you want to extend the fields later.
//!         }
//!     ); // It is lazy. Nothing is made yet.
//!
//!     WebEventBase!(); // You have to call it to use the enum.
//!
//!     fn inspect(event: WebEventBase) {
//!         match event {
//!             WebEventBase ::PageLoad => println!("page loaded"),
//!             WebEventBase ::PageUnload => println!("page unloaded"),
//!         }
//!     }
//!
//!     let load    = WebEventBase::PageLoad;
//!     let unload  = WebEventBase::PageUnload;
//!
//!     inspect(load);
//!     inspect(unload);
//! }
//! ```
//!
//! <br>
//!
//! ## Details
//!
//! - Each struct and enum created from the macros are completely unrelevant except they have the same fields you define.
//!
//! - When you use `private_struct!` and `private_enum!`, you can't use pub keyword in it and others use them. [It wouldn't be logical if private struct or private enum can have public fields.](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#making-structs-and-enums-public)
//!
//! - `nested_macro!` is required to use the other macros from this crate. It is used to make a macro that creates macros.
//! ```rust
//! macro_rules! nested_macro {
//!     ($($body:tt)*) => {
//!         macro_rules! __nested_macro { $($body)+ }
//!         __nested_macro!($);
//!     }
//! }
//! ```
//!
//! <br>
//!
//! ## Comparison with attribute macros
//!
//! - [You can reuse the fields with attribute macros also.](https://github.com/steadylearner/Rust-Full-Stack/tree/master/macro/attribute) But, you need to install more dependencies.
//!
//! - [If you want more, please read the official documenation about procedural macros.](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)

mod r#enum;
mod r#struct;

/// You can make a macro that create macros with this.
///
/// ## Example
///
/// ```
/// macro_rules! public_struct {
///     (pub struct $commonstruct:ident { $( $commonfieldpub:vis $commonfield:ident: $commonty:ty ),+ $(,)* }) => {
///         nested_macro! {
///             ($s:tt) => {
///                 macro_rules! $commonstruct {
///                     () => {
///                         pub struct $commonstruct {
///                             $( $commonfieldpub $commonfield: $commonty, )+
///                         }
///                     };
///                 }
///             }
///         }
///     };
/// }
/// ```
#[macro_export]
macro_rules! nested_macro {
    ($($body:tt)*) => {
        macro_rules! __nested_macro { $($body)+ }
        __nested_macro!($);
    }
}

// It will only work with $cargo test --doc or $cargo test
#[cfg(doctest)]
doc_comment::doctest!("../README.md"); // If you want it to fail, ../README_FAIL.md

// $cargo expand --ugly
// $cargo clippy --all-targets --all-features -- -D warnings
