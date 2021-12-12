// Integration tests: make sure all modules work together as expected
// `cargo test --test integration_tests` runs all tests in this crate

// These are compiled as a separate crate so must bring mods into scope
use c11_testing;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, c11_testing::add_two(2));
}

// BIG CAVEAT: binary crates (contain main.rs) dont allow integration tests
// to import main. This is why main usually separates out into different module
// files so we can import those into the integration tests.