// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0


use libra_genesis_tool::command::Command;
use structopt::StructOpt;

fn main() {
    println!("{}", Command::from_args().execute());
}
