use libra_types::account_config::CORE_CODE_ADDRESS;
use move_core_types::{identifier::Identifier, language_storage::ModuleId};
use move_vm_types::{
    gas_schedule::NativeCostIndex,
    loaded_data::runtime_types::Type,
    natives::function::{native_gas, NativeContext, NativeResult},
    values::{Struct, Value},
};
use once_cell::sync::Lazy;
use std::collections::VecDeque;
use vm::errors::VMResult;

pub static DFINANCE_MODULE: Lazy<ModuleId> =
    Lazy::new(|| ModuleId::new(CORE_CODE_ADDRESS, Identifier::new("Dfinance").unwrap()));

static INFO_NAME: Lazy<Identifier> = Lazy::new(|| Identifier::new("Info").unwrap());

pub fn native_register_token_info(
    context: &mut impl NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> VMResult<NativeResult> {
    let cost = native_gas(context.cost_table(), NativeCostIndex::SAVE_ACCOUNT, 0);

    context.save_under_address(
        &[ty_args[0].clone()],
        &DFINANCE_MODULE,
        &INFO_NAME,
        pop_arg!(arguments, Struct),
        CORE_CODE_ADDRESS,
    )?;
    Ok(NativeResult::ok(cost, vec![]))
}
