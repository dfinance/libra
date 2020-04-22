// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{interpreter::Interpreter, loader::Resolver};
use libra_types::{
    access_path::AccessPath, account_address::AccountAddress, language_storage::ModuleId,
};
use move_core_types::{gas_schedule::CostTable, identifier::IdentStr};
use move_vm_types::{
    interpreter_context::InterpreterContext,
    loaded_data::{runtime_types::Type, types::FatType},
    native_functions::context::NativeContext,
    values::Struct,
};
use std::fmt::Write;
use vm::errors::VMResult;

pub struct FunctionContext<'a, 'txn> {
    caller_module: Option<&'a ModuleId>,
    self_module: &'a ModuleId,
    interpreter: &'a mut Interpreter<'txn>,
    interpreter_context: &'a mut dyn InterpreterContext,
    resolver: &'a Resolver<'a>,
}

impl<'a, 'txn> FunctionContext<'a, 'txn> {
    pub fn new(
        caller_module: Option<&'a ModuleId>,
        self_module: &'a ModuleId,
        interpreter: &'a mut Interpreter<'txn>,
        context: &'a mut dyn InterpreterContext,
        resolver: &'a Resolver<'a>,
    ) -> FunctionContext<'a, 'txn> {
        FunctionContext {
            caller_module,
            self_module,
            interpreter,
            interpreter_context: context,
            resolver,
        }
    }
}

impl<'a, 'txn> NativeContext for FunctionContext<'a, 'txn> {
    fn caller_module(&self) -> Option<&ModuleId> {
        self.caller_module
    }

    fn self_module(&self) -> &ModuleId {
        self.self_module
    }

    fn interpreter(&self) -> &dyn InterpreterContext {
        self.interpreter_context
    }

    fn interpreter_mut(&mut self) -> &mut dyn InterpreterContext {
        self.interpreter_context
    }

    fn print_stack_trace<B: Write>(&self, buf: &mut B) -> VMResult<()> {
        self.interpreter
            .debug_print_stack_trace(buf, &self.resolver)
    }

    fn cost_table(&self) -> &CostTable {
        self.interpreter.gas_schedule()
    }

    fn save_under_address(
        &mut self,
        ty_args: &[Type],
        module_id: &ModuleId,
        struct_name: &IdentStr,
        resource_to_save: Struct,
        account_address: AccountAddress,
    ) -> VMResult<()> {
        let libra_type = self.resolver.get_libra_type_info(
            module_id,
            struct_name,
            ty_args,
            self.interpreter_context,
        )?;
        let ap = AccessPath::new(account_address, libra_type.resource_key().to_vec());
        self.interpreter_context
            .move_resource_to(&ap, libra_type.fat_type(), resource_to_save)
    }

    fn convert_to_fat_types(&self, types: Vec<Type>) -> VMResult<Vec<FatType>> {
        types
            .iter()
            .map(|ty| self.resolver.type_to_fat_type(ty))
            .collect()
    }
}
