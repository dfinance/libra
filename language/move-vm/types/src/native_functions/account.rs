// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::loaded_data::types::Type;
use crate::native_functions::context::{Module, NativeContext};
use crate::native_functions::dispatch::{native_gas, NativeResult};
use crate::values::{Struct, Value};
use libra_types::account_address::AccountAddress;
use libra_types::account_config::{AccountResource, BalanceResource};
use libra_types::move_resource::MoveResource;
use libra_types::vm_error::{StatusCode, VMStatus};
use move_core_types::identifier::IdentStr;
use std::collections::VecDeque;
use vm::errors::VMResult;
use vm::gas_schedule::NativeCostIndex;

pub fn native_save_account(
    context: &mut impl NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> VMResult<NativeResult> {
    let cost = native_gas(context.cost_table(), NativeCostIndex::SAVE_ACCOUNT, 0);

    let account_module = context.self_module();
    let address = pop_arg!(arguments, AccountAddress);

    let id = account_module.self_id();
    if address == *id.address() {
        return Err(VMStatus::new(StatusCode::CREATE_NULL_ACCOUNT));
    }

    save_under_address(
        context,
        &[],
        &account_module,
        &AccountResource::struct_identifier(),
        pop_arg!(arguments, Struct),
        address,
    )?;
    save_under_address(
        context,
        &ty_args,
        &account_module,
        &BalanceResource::struct_identifier(),
        pop_arg!(arguments, Struct),
        address,
    )?;

    Ok(NativeResult::ok(cost, vec![]))
}

// Save a resource under the address specified by `account_address`
fn save_under_address(
    context: &mut impl NativeContext,
    ty_args: &[Type],
    module: &impl Module,
    struct_name: &IdentStr,
    resource_to_save: Struct,
    account_address: AccountAddress,
) -> VMResult<()> {
    let struct_ty = module.resolve_struct_def(context, struct_name, ty_args)?;
    let path = context.create_access_path(account_address, &struct_ty)?;
    context
        .interpreter_mut()
        .move_resource_to(&path, struct_ty, resource_to_save)
}
