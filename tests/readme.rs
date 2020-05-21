// https://crates.io/crates/doc-comment

#[macro_use] extern crate doc_comment;

#[test]
pub fn readme() {
    doctest!("../README_FAIL.md");
    // doctest!("../README.md");
}