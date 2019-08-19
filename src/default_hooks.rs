use crate::backend::Backend;
use crate::config::*;
use crate::error::*;
use crate::state::State;
use llvm_ir::*;

pub(crate) fn malloc_hook<'ctx, B: Backend<'ctx>>(state: &mut State<'ctx, '_, B>, call: &instruction::Call) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.arguments.len(), 1);
    match call.arguments[0].0.get_type() {
        Type::IntegerType { .. } => {},
        ty => return Err(Error::OtherError(format!("malloc_hook: expected argument to have integer type, but got {:?}", ty))),
    };
    match call.get_type() {
        Type::PointerType { .. } => {},
        ty => return Err(Error::OtherError(format!("malloc_hook: expected return type to be a pointer type, but got {:?}", ty))),
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

pub(crate) fn free_hook<'ctx, B: Backend<'ctx>>(_state: &mut State<'ctx, '_, B>, _call: &instruction::Call) -> Result<ReturnValue<B::BV>> {
    // The simplest implementation of free() is a no-op.
    // Our malloc_hook() above won't ever reuse allocated addresses anyway.
    Ok(ReturnValue::ReturnVoid)
}
