Reuse(Struct, Enum)
=============

This library provides convenient functional macros to reuse fields with Struct and Enum.

<br>

## Example

### Struct

```rust
use publish::{
    nested_macro,
    public_struct,
};

public_struct!(
    pub struct MessageBase {
        pub text: String
    }
);

MessageBase!(); // You have to call it to use.

fn main() {
     // You have to call it to use.
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
    pub enum WebEventBase {
        PageLoad,
        PageUnload, // , here is required.
    }
);

WebEventBase!(); // You have to call it to use.

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

- The functional macros from this crate will help you to reuse fields in a struct and enum. No real inherit happens. So, you don't need to worry about the relevant problem.

<br>

## Comparison to attribute macros

[You can do the same with attribute macros from Rust.](https://github.com/steadylearner/Rust-Full-Stack/tree/master/macro/attribute) But, you will need to have more dependencies.

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

* documentation style //! at lib.rs similar to README.md
* /// and unit tests at each files in src/
* Include Travis CI.
* Test a publish with real crate name and code.
* Include dev dependencies to expand and test macros?
