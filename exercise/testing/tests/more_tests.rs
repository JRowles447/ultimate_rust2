use testing::{splish, sploosh};
// 5. Create a `tests/` directory and an integration test file `tests/more_tests.rs`
// Inside that file, create a test function that verifies:
// - that `sploosh(splish(-1, 0), splish(1, 1), splish(3, 2))` returns the value `4`
//
// `cargo test` should run your `more_tests.rs` file and pass

#[test]
pub fn integration_test() {
    assert!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)) == 4)
}
