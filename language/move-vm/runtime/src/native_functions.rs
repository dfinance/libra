// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{interpreter::Interpreter, loader::Resolver, logging::LogContext};
use move_core_types::{
    account_address::AccountAddress, gas_schedule::CostTable, language_storage::CORE_CODE_ADDRESS,
    value::MoveTypeLayout, vm_status::StatusType,
};
use move_vm_natives::{account, bcs, debug, event_dfi, hash, signature, signer, vector, event};
use move_vm_types::{
    data_store::DataStore,
    gas_schedule::CostStrategy,
    loaded_data::runtime_types::Type,
    natives::function::{NativeContext, NativeResult},
    values::Value,
};
use std::{collections::VecDeque, fmt::Write};
use vm::errors::PartialVMResult;
use move_core_types::language_storage::ModuleId;

// The set of native functions the VM supports.
// The functions can line in any crate linked in but the VM declares them here.
// 2 functions have to be implemented for a `NativeFunction`:
// - `resolve` which given a function unique name ModuleAddress::ModuleName::FunctionName
// returns a `NativeFunction`
// - `dispatch` which given a `NativeFunction` invokes the native
#[derive(Debug, Clone, Copy)]
pub(crate) enum NativeFunction {
    HashSha2_256,
    HashSha3_256,
    BCSToBytes,
    PubED25519Validate,
    SigED25519Verify,
    VectorLength,
    VectorEmpty,
    VectorBorrow,
    VectorBorrowMut,
    VectorPushBack,
    VectorPopBack,
    VectorDestroyEmpty,
    VectorSwap,
    AccountWriteEvent,
    AccountEmitEvent,
    DebugPrint,
    DebugPrintStackTrace,
    SignerBorrowAddress,
    CreateSigner,
    DestroySigner,
    DfinanceCreateSigner,
    DfinanceDestroySigner,
}

impl NativeFunction {
    pub(crate) fn resolve(
        module_address: &AccountAddress,
        module_name: &str,
        function_name: &str,
    ) -> Option<NativeFunction> {
        use NativeFunction::*;

        let case = (module_address, module_name, function_name);
        Some(match case {
            (&CORE_CODE_ADDRESS, "Hash", "sha2_256") => HashSha2_256,
            (&CORE_CODE_ADDRESS, "Hash", "sha3_256") => HashSha3_256,
            (&CORE_CODE_ADDRESS, "BCS", "to_bytes") => BCSToBytes,
            (&CORE_CODE_ADDRESS, "Signature", "ed25519_validate_pubkey") => PubED25519Validate,
            (&CORE_CODE_ADDRESS, "Signature", "ed25519_verify") => SigED25519Verify,
            (&CORE_CODE_ADDRESS, "Vector", "length") => VectorLength,
            (&CORE_CODE_ADDRESS, "Vector", "empty") => VectorEmpty,
            (&CORE_CODE_ADDRESS, "Vector", "borrow") => VectorBorrow,
            (&CORE_CODE_ADDRESS, "Vector", "borrow_mut") => VectorBorrowMut,
            (&CORE_CODE_ADDRESS, "Vector", "push_back") => VectorPushBack,
            (&CORE_CODE_ADDRESS, "Vector", "pop_back") => VectorPopBack,
            (&CORE_CODE_ADDRESS, "Vector", "destroy_empty") => VectorDestroyEmpty,
            (&CORE_CODE_ADDRESS, "Vector", "swap") => VectorSwap,
            (&CORE_CODE_ADDRESS, "Event", "write_to_event_store") => AccountWriteEvent,
            (&CORE_CODE_ADDRESS, "Event", "emit") => AccountEmitEvent,
            (&CORE_CODE_ADDRESS, "DiemAccount", "create_signer") => CreateSigner,
            (&CORE_CODE_ADDRESS, "DiemAccount", "destroy_signer") => DestroySigner,
            (&CORE_CODE_ADDRESS, "Account", "create_signer") => CreateSigner,
            (&CORE_CODE_ADDRESS, "Account", "destroy_signer") => DestroySigner,
            (&CORE_CODE_ADDRESS, "Debug", "print") => DebugPrint,
            (&CORE_CODE_ADDRESS, "Debug", "print_stack_trace") => DebugPrintStackTrace,
            (&CORE_CODE_ADDRESS, "Signer", "borrow_address") => SignerBorrowAddress,
            (&CORE_CODE_ADDRESS, "Dfinance", "create_signer") => DfinanceCreateSigner,
            (&CORE_CODE_ADDRESS, "Dfinance", "destroy_signer") => DfinanceDestroySigner,
            _ => return None,
        })
    }

