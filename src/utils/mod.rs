// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! The utils module defines utility files that are too small to their own modules.

// pub mod bootstrapper;
pub mod cli_driver;
pub mod context;
pub mod error_messages;
pub mod lang_common;

/// In many places throughout the compiler, we want to use Hash tables for performance. However for testing, we need the
/// result of these steps to be deterministic. Create an aliased type that is stubbed based on the test environment.
#[cfg(not(feature = "test"))]
pub type Map<K, V> = std::collections::HashMap<K, V>;

#[cfg(feature = "test")]
pub type Map<K, V> = std::collections::BTreeMap<K, V>;

#[cfg(not(feature = "test"))]
pub type Set<T> = std::collections::HashSet<T>;

#[cfg(feature = "test")]
pub type Set<T> = std::collections::BTreeSet<T>;
