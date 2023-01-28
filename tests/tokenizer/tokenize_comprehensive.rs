// Copyright © 2022 Brandon Li. All rights reserved.

//! Overall test for tokenizing a Solis program.

use expect_test::expect;
use test_utils::tokenize_check;

#[test]
fn test_comprehenzive() {
    tokenize_check(
        r"
        ",
        expect![],
    )
}
