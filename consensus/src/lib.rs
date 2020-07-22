// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0



//! Consensus for the Libra Core blockchain
//!
//! The consensus protocol implemented is LibraBFT (based on
//! [HotStuff](https://arxiv.org/pdf/1803.05069.pdf)).

#![cfg_attr(not(feature = "fuzzing"),)]
#![cfg_attr(feature = "fuzzing", allow(dead_code))]
#![recursion_limit = "512"]

pub mod block_storage;
pub mod consensusdb;
pub mod counters;
pub mod epoch_manager;
pub mod liveness;
pub mod metrics_safety_rules;
pub mod network;
#[cfg(test)]
pub mod network_tests;
pub mod pending_votes;
pub mod persistent_liveness_storage;
pub mod round_manager;
pub mod state_computer;
pub mod state_replication;
#[cfg(any(test, feature = "fuzzing"))]
pub mod test_utils;
#[cfg(test)]
pub mod twins_test;
pub mod txn_manager;
pub mod util;

/// LibraBFT implementation
pub mod consensus_provider;
/// LibraNet interface.
pub mod network_interface;

#[cfg(feature = "fuzzing")]
pub use round_manager::round_manager_fuzzing;
pub use util::config_subscription::gen_consensus_reconfig_subscription;
