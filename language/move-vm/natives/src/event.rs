// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use libra_types::event::EventKey;
use move_core_types::gas_schedule::ZERO_GAS_UNITS;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::{NativeContext, NativeResult},
    values::Value,
};
use rand::{rngs::OsRng, RngCore};
use std::collections::VecDeque;
use vm::errors::PartialVMResult;

pub fn native_emit_event(
    context: &mut impl NativeContext,
    mut ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.len() == 1);
    debug_assert!(arguments.len() == 1);

    let mut rng = OsRng;
    context.save_event(
        EventKey::new_from_address(&context.sender(), rng.next_u64()).to_vec(),
        0,
        ty_args.pop().unwrap(),
        arguments.pop_back().unwrap(),
        context.caller().cloned(),
    )?;
    Ok(NativeResult::ok(ZERO_GAS_UNITS, vec![]))
}
