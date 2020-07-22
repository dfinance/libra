// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

//! JSON RPC endpoint
//!
//! Used as public API interface for interacting with Full Nodes
//! It serves HTTP API requests from various external clients (such as wallets)
//!
//! Protocol specification: https://www.jsonrpc.org/specification
//!
//! Module organization:
//! ├── methods.rs        # contains all available JSON RPC method handlers
//! ├── runtime.rs        # implementation of JSON RPC protocol over HTTP
//! ├── tests.rs          # tests

#[macro_use]
pub mod util;

pub mod counters;
pub mod methods;
pub mod runtime;

pub use libra_json_rpc_types::{errors, response, views};

pub use runtime::{bootstrap, bootstrap_from_config};

#[cfg(any(feature = "fuzzing", test))]
/// Fuzzer for JSON RPC service
pub mod fuzzing;
#[cfg(any(test, feature = "fuzzing"))]
pub mod tests;
#[cfg(any(test, feature = "fuzzing"))]
pub use tests::test_bootstrap;
