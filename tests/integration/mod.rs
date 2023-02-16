// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Integration tests module.

use assert_cmd::Command;
use std::fs;

// Runs a given integration test.
// * name - the name of the integration test, which corresponds to a file in the `integration` directory.
fn run_integration_test(integration_test_name: &str) {
    // Get the expected output (stdout) of the solis file, in `integration/expected`
    let expected_output =
        fs::read_to_string(format!("./tests/integration/expected/{integration_test_name}.out")).unwrap();

    Command::cargo_bin("solis")
        .unwrap()
        .arg(format!("./tests/integration/{integration_test_name}.sol"))
        .arg("-d")
        .arg("./build/solis_tests/")
        .arg("-n")
        .arg(integration_test_name)
        .arg("-r")
        .assert()
        .success()
        .stdout(expected_output);
}

// Macro to create a test function for each registered integration test.
macro_rules! gen_integration_tests {
    ($($integration_test_name:ident), *) => {
        $(
            #[test]
            fn $integration_test_name() {
                run_integration_test(stringify!($integration_test_name))
            }
        )*
    }
}

gen_integration_tests!(
    ints_and_bools_basic_1,
    ints_and_bools_random_1,
    ints_and_bools_random_2,
    ints_and_bools_random_3,
    ints_and_bools_random_4
);
