use byteorder::{ByteOrder, LittleEndian};
use libra_crypto::hash::DefaultHasher;
use libra_types::{access_path::AccessPath, vm_status::StatusCode};
use move_core_types::{
    gas_schedule::{GasAlgebra, GasUnits},
    language_storage::{TypeTag, CORE_CODE_ADDRESS},
};
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::{NativeContext, NativeResult},
    values::Value,
};
use std::{collections::VecDeque, hash::Hasher};
use twox_hash::XxHash64;
use vm::errors::{PartialVMError, PartialVMResult};

const COST: u64 = 929;
const PRICE_ORACLE_TAG: u8 = 255;

pub fn native_oracle_get_price(
    context: &impl NativeContext,
    ty_args: Vec<Type>,
    _arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    let ty_args = ty_args
        .into_iter()
        .map(|t| context.type_to_type_tag(&t))
        .collect::<Result<Vec<_>, _>>()?;

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
                    Ok(format!("{}_{}", ticker_part, ticker_part_2).to_lowercase())
                })
                .and_then(|ticker| {
                    let mut hash = XxHash64::default();
                    Hasher::write(&mut hash, ticker.as_bytes());
                    Ok(Hasher::finish(&hash))
                })
        })
        .map(|ticker| make_path(ticker))
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
                Err(status(StatusCode::RESOURCE_DOES_NOT_EXIST, "Price is not found"))
            }
        });

    let cost = GasUnits::new(COST);
    Ok(match price {
        Ok(price) => NativeResult::ok(cost, vec![Value::u64(price)]),
        Err(status) => NativeResult::err(cost, status.major_status() as u64),
    })
}

fn type_parameter_name(ty_arg: &TypeTag) -> Result<String, PartialVMError> {
    match ty_arg {
        TypeTag::Struct(t) => Ok(t.name.as_str().to_owned()),
        _ => Err(status(StatusCode::TYPE_MISMATCH, "Expected a struct")),
    }
}

fn status(code: StatusCode, msg: &str) -> PartialVMError {
    PartialVMError::new(code).with_message(msg.to_owned())
}

pub fn make_path(ticker_pair: u64) -> AccessPath {
    let mut hasher = DefaultHasher::new(&[]);
    let mut buf = [0; 8];
    LittleEndian::write_u64(&mut buf, ticker_pair);
    hasher.update(&buf);
    let mut hash = hasher.finish().to_vec();
    hash.insert(0, PRICE_ORACLE_TAG);
    AccessPath::new(CORE_CODE_ADDRESS, hash)
}
