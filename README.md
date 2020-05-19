Reuse(Struct, Enum)
=============

This library provides a convenient functional macros to reuse fields with Struct and Enum.

<br>

## Example

### Struct

```rust
use publish::{
    public_struct, 
};

public_struct!()

fn main() {

}
```

### Enum

```rust
use publish::{
    public_enum,
};

public_enum!()

fn main() {

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
