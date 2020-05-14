// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

pub mod cached_storage;
pub mod crypto_kv_storage;
pub mod crypto_storage;
pub mod error;
pub mod github;
pub mod in_memory;
pub mod kv_storage;
pub mod namespaced_storage;
pub mod on_disk;
pub mod policy;
pub mod storage;
pub mod value;
pub mod vault;

pub use crate::{
    cached_storage::CachedStorage,
    crypto_kv_storage::CryptoKVStorage,
    crypto_storage::{CryptoStorage, PublicKeyResponse},
    error::Error,
    github::GitHubStorage,
    in_memory::{InMemoryStorage, InMemoryStorageInternal},
    kv_storage::{GetResponse, KVStorage},
    namespaced_storage::NamespacedStorage,
    on_disk::{OnDiskStorage, OnDiskStorageInternal},
    policy::{Capability, Identity, Permission, Policy},
    storage::Storage,
    value::Value,
    vault::VaultStorage,
};

#[cfg(test)]
pub mod tests;
