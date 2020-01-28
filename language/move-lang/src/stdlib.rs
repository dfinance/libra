use crate::errors::{FilesSourceText, Errors};
use std::collections::HashMap;
use crate::{strip_comments_and_verify, parser, check_program, compile_program};
use std::io;
use crate::parser::syntax::parse_file_string;
use crate::parser::ast::FileDefinition;
use crate::shared::Address;
use crate::errors;

pub fn stdlib() -> Vec<&'static str> {
    vec![
        include_str!("../stdlib/modules/address_util.move"),
        include_str!("../stdlib/modules/block.move"),
        include_str!("../stdlib/modules/bytearray_util.move"),
        include_str!("../stdlib/modules/event.move"),
        include_str!("../stdlib/modules/hash.move"),
        include_str!("../stdlib/modules/libra_account.move"),
        include_str!("../stdlib/modules/libra_coin.move"),
        include_str!("../stdlib/modules/signature.move"),
        include_str!("../stdlib/modules/transaction.move"),
        include_str!("../stdlib/modules/transaction_fee_distribution.move"),
        include_str!("../stdlib/modules/u64_util.move"),
        include_str!("../stdlib/modules/validator_config.move"),
        include_str!("../stdlib/modules/validator_set.move"),
        include_str!("../stdlib/modules/vector.move"),
    ]
}