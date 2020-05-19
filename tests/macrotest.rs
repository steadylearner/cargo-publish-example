use macrotest::expand;

#[test]
pub fn macrotest() {
    expand("tests/expand/*.rs");
}