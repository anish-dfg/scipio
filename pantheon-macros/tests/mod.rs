#[test]
pub fn test_macros() {
    let t = trybuild::TestCases::new();
    t.pass("tests/test_build_query.rs");
}
