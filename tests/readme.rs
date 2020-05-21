// https://crates.io/crates/doc-comment

use doc_comment::doctest;

#[test]
fn readme() {
    doctest!("../README.md");
    doctest!("../README_FAIL.md");
}