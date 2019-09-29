use crate::backend::{Backend, BV};
use crate::error::*;
use crate::return_value::*;
use crate::state::State;
use llvm_ir::*;
use log::warn;

/// Assume that allocations never exceed this size.
const MAX_ALLOCATION_SIZE_BYTES: u64 = 1 << 20;

pub(crate) fn malloc_hook<'p, B: Backend + 'p>(state: &mut State<'p, B>, call: &'p instruction::Call) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.arguments.len(), 1);
    let bytes = &call.arguments[0].0;
    match bytes.get_type() {
        Type::IntegerType { .. } => {},
        ty => return Err(Error::OtherError(format!("malloc_hook: expected argument to have integer type, but got {:?}", ty))),
    };
    match call.get_type() {
        Type::PointerType { .. } => {},
        ty => return Err(Error::OtherError(format!("malloc_hook: expected return type to be a pointer type, but got {:?}", ty))),
    };

    // Note that allocating too much doesn't hurt anything, as long as we don't
    // run out of address space in our symbolic memory.
    let bytes = try_as_u64(bytes).unwrap_or(MAX_ALLOCATION_SIZE_BYTES);
    if bytes > MAX_ALLOCATION_SIZE_BYTES {
        warn!("warning: encountered an allocation of {} bytes, greater than the assumed max of {}. \
            Since this allocation is constant-sized, it's fine in this case, but does draw into question the assumption.", bytes, MAX_ALLOCATION_SIZE_BYTES);
    }
    let bits = bytes * 8;
    let addr = state.allocate(bits);
    Ok(ReturnValue::Return(addr))
}

pub(crate) fn calloc_hook<'p, B: Backend + 'p>(state: &mut State<'p, B>, call: &'p instruction::Call) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.arguments.len(), 2);
    let num = &call.arguments[0].0;
    let size = &call.arguments[1].0;
    match num.get_type() {
        Type::IntegerType { .. } => {},
        ty => return Err(Error::OtherError(format!("calloc_hook: expected first argument to have integer type, but got {:?}", ty))),
    };
    match size.get_type() {
        Type::IntegerType { .. } => {},
        ty => return Err(Error::OtherError(format!("calloc_hook: expected second argument to have integer type, but got {:?}", ty))),
    };
    match call.get_type() {
        Type::PointerType { .. } => {},
        ty => return Err(Error::OtherError(format!("calloc_hook: expected return type to be a pointer type, but got {:?}", ty))),
    };

    // As in `malloc_hook()`, note that allocating too much doesn't hurt anything
    let bytes = match (try_as_u64(num), try_as_u64(size)) {
        (Some(num), Some(size)) => num * size,
        _ => MAX_ALLOCATION_SIZE_BYTES,
    };
    if bytes > MAX_ALLOCATION_SIZE_BYTES {
        warn!("warning: encountered an allocation of {} bytes, greater than the assumed max of {}. \
            Since this allocation is constant-sized, it's fine in this case, but does draw into question the assumption.", bytes, MAX_ALLOCATION_SIZE_BYTES);
    }
    let bits = bytes * 8;
    let addr = state.allocate(bits);
    state.write(&addr, B::BV::zero(state.solver.clone(), bits as u32))?;  // calloc() requires zeroed memory
    Ok(ReturnValue::Return(addr))
}

/// Try to interpret the `Operand` as a constant integer, and if so, return the value as a `u64`.
/// (But don't try too hard - as of this writing, doesn't even try to evaluate constant expressions.)
fn try_as_u64(op: &Operand) -> Option<u64> {
    match op {
        Operand::ConstantOperand(Constant::Int { value, .. }) => Some(*value),
        _ => None,
    }
}

pub(crate) fn free_hook<'p, B: Backend + 'p>(_state: &mut State<'p, B>, _call: &'p instruction::Call) -> Result<ReturnValue<B::BV>> {
    // The simplest implementation of free() is a no-op.
    // Our malloc_hook() above won't ever reuse allocated addresses anyway.
    Ok(ReturnValue::ReturnVoid)
}

pub(crate) fn realloc_hook<'p, B: Backend + 'p>(state: &mut State<'p, B>, call: &'p instruction::Call) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.arguments.len(), 2);
    let addr = &call.arguments[0].0;
    let new_size = &call.arguments[1].0;
    match addr.get_type() {
        Type::PointerType { .. } => {},
        ty => return Err(Error::OtherError(format!("realloc_hook: expected first argument to be a pointer type, but got {:?}", ty))),
    };
    match new_size.get_type() {
        Type::IntegerType { .. } => {},
        ty => return Err(Error::OtherError(format!("realloc_hook: expected second argument to be an integer type, but got {:?}", ty))),
    };
    match call.get_type() {
        Type::PointerType { .. } => {},
        ty => return Err(Error::OtherError(format!("realloc_hook: expected return type to be a pointer type, but got {:?}", ty))),
    };

    let addr = state.operand_to_bv(addr)?;
    // As in `malloc_hook()`, note that allocating too much doesn't hurt anything
    let new_size = try_as_u64(new_size).unwrap_or(MAX_ALLOCATION_SIZE_BYTES);
    let old_size = state.get_allocation_size(&addr)?.ok_or_else(|| Error::OtherError("realloc_hook: failed to get old allocation size".to_owned()))?;
    if new_size <= old_size {
        // We treat this as a no-op. You get to keep the larger old_size region you already had.
        Ok(ReturnValue::Return(addr))
    } else {
        // Make a new allocation
        let new_addr = state.allocate(new_size);
        // Copy the contents of the old allocation
        let contents = state.read(&addr, old_size as u32)?;
        state.write(&new_addr, contents)?;
        // We don't free() (see comments in `free_hook`), so just return
        Ok(ReturnValue::Return(new_addr))
    }
}
