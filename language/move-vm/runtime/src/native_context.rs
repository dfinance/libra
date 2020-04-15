use crate::interpreter::Interpreter;
use crate::loaded_data::loaded_module::LoadedModule;
use crate::runtime::VMRuntime;
use libra_types::access_path::AccessPath;
use libra_types::account_address::AccountAddress;
use libra_types::language_storage::ModuleId;
use libra_types::vm_error::{StatusCode, VMStatus};
use move_core_types::identifier::IdentStr;
use move_vm_types::identifier::create_access_path;
use move_vm_types::interpreter_context::InterpreterContext;
use move_vm_types::loaded_data::types::{StructType, Type};
use move_vm_types::native_functions::context::{Module, NativeContext};
use move_vm_types::native_functions::dispatch::FunctionResolver;
use std::fmt::Write;
use vm::access::ModuleAccess;
use vm::errors::VMResult;
use vm::gas_schedule::CostTable;

pub struct FunctionContext<'a, 'alloc, 'txn, R: FunctionResolver> {
    caller_module: &'a LoadedModule,
    self_module: &'a LoadedModule,
    interpreter: &'a mut Interpreter<'txn, R>,
    runtime: &'a VMRuntime<'alloc>,
    context: &'a mut dyn InterpreterContext,
}

impl<'a, 'alloc, 'txn, R> FunctionContext<'a, 'alloc, 'txn, R>
where
    R: FunctionResolver,
{
    pub fn new(
        caller_module: &'a LoadedModule,
        self_module: &'a LoadedModule,
        interpreter: &'a mut Interpreter<'txn, R>,
        runtime: &'a VMRuntime<'alloc>,
        context: &'a mut dyn InterpreterContext,
    ) -> FunctionContext<'a, 'alloc, 'txn, R> {
        FunctionContext {
            caller_module,
            self_module,
            interpreter,
            runtime,
            context,
        }
    }
}

impl<'a, 'alloc, 'txn, R> NativeContext for FunctionContext<'a, 'alloc, 'txn, R>
where
    R: FunctionResolver,
{
    type Mod = ModuleContext<'a, 'alloc>;

    fn caller_module(&self) -> ModuleContext<'a, 'alloc> {
        ModuleContext {
            module: self.caller_module,
            runtime: self.runtime,
        }
    }

    fn self_module(&self) -> ModuleContext<'a, 'alloc> {
        ModuleContext {
            module: self.self_module,
            runtime: self.runtime,
        }
    }

    fn create_access_path(
        &self,
        account_address: AccountAddress,
        struct_ty: &StructType,
    ) -> VMResult<AccessPath> {
        Ok(create_access_path(
            account_address,
            struct_ty.clone().into_struct_tag()?,
        ))
    }

    fn interpreter(&self) -> &dyn InterpreterContext {
        self.context
    }

    fn interpreter_mut(&mut self) -> &mut dyn InterpreterContext {
        self.context
    }

    fn debug_print_stack_trace<B: Write>(&self, buf: &mut B) -> VMResult<()> {
        self.interpreter
            .debug_print_stack_trace(buf, self.runtime, self.context)
    }

    fn cost_table(&self) -> &CostTable {
        self.interpreter.gas_schedule()
    }
}

pub struct ModuleContext<'a, 'alloc> {
    module: &'a LoadedModule,
    runtime: &'a VMRuntime<'alloc>,
}

impl<'a, 'alloc> Module for ModuleContext<'a, 'alloc> {
    fn self_id(&self) -> ModuleId {
        self.module.self_id()
    }

    fn resolve_struct_def(
        &self,
        module: &impl NativeContext,
        struct_name: &IdentStr,
        ty_args: &[Type],
    ) -> VMResult<StructType> {
        let struct_id = self
            .module
            .struct_defs_table
            .get(struct_name)
            .ok_or_else(|| VMStatus::new(StatusCode::LINKER_ERROR))?;
        self.runtime
            .resolve_struct_def(self.module, *struct_id, ty_args, module.interpreter())
    }
}
