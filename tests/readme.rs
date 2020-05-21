// https://crates.io/crates/doc-comment

use doc_comment::doctest;

#[test]
pub fn readme() {
    doctest!("../README_FAIL.md");
    // doctest!("../README.md");
}