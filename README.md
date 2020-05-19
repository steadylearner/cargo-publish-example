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

- Each structs and enum created from them are completely separated except they have the same fields.

<br>

## Comparison to attribute macros

[You can do the same with attribute macros.](https://github.com/steadylearner/Rust-Full-Stack/tree/master/macro/attribute) But, you will need to have more dependencies.

[If you want more information, please read the official documenation about procedural macros.](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)

<br>

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

* cargo fmt, cargo clippy etc with some script files before publish?
* documentation style //! at lib.rs similar to README.md
* /// and unit tests at each files in src/
* Include dev dependencies to expand and test macros?
* Include Travis CI.(How to use cargo install cargo-expand in it to use macrotest?)
* Test it with real crate name and code instead.
