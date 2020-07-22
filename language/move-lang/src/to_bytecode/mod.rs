// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

#[macro_use]
pub mod context;
pub mod remove_fallthrough_jumps;
pub mod translate;

use move_ir_types::ast as IR;
use std::collections::HashMap;

pub fn remap_labels(blocks: &mut IR::BytecodeBlocks, map: &HashMap<IR::BlockLabel, IR::BlockLabel>) {
    use IR::Bytecode_ as B;
    for (_, block) in blocks {
        for instr in block {
            match &mut instr.value {
                B::Branch(lbl) | B::BrTrue(lbl) | B::BrFalse(lbl) => {
                    *lbl = map[lbl].clone();
                }
                _ => (),
            }
        }
    }
}
