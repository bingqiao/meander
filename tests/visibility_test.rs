#[test]
fn private_items_are_not_accessible() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}
