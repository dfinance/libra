// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use consensus_types::block::Block;
use libra_crypto::HashValue;
use libra_types::transaction::Transaction;

pub mod execution_correctness;
pub mod execution_correctness_manager;
pub mod local;
pub mod process;
pub mod remote_service;
pub mod serializer;
pub mod spawned_process;
pub mod thread;

pub use crate::{
    execution_correctness::ExecutionCorrectness,
    execution_correctness_manager::ExecutionCorrectnessManager, process::Process,
};

#[cfg(test)]
#[path = "process_client_wrapper.rs"]
pub mod process_client_wrapper;

#[cfg(test)]
pub mod tests;

pub fn id_and_transactions_from_block(block: &Block) -> (HashValue, Vec<Transaction>) {
    let id = block.id();
    let mut transactions = vec![Transaction::BlockMetadata(block.into())];
    transactions.extend(
        block
            .payload()
            .unwrap_or(&vec![])
            .iter()
            .map(|txn| Transaction::UserTransaction(txn.clone())),
    );
    (id, transactions)
}
