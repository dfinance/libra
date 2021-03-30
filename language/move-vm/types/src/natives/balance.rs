use core::fmt;
use core::fmt::{Display, Formatter};

use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::StructTag;
use std::collections::HashMap;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub struct WalletId {
    pub address: AccountAddress,
    pub tag: StructTag,
}

impl WalletId {
    pub fn new(address: AccountAddress, tag: StructTag) -> WalletId {
        WalletId { address, tag }
    }
}

impl Display for WalletId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}::[{}::{}::{}]",
            self.address, self.tag.address, self.tag.module, self.tag.name
        )
    }
}

pub type Balance = u128;

pub trait NativeBalance {
    fn get_balance(&self, address: &WalletId) -> Option<Balance>;
}

pub struct ZeroBalance;
impl NativeBalance for ZeroBalance {
    fn get_balance(&self, _: &WalletId) -> Option<u128> {
        None
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BalanceOperation {
    Deposit(Balance),
    Withdraw(Balance),
}

impl BalanceOperation {
    pub fn empty() -> BalanceOperation {
        BalanceOperation::Deposit(0)
    }

    pub fn merge(&mut self, op: BalanceOperation) {
        let op = match (&self, op) {
            (BalanceOperation::Deposit(current), BalanceOperation::Deposit(change)) => {
                BalanceOperation::Deposit(*current + change)
            }
            (BalanceOperation::Withdraw(current), BalanceOperation::Withdraw(change)) => {
                BalanceOperation::Withdraw(*current + change)
            }
            (BalanceOperation::Deposit(current), BalanceOperation::Withdraw(change)) => {
                if *current >= change {
                    BalanceOperation::Deposit(*current - change)
                } else {
                    BalanceOperation::Withdraw(change - *current)
                }
            }
            (BalanceOperation::Withdraw(current), BalanceOperation::Deposit(change)) => {
                if *current >= change {
                    BalanceOperation::Withdraw(*current - change)
                } else {
                    BalanceOperation::Deposit(change - *current)
                }
            }
        };

        *self = op;
    }
}

pub struct MasterOfCoin {
    native_balances: Box<dyn NativeBalance>,
    bank: HashMap<WalletId, BalanceOperation>,
}

impl MasterOfCoin {
    pub fn new(native_balances: Box<dyn NativeBalance>) -> MasterOfCoin {
        MasterOfCoin {
            native_balances,
            bank: Default::default(),
        }
    }

    pub fn get_balance(&self, wallet_id: &WalletId) -> Option<Balance> {
        self.native_balances
            .get_balance(wallet_id)
            .map(|mut balance| {
                if let Some(op) = self.bank.get(wallet_id) {
                    match op {
                        BalanceOperation::Deposit(val) => {
                            balance -= *val;
                        }
                        BalanceOperation::Withdraw(val) => {
                            balance += *val;
                        }
                    }
                }
                balance
            })
            .or_else(|| {
                self.bank.get(wallet_id).and_then(|op| {
                    if let BalanceOperation::Withdraw(val) = op {
                        Some(*val)
                    } else {
                        None
                    }
                })
            })
    }

    pub fn save_balance_operation(&mut self, wallet_id: WalletId, op: BalanceOperation) {
        let entry = self.bank.entry(wallet_id);
        let current_op = entry.or_insert_with(BalanceOperation::empty);
        current_op.merge(op);
    }
}

impl From<MasterOfCoin> for HashMap<WalletId, BalanceOperation> {
    fn from(moc: MasterOfCoin) -> Self {
        moc.bank
    }
}
