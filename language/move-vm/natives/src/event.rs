// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use libra_types::{
    contract_event::ContractEvent,
    event::EventKey,
    vm_error::{StatusCode, VMStatus},
};
use move_core_types::gas_schedule::ZERO_GAS_UNITS;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::{NativeContext, NativeResult},
    values::Value,
};
use std::collections::VecDeque;
use vm::errors::VMResult;
use rand::rngs::OsRng;
use rand::RngCore;

pub fn native_emit_event(
    context: &mut impl NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> VMResult<NativeResult> {
    debug_assert!(ty_args.len() == 1);
    debug_assert!(arguments.len() == 1);

    let mut ty_args = context.convert_to_fat_types(ty_args)?;
    let ty = ty_args.pop().unwrap();

    let msg = arguments
        .pop_back()
        .unwrap()
        .simple_serialize(&ty)
        .ok_or_else(|| VMStatus::new(StatusCode::DATA_FORMAT_ERROR))?;

    let mut rng = OsRng;
    let event = ContractEvent::with_caller(
        EventKey::new_from_address(&context.sender(), rng.next_u64()),
        0,
        ty.type_tag()?,
        msg,
        context.caller().cloned(),
    );

    context.save_event(event)?;
    Ok(NativeResult::ok(ZERO_GAS_UNITS, vec![]))
}
