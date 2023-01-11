# Copyright © 2022 Brandon Li. All rights reserved.

# ========================================================= #
#  Workaround for specifying an exact clippy lint configuration.
#  See https://github.com/rust-lang/cargo/issues/5034
#
#  This file describes the configuration that is passed into
#  cargo clippy. Run ``make lint`` to use this configuration.
#
#  See https://github.com/rust-lang/rust-clippy#usage for
#  configuration options.
# ========================================================= #

# Deny all, then whitelist certain rules.
-D clippy::all
-D clippy::pedantic
-D clippy::nursery
-D clippy::cargo
-D clippy::correctness
-D clippy::suspicious
-D clippy::style
-D clippy::complexity
-D clippy::perf

# Allow list
-A clippy::range-plus-one
-A clippy::trivial_regex
-A clippy::derive-partial-eq-without-eq
-A clippy::needless_pass_by_value
-A clippy::module-inception
-A clippy::module-name-repetitions