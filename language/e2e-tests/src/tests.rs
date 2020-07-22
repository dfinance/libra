// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

//! Test module.
//!
//! Add new test modules to this list.
//!
//! This is not in a top-level tests directory because each file there gets compiled into a
//! separate binary. The linker ends up repeating a lot of work for each binary to not much
//! benefit.

pub mod account_universe;
pub mod admin_script;
pub mod create_account;
pub mod data_store;
pub mod execution_strategies;
pub mod failed_transaction_tests;
pub mod genesis;
pub mod mint;
pub mod module_publishing;
pub mod on_chain_configs;
pub mod peer_to_peer;
pub mod rotate_key;
pub mod scripts;
pub mod transaction_builder;
pub mod transaction_fees;
pub mod validator_set_management;
pub mod verify_txn;
pub mod write_set;
