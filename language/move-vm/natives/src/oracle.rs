use byteorder::{ByteOrder, LittleEndian};
use libra_crypto::hash::{CryptoHasher, DefaultHasher};
use libra_types::{
    access_path::AccessPath,
    account_address::AccountAddress,
    vm_error::{StatusCode, VMStatus},
};
use move_core_types::gas_schedule::{GasAlgebra, GasUnits};
use move_vm_types::{
    loaded_data::{runtime_types::Type, types::FatType},
    natives::function::{NativeContext, NativeResult},
    values::Value,
};
use std::{collections::VecDeque, hash::Hasher, io::Write};
use twox_hash::XxHash64;
use vm::errors::VMResult;

const COST: u64 = 929;
const PRICE_ORACLE_TAG: u8 = 255;

pub fn native_oracle_get_price(
    context: &impl NativeContext,
    ty_args: Vec<Type>,
    _arguments: VecDeque<Value>,
) -> VMResult<NativeResult> {
    let ty_args = context.convert_to_fat_types(ty_args)?;

    if ty_args.len() != 2 {
        let msg = format!(
            "wrong number of type parameters for get_price expected 2 found {}",
            ty_args.len()
        );
        return Err(status(StatusCode::UNREACHABLE, &msg));
    }

    let price = type_parameter_name(&ty_args[0])
        .and_then(|ticker_part| {
            type_parameter_name(&ty_args[1])
                .and_then(|ticker_part_2| {
                    Ok(format!("{}{}", ticker_part, ticker_part_2).to_lowercase())
                })
                .and_then(|ticker| {
                    let mut hash = XxHash64::default();
                    Hasher::write(&mut hash, ticker.as_bytes());
                    Ok(Hasher::finish(&hash))
                })
        })
        .and_then(|ticker| make_path(ticker))
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

fn type_parameter_name(ty_arg: &FatType) -> Result<String, VMStatus> {
    match ty_arg {
        FatType::Struct(t) => Ok(t.name.as_str().to_owned()),
        _ => Err(status(StatusCode::TYPE_MISMATCH, "Expected a struct")),
    }
}

fn status(code: StatusCode, msg: &str) -> VMStatus {
    VMStatus::new(code).with_message(msg.to_owned())
}

pub fn make_path(ticker_pair: u64) -> Result<AccessPath, VMStatus> {
    let mut hasher = DefaultHasher::default();
    let mut buf = [0; 8];
    LittleEndian::write_u64(&mut buf, ticker_pair);
    hasher
        .write(&buf)
        .map_err(|_| VMStatus::new(StatusCode::BAD_U64))?;
    let mut hash = hasher.finish().to_vec();
    hash.insert(0, PRICE_ORACLE_TAG);
    Ok(AccessPath::new(AccountAddress::DEFAULT, hash))
}
