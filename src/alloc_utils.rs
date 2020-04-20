//! Utility functions for performing memory allocation.
//! These may be useful in implementing hooks for various functions that
//! perform memory allocation.

use crate::backend::Backend;
use crate::error::*;
use crate::state::State;
use llvm_ir::*;
use log::warn;

/// Assume that allocations never exceed this size.
const MAX_ALLOCATION_SIZE_BYTES: u64 = 1 << 20;

/// Allocate a number of bytes given by the `Operand`.
///
/// Returns the address of the newly-allocated memory.
pub fn malloc<B: Backend>(state: &mut State<B>, num_bytes: &Operand) -> Result<B::BV> {
    // Note that allocating too much doesn't hurt anything, as long as we don't
    // run out of address space in our symbolic memory.
    let num_bytes = try_as_u64(num_bytes).unwrap_or(MAX_ALLOCATION_SIZE_BYTES);
    if num_bytes > MAX_ALLOCATION_SIZE_BYTES {
        warn!("warning: encountered an allocation of {} bytes, greater than the assumed max of {}. \
            Since this allocation is constant-sized, it's fine in this case, but does draw into question the assumption.", num_bytes, MAX_ALLOCATION_SIZE_BYTES);
    }
    let num_bits = num_bytes * 8;
    Ok(state.allocate(num_bits))
}

/// Allocate a number of bytes given by the `Operand`.
/// The newly-allocated memory will be initialized to all zeroes.
///
/// Returns the address of the newly-allocated memory.
pub fn zalloc<B: Backend>(state: &mut State<B>, num_bytes: &Operand) -> Result<B::BV> {
    // As in `malloc()`, note that allocating too much doesn't hurt anything
    let num_bytes = try_as_u64(num_bytes).unwrap_or(MAX_ALLOCATION_SIZE_BYTES);
    if num_bytes > MAX_ALLOCATION_SIZE_BYTES {
        warn!("warning: encountered an allocation of {} bytes, greater than the assumed max of {}. \
            Since this allocation is constant-sized, it's fine in this case, but does draw into question the assumption.", num_bytes, MAX_ALLOCATION_SIZE_BYTES);
    }
    let num_bits = num_bytes * 8;
    let addr = state.allocate(num_bits);
    state.write(&addr, state.zero(num_bits as u32))?;
    Ok(addr)
}

/// Allocate a number of bytes given by `a` times `b`, where `a` and `b` are
/// `Operand`s. The newly-allocated memory will be initialized to all zeroes.
///
/// Returns the address of the newly-allocated memory.
pub fn calloc<B: Backend>(state: &mut State<B>, a: &Operand, b: &Operand) -> Result<B::BV> {
    // As in `malloc()`, note that allocating too much doesn't hurt anything
    let num_bytes = match (try_as_u64(a), try_as_u64(b)) {
        (Some(a), Some(b)) => a * b,
        _ => MAX_ALLOCATION_SIZE_BYTES,
    };
    if num_bytes > MAX_ALLOCATION_SIZE_BYTES {
        warn!("warning: encountered an allocation of {} bytes, greater than the assumed max of {}. \
            Since this allocation is constant-sized, it's fine in this case, but does draw into question the assumption.", num_bytes, MAX_ALLOCATION_SIZE_BYTES);
    }
    let num_bits = num_bytes * 8;
    let addr = state.allocate(num_bits);
    state.write(&addr, state.zero(num_bits as u32))?;
    Ok(addr)
}

/// Reallocate the given `addr` to be at least the number of bytes given by the `Operand`.
///
/// Returns the address of the allocation, which may or may not be the same
/// address which was passed in.
pub fn realloc<B: Backend>(
    state: &mut State<B>,
    addr: &Operand,
    num_bytes: &Operand,
) -> Result<B::BV> {
    let addr = state.operand_to_bv(addr)?;
    // As in `malloc()`, note that allocating too much doesn't hurt anything
    let new_size = try_as_u64(num_bytes).unwrap_or(MAX_ALLOCATION_SIZE_BYTES);
    if new_size > MAX_ALLOCATION_SIZE_BYTES {
        warn!("warning: encountered an allocation of {} bytes, greater than the assumed max of {}. \
            Since this allocation is constant-sized, it's fine in this case, but does draw into question the assumption.", new_size, MAX_ALLOCATION_SIZE_BYTES);
    }
    let old_size = state.get_allocation_size(&addr)?.ok_or_else(|| {
        Error::OtherError("realloc: failed to get old allocation size".to_owned())
    })?;
    if new_size <= old_size {
        // We treat this as a no-op. You get to keep the larger old_size region you already had.
        Ok(addr)
    } else {
        // Make a new allocation
        let new_addr = state.allocate(new_size);
        // Copy the contents of the old allocation
        let contents = state.read(&addr, old_size as u32)?;
        state.write(&new_addr, contents)?;
        // We don't free(), as our allocator won't ever reuse allocated addresses anyway.
        // So, we can just return
        Ok(new_addr)
    }
}

/// Try to interpret the `Operand` as a constant integer, and if so, return the value as a `u64`.
/// (But don't try too hard - as of this writing, doesn't even try to evaluate constant expressions.)
fn try_as_u64(op: &Operand) -> Option<u64> {
    match op {
        Operand::ConstantOperand(Constant::Int { value, .. }) => Some(*value),
        _ => None,
    }
}
