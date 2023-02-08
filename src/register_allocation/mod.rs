// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! The `register_allocation` module performs analysis needed for the register allocation optimization.

pub mod conflict_analysis;
pub mod liveness_analysis;
pub mod register_allocator;
