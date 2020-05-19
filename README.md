publish(Struct, Enum)
=============

This library provides a convenient functional macros to reuse fields with Struct and Enum.

<br>

## Example

```rust
use publish::struct, enum;

struct!()
enum!()

fn main() {
  
}

```

<br>

## Details

- The functional macros from this crate will help you to reuse fields in a struct and enum. No real inherit happens. So, you don't need to worry about the relevant problem.

<br>

## Comparison to attribute

[You can do the same with attribute features from Rust.](https://github.com/steadylearner/Rust-Full-Stack/tree/master/macro/attribute) But, you will need to have more dependencies.

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