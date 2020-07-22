// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

pub mod block;
pub mod quorum_certificate;
pub mod single_entry;

use anyhow::{ensure, Result};
use schemadb::ColumnFamilyName;

pub const BLOCK_CF_NAME: ColumnFamilyName = "block";
pub const QC_CF_NAME: ColumnFamilyName = "quorum_certificate";
pub const SINGLE_ENTRY_CF_NAME: ColumnFamilyName = "single_entry";

pub fn ensure_slice_len_eq(data: &[u8], len: usize) -> Result<()> {
    ensure!(
        data.len() == len,
        "Unexpected data len {}, expected {}.",
        data.len(),
        len,
    );
    Ok(())
}
