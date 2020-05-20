use macrotest::expand;

// $cargo test macrotest
#[test]
pub fn macrotest() {
    expand("tests/expand/*.rs");
}