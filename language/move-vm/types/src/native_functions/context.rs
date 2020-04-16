// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::interpreter_context::InterpreterContext;
use crate::loaded_data::types::{StructType, Type};
use libra_types::access_path::AccessPath;
use libra_types::account_address::AccountAddress;
use libra_types::language_storage::ModuleId;
use move_core_types::identifier::IdentStr;
use std::fmt::Write;
use vm::errors::VMResult;
use vm::gas_schedule::CostTable;

/// Native function context.
pub trait NativeContext {
    type Mod: Module;

    fn caller_module(&self) -> Self::Mod;
    fn self_module(&self) -> Self::Mod;
    /// Get the AccessPath to a resource stored under `address` with type `struct_ty`
    fn create_access_path(
        &self,
        account_address: AccountAddress,
        struct_ty: &StructType,
    ) -> VMResult<AccessPath>;
    fn interpreter(&self) -> &dyn InterpreterContext;
    fn interpreter_mut(&mut self) -> &mut dyn InterpreterContext;
    fn debug_print_stack_trace<B: Write>(&self, buf: &mut B) -> VMResult<()>;
    fn cost_table(&self) -> &CostTable;
}

pub trait Module {
    /// Gets module id.
    fn self_id(&self) -> ModuleId;
    /// Resolve a StructType by struct name.
    fn resolve_struct_def(
        &self,
        module: &impl NativeContext,
        struct_name: &IdentStr,
        ty_args: &[Type],
    ) -> VMResult<StructType>;
}
