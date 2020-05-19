Reuse(Struct, Enum)
=============

It provides functional macros to reuse fields with Struct and Enum.

<br>

## Example

### Struct

```rust
use publish::{
    nested_macro,
    public_struct,
};

public_struct!(
    // pub is required before 'struct' when you use public_struct!
    pub struct MessageBase {
        pub text: String
        // text: String // pub is optional in fields.
    }
);

MessageBase!(); // You have to call it to use the struct.

fn main() {
    let message = MessageBase {
        text: "First Message".into(),
    };
    println!("{}", &message.text);
}
```

### Enum

```rust
use publish::{
    nested_macro,
    public_enum,
};

public_enum!(
    // pub is required before 'enum' when you use public_enum!
    pub enum WebEventBase {
        PageLoad,
        PageUnload, // , here is required.
    }
);

WebEventBase!(); // You have to call it to use the enum.

fn inspect(event: WebEventBase) {
    match event {
        WebEventBase ::PageLoad => println!("page loaded"),
        WebEventBase ::PageUnload => println!("page unloaded"),
    }
}

fn main() {
    let load    = WebEventBase::PageLoad;
    let unload  = WebEventBase::PageUnload;

    inspect(load);
    inspect(unload);
}
```

<br>

## Details

- The functional macros from this crate will help you to reuse fields in a struct and enum.

- Each struct and enum created from them are completely unrelevant except they have the same fields you define.

- When you use private_struct! and private_struct!, you can't use pub keyword in it. [Read this if you want more information.](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#making-structs-and-enums-public)

- nested_macro! is required to use the other macros provied from this crate. It internally helps you to cutomize struct and enum name.

```rust
macro_rules! nested_macro {
    ($($body:tt)*) => {
        macro_rules! __nested_macro { $($body)+ }
        __nested_macro!($);
    }
}
```

<br>

## Comparison to attribute macros

[You can reuse the fields with attribute macros.](https://github.com/steadylearner/Rust-Full-Stack/tree/master/macro/attribute) But, you need to install more dependencies.

[If you want more information, please read the official documenation about procedural macros.](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)

<br>

## How to test it

If you want to test how macros from this package expands, install [rustfmt](https://github.com/rust-lang/rustfmt) and [cargo-expand](https://github.com/dtolnay/cargo-expand) first.

Otherwise, delete tests/expand and use `$cargo test`.

```console
$rustup component add rustfmt && cargo install cargo-expand
$git clone git@github.com:steadylearner/publish.git && cargo test
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

#### What left

* cargo fmt, cargo clippy etc with some script files before publish? Include some commands to help the development.
* Include Travis CI.(How to use cargo install cargo-expand in it to use macrotest?)
* documentation style //! at lib.rs similar to README.md
* /// and unit tests at each files in src/
* Test it with real crate name and code instead.
