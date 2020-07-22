// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0



//! The core Move VM logic.
//!
//! It is a design goal for the Move VM to be independent of the Libra blockchain, so that
//! other blockchains can use it as well. The VM isn't there yet, but hopefully will be there
//! soon.

#[macro_use]
extern crate mirai_annotations;

pub mod data_cache;
pub mod data_operations;
pub mod interpreter;
pub mod loader;
pub mod move_vm;
pub mod native_functions;
pub mod runtime;
pub mod session;
#[macro_use]
pub mod tracing;

#[cfg(test)]
pub mod unit_tests;
