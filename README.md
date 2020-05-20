[trybuild]: https://github.com/dtolnay/trybuild
[macrotest]: https://github.com/eupn/macrotest

reuse(Struct, Enum)
=============

It provides functional macros to reuse fields with [Struct](https://doc.rust-lang.org/std/keyword.struct.html) and [Enum](https://doc.rust-lang.org/std/keyword.enum.html).

```toml
[dependencies]
publish = "0.0.0"
```

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
); // It is lazy. Nothing is made yet.

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
        PageUnload, // , here is required if you want to extend the fields later.
    }
); // It is lazy. Nothing is made yet.

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

- Each struct and enum created from them are completely unrelevant except they have the same fields you define.

- When you use `private_struct!` and `private_enum!`, you can't use pub keyword in it and others use them. [It wouldn't be logical if private struct or private enum can have public fields at all.](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#making-structs-and-enums-public)

- `nested_macro!` is required to use the other macros from this crate. It helps the macros created from this crate to cutomize name etc.

```rust
macro_rules! nested_macro {
    ($($body:tt)*) => {
        macro_rules! __nested_macro { $($body)+ }
        __nested_macro!($);
    }
}
```

<br>

## Comparison with attribute macros

[You can reuse the fields with attribute macros also.](https://github.com/steadylearner/Rust-Full-Stack/tree/master/macro/attribute) But, you need to install more dependencies.

[If you want more, please read the official documenation about procedural macros.](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)

<br>

## How to test it

```console
$git clone git@github.com:steadylearner/publish.git && cargo test pass
```

1. `$cargo test pass` to run passing tests.
2. `$cargo test fail` to run failing tests. You need to install[trybuild] first.

If you want to see how macros from this package expand, use `$cargo test macros`. You need to install [rustfmt](https://github.com/rust-lang/rustfmt) and [cargo-expand](https://github.com/dtolnay/cargo-expand) to use it.

```console
$rustup component add rustfmt && cargo install cargo-expand
```

[macrotest] is based on [trybuild]. It make the tests recompile everytime and take very long if you would test them in a single command.

That is why there are separate test commands to save your time.

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

* [cargo fmt](https://github.com/rust-lang/rustfmt), [cargo clippy](https://github.com/rust-lang/rust-clippy) etc with some script files before publish? Include some commands to help the development. Include Travis CI.(How to use cargo install cargo-expand in it to use macrotest or exclude expand/ and pass/?)

* documentation style //! at lib.rs similar to README.md. /// and unit tests at each files in src/

* Test it with real crate name and code instead.