    /// Given the vector of aguments, it executes the native function.
    pub(crate) fn dispatch(
        self,
        ctx: &mut impl NativeContext,
        t: Vec<Type>,
        v: VecDeque<Value>,
    ) -> PartialVMResult<NativeResult> {
        let result = match self {
            Self::HashSha2_256 => hash::native_sha2_256(ctx, t, v),
            Self::HashSha3_256 => hash::native_sha3_256(ctx, t, v),
            Self::PubED25519Validate => signature::native_ed25519_publickey_validation(ctx, t, v),
            Self::SigED25519Verify => signature::native_ed25519_signature_verification(ctx, t, v),
            Self::VectorLength => vector::native_length(ctx, t, v),
            Self::VectorEmpty => vector::native_empty(ctx, t, v),
            Self::VectorBorrow => vector::native_borrow(ctx, t, v),
            Self::VectorBorrowMut => vector::native_borrow(ctx, t, v),
            Self::VectorPushBack => vector::native_push_back(ctx, t, v),
            Self::VectorPopBack => vector::native_pop(ctx, t, v),
            Self::VectorDestroyEmpty => vector::native_destroy_empty(ctx, t, v),
            Self::VectorSwap => vector::native_swap(ctx, t, v),
            // natives that need the full API of `NativeContext`
            Self::AccountWriteEvent => event::native_emit_event(ctx, t, v),
            Self::AccountEmitEvent => event_dfi::native_emit_event(ctx, t, v),
            Self::BCSToBytes => bcs::native_to_bytes(ctx, t, v),
            Self::DebugPrint => debug::native_print(ctx, t, v),
            Self::DebugPrintStackTrace => debug::native_print_stack_trace(ctx, t, v),
            Self::SignerBorrowAddress => signer::native_borrow_address(ctx, t, v),
            Self::CreateSigner => account::native_create_signer(ctx, t, v),
            Self::DestroySigner => account::native_destroy_signer(ctx, t, v),
            Self::DfinanceCreateSigner => account::native_create_signer(ctx, t, v),
            Self::DfinanceDestroySigner => account::native_destroy_signer(ctx, t, v),
        };
        debug_assert!(match &result {
            Err(e) => e.major_status().status_type() == StatusType::InvariantViolation,
            Ok(_) => true,
        });
        result
    }
}

pub(crate) struct FunctionContext<'a, L: LogContext> {
    interpreter: &'a mut Interpreter<L>,
    data_store: &'a mut dyn DataStore,
    cost_strategy: &'a CostStrategy<'a>,
    resolver: &'a Resolver<'a>,
    caller: Option<&'a ModuleId>,
}

impl<'a, L: LogContext> FunctionContext<'a, L> {
    pub(crate) fn new(
        interpreter: &'a mut Interpreter<L>,
        data_store: &'a mut dyn DataStore,
        cost_strategy: &'a mut CostStrategy,
        resolver: &'a Resolver<'a>,
        caller: Option<&'a ModuleId>,
    ) -> FunctionContext<'a, L> {
        FunctionContext {
            interpreter,
            data_store,
            cost_strategy,
            resolver,
            caller,
        }
    }
}

impl<'a, L: LogContext> NativeContext for FunctionContext<'a, L> {
    fn print_stack_trace<B: Write>(&self, buf: &mut B) -> PartialVMResult<()> {
        self.interpreter
            .debug_print_stack_trace(buf, self.resolver.loader())
    }

    fn cost_table(&self) -> &CostTable {
        self.cost_strategy.cost_table()
    }

    fn save_event(
        &mut self,
        guid: Vec<u8>,
        seq_num: u64,
        ty: Type,
        val: Value,
        caller: Option<ModuleId>
    ) -> PartialVMResult<bool> {
        match self.data_store.emit_event(guid, seq_num, ty, val, caller) {
            Ok(()) => Ok(true),
            Err(e) if e.major_status().status_type() == StatusType::InvariantViolation => Err(e),
            Err(_) => Ok(false),
        }
    }

    fn type_to_type_layout(&self, ty: &Type) -> PartialVMResult<Option<MoveTypeLayout>> {
        match self.resolver.type_to_type_layout(ty) {
            Ok(ty_layout) => Ok(Some(ty_layout)),
            Err(e) if e.major_status().status_type() == StatusType::InvariantViolation => Err(e),
            Err(_) => Ok(None),
        }
    }

    fn is_resource(&self, ty: &Type) -> bool {
        self.resolver.is_resource(ty)
    }

    fn caller(&self) -> Option<&ModuleId> {
        self.caller
    }
}
