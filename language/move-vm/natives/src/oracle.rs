use move_vm_types::natives::function::{NativeContext, NativeResult};
use move_vm_types::loaded_data::runtime_types::Type;
use move_vm_types::values::Value;
use libra_types::access_path::AccessPath;
use libra_types::account_address::AccountAddress;
use libra_types::vm_error::{VMStatus, StatusCode};
use libra_crypto::hash::{DefaultHasher, CryptoHasher};
use std::collections::VecDeque;
use vm::errors::VMResult;
use byteorder::{LittleEndian, ByteOrder};
use move_core_types::{
    gas_schedule::{GasUnits, GasAlgebra},
};

const COST: u64 = 929;
const PRICE_ORACLE_TAG: u8 = 255;

pub fn native_oracle_get_price(
    context: &impl NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> VMResult<NativeResult> {
    if arguments.len() != 1 {
        let msg = format!(
            "wrong number of arguments for get_price expected 1 found {}",
            arguments.len()
        );
        return Err(status(StatusCode::UNREACHABLE, &msg));
    }

    let ticker = pop_arg!(arguments, u64);
    let price =
        make_path(ticker)
        .and_then(|path| {
            let value = context.raw_load(&path).map_err(|err| {
                status(
                    StatusCode::STORAGE_ERROR,
                    &format!("Failed to load ticker [{}]", err),
                )
            })?;

            if let Some(price) = value {
                if price.len() != 8 {
                    Err(status(StatusCode::TYPE_MISMATCH, "Invalid prise size"))
                } else {
                    Ok(LittleEndian::read_u64(&price))
                }
            } else {
                Err(status(StatusCode::STORAGE_ERROR, "Price is not found"))
            }
        });

    let cost = GasUnits::new(COST);
    Ok(match price {
        Ok(price) => NativeResult::ok(cost, vec![Value::u64(price)]),
        Err(status) => NativeResult::err(cost, status),
    })
}

fn status(code: StatusCode, msg: &str) -> VMStatus {
    VMStatus::new(code).with_message(msg.to_owned())
}

pub fn make_path(ticker_pair: u64) -> Result<AccessPath, VMStatus> {
    let mut hasher = DefaultHasher::default();
    let mut buf = [0; 8];
    LittleEndian::write_u64(&mut buf, ticker_pair);
    hasher.write(&buf);
    let mut hash = hasher.finish().to_vec();
    hash.insert(0, PRICE_ORACLE_TAG);
    Ok(AccessPath::new(AccountAddress::DEFAULT, hash))
}

