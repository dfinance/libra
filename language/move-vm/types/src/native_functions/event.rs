use crate::loaded_data::types::Type;
use crate::native_functions::context::NativeContext;
use crate::native_functions::dispatch::NativeResult;
use crate::values::Value;
use libra_types::contract_event::ContractEvent;
use libra_types::{
    event::EventKey,
    vm_error::{StatusCode, VMStatus},
};
use std::collections::VecDeque;
use std::convert::TryFrom;
use vm::errors::VMResult;
use vm::gas_schedule::ZERO_GAS_UNITS;

pub fn native_emit_event(
    context: &mut impl NativeContext,
    mut ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> VMResult<NativeResult> {
    if ty_args.len() != 1 {
        return Err(
            VMStatus::new(StatusCode::VERIFIER_INVARIANT_VIOLATION).with_message(format!(
                "write_to_event_storage expects 1 type argument got {}.",
                ty_args.len()
            )),
        );
    }

    let ty = ty_args.pop().unwrap();
    let msg = arguments
        .pop_back()
        .unwrap()
        .simple_serialize(&ty)
        .ok_or_else(|| VMStatus::new(StatusCode::DATA_FORMAT_ERROR))?;

    let count = pop_arg!(arguments, u64);
    let key = pop_arg!(arguments, Vec<u8>);

    let guid = EventKey::try_from(key.as_slice())
        .map_err(|_| VMStatus::new(StatusCode::EVENT_KEY_MISMATCH))?;
    context
        .interpreter_mut()
        .push_event(ContractEvent::new(guid, count, ty.into_type_tag()?, msg));
    Ok(NativeResult::ok(ZERO_GAS_UNITS, vec![]))
}
