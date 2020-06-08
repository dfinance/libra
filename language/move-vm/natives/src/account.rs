// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use libra_types::{
    account_address::AccountAddress,
    account_config,
    account_config::{BalanceResource, CORE_CODE_ADDRESS},
    vm_error::{StatusCode, VMStatus},
};
use move_core_types::{identifier::Identifier, move_resource::MoveResource};
use move_vm_types::{
    gas_schedule::NativeCostIndex,
    loaded_data::runtime_types::Type,
    natives::function::{native_gas, NativeContext, NativeResult},
    values::{Struct, Value},
};
use once_cell::sync::Lazy;
use std::collections::VecDeque;
use vm::errors::VMResult;

static ACCOUNT_NAME: Lazy<Identifier> = Lazy::new(|| Identifier::new("T").unwrap());

pub fn native_save_account(
    context: &mut impl NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> VMResult<NativeResult> {
    let cost = native_gas(context.cost_table(), NativeCostIndex::SAVE_ACCOUNT, 0);

    let address = pop_arg!(arguments, AccountAddress);

    if address == CORE_CODE_ADDRESS {
        return Err(VMStatus::new(StatusCode::CREATE_NULL_ACCOUNT));
    }

    context.save_under_address(
        &[],
        &account_config::EVENT_MODULE,
        account_config::event_handle_generator_struct_name(),
        pop_arg!(arguments, Struct),
        address,
    )?;
    context.save_under_address(
        &[],
        &account_config::ACCOUNT_MODULE,
        &ACCOUNT_NAME,
        pop_arg!(arguments, Struct),
        address,
    )?;
    Ok(NativeResult::ok(cost, vec![]))
}

pub fn native_save_balance(
    context: &mut impl NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> VMResult<NativeResult> {
    let cost = native_gas(context.cost_table(), NativeCostIndex::SAVE_ACCOUNT, 0);

    let address = pop_arg!(arguments, AccountAddress);

    if address == CORE_CODE_ADDRESS {
        return Err(VMStatus::new(StatusCode::CREATE_NULL_ACCOUNT));
    }

    context.save_under_address(
        &[ty_args[0].clone()],
        &account_config::ACCOUNT_MODULE,
        &BalanceResource::struct_identifier(),
        pop_arg!(arguments, Struct),
        address,
    )?;
    Ok(NativeResult::ok(cost, vec![]))
}
