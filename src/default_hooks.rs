use crate::backend::Backend;
use crate::config::*;
use crate::state::State;
use llvm_ir::*;
use std::collections::HashMap;

pub fn default_hooks<'ctx, B>() -> HashMap<String, FunctionHook<'ctx, B>> where B: Backend<'ctx> {
    std::iter::once(("malloc".to_owned(), FunctionHook::new(&malloc_hook::<B>)))
        .chain(std::iter::once(("free".to_owned(), FunctionHook::new(&free_hook::<B>))))
        .collect()
}

fn malloc_hook<'ctx, B>(state: &mut State<'ctx, '_, B>, call: &instruction::Call) -> HookResult<B::BV> where B: Backend<'ctx> {
    assert_eq!(call.arguments.len(), 1);
    match call.arguments[0].0.get_type() {
        Type::IntegerType { .. } => {},
        ty => panic!("malloc_hook: expected argument to have integer type, but got {:?}", ty),
    };
    match call.get_type() {
        Type::PointerType { .. } => {},
        ty => panic!("malloc_hook: expected return type to be a pointer type, but got {:?}", ty),
    };
    let allocation_size = if let Operand::ConstantOperand(Constant::Int { value, .. }) = call.arguments[0].0 {
        value
    } else {
        // Assume that allocations never exceed 1 MiB in size. Note that allocating too
        // much doesn't hurt anything, as long as we don't run out of address space in
        // our symbolic memory.
        1 << 20
    };
    let addr = state.allocate(allocation_size);
    Ok(ReturnValue::Return(addr))
}

fn free_hook<'ctx, B>(_state: &mut State<'ctx, '_, B>, _call: &instruction::Call) -> HookResult<B::BV> where B: Backend<'ctx> {
    // The simplest implementation of free() is a no-op.
    // Our malloc_hook() above won't ever reuse allocated addresses anyway.
    Ok(ReturnValue::ReturnVoid)
}
