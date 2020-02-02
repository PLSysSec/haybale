//! Utility functions for performing memset or memcpy operations.
//! These may be useful in implementing hooks for other functions.

use crate::backend::{Backend, BV};
use crate::config::Concretize;
use crate::error::*;
use crate::solver_utils::PossibleSolutions;
use crate::state::State;
use llvm_ir::Operand;
use log::{debug, warn};
use reduce::Reduce;
use std::convert::TryFrom;

/// Set `num_bytes` bytes of memory at address `addr` each to the value `val`.
/// Each individual byte will be set to `val`, so only the lowest 8 bits of `val`
/// will be used.
///
/// Returns `addr` as a `BV`. Most callers probably won't need this.
///
/// Respects the `state.config.concretize_memcpy_lengths` setting.
pub fn memset<B: Backend>(state: &mut State<B>, addr: &Operand, val: &Operand, num_bytes: &Operand) -> Result<B::BV> {
    let addr = state.operand_to_bv(addr)?;
    let val = {
        let mut val = state.operand_to_bv(val)?;
        if val.get_width() > 8 {
            // some memset declarations have a larger type here, but it's still intended to be a byte value; we ignore any upper bits
            val = val.slice(7, 0);
        }
        val
    };

    let num_bytes = state.operand_to_bv(num_bytes)?;

    // if num_bytes is `Some`, we perform the operation with that num_bytes;
    // else (if num_bytes is `None`) we assume that everything has already been
    // handled and we're done
    let num_bytes: Option<_> = match state.get_possible_solutions_for_bv(&num_bytes, 1)? {
        PossibleSolutions::Exactly(v) => Some(v.iter().next().ok_or(Error::Unsat)?.as_u64().unwrap()),
        PossibleSolutions::AtLeast(v) => {
            let num_bytes_concrete: Option<_> = match state.config.concretize_memcpy_lengths {
                Concretize::Arbitrary => Some(v.iter().next().unwrap().as_u64().unwrap()),
                Concretize::Minimum => Some(state.min_possible_solution_for_bv_as_u64(&num_bytes)?.unwrap()),
                Concretize::Maximum => Some(state.max_possible_solution_for_bv_as_u64(&num_bytes)?.unwrap()),
                Concretize::Prefer(val, _) => {
                    let val_as_bv = state.bv_from_u64(val, num_bytes.get_width());
                    if state.bvs_can_be_equal(&num_bytes, &val_as_bv)? {
                        Some(val)
                    } else if !state.sat()? {
                        return Err(Error::Unsat);
                    } else {
                        return Err(Error::UnsupportedInstruction("not implemented yet: memset with non-constant size in bytes, Concretize::Prefer, and needing to execute the fallback path".to_owned()));
                    }
                },
                Concretize::Symbolic => {
                    // In this case we just do the entire write here
                    let max_num_bytes = state.max_possible_solution_for_bv_as_u64(&num_bytes)?.unwrap();
                    if max_num_bytes > 0x4000 {
                        warn!("Encountered a memset with symbolic size, up to {} bytes. This may be slow.", max_num_bytes);
                    } else {
                        debug!("Processing a memset of symbolic size, up to {} bytes", max_num_bytes);
                    }
                    let mut addr = addr.clone();
                    let mut bytes_written = state.zero(num_bytes.get_width());
                    for _ in 0 ..= max_num_bytes {
                        let old_val = state.read(&addr, 8)?;
                        let should_write = num_bytes.ugt(&bytes_written);
                        state.write(&addr, should_write.cond_bv(&val, &old_val))?;
                        addr = addr.inc();
                        bytes_written = bytes_written.inc();
                    }
                    None
                }
            };
            if let Some(num_bytes_concrete) = num_bytes_concrete {
                num_bytes._eq(&state.bv_from_u64(num_bytes_concrete, num_bytes.get_width())).assert()?;
            }
            num_bytes_concrete
        }
    };

    if let Some(num_bytes) = num_bytes {
        // we picked a single concrete value for num_bytes: perform the operation with that value
        if num_bytes == 0 {
            debug!("Ignoring a memset of size 0 bytes");
        } else {
            debug!("Processing a memset of size {} bytes", num_bytes);
            // Do the operation as just one large write; let the memory choose the most efficient way to implement that.
            assert_eq!(val.get_width(), 8);
            let big_val = if state.bvs_must_be_equal(&val, &state.zero(8))? {
                // optimize this special case
                state.zero(8 * u32::try_from(num_bytes).map_err(|e| Error::OtherError(format!("memset too big: {} bytes (error: {})", num_bytes, e)))?)
            } else if state.bvs_must_be_equal(&val, &state.ones(8))? {
                // optimize this special case
                state.ones(8 * u32::try_from(num_bytes).map_err(|e| Error::OtherError(format!("memset too big: {} bytes (error: {})", num_bytes, e)))?)
            } else {
                std::iter::repeat(val).take(num_bytes as usize).reduce(|a,b| a.concat(&b)).unwrap()
            };
            state.write(&addr, big_val)?;
        }
    }

    Ok(addr)
}

/// Copies `num_bytes` bytes of memory from address `src` to address `dest`.
/// `src` and `dest` may overlap.
///
/// Returns `dest` as a `BV`. Most callers probably won't need this.
///
/// Respects the `state.config.concretize_memcpy_lengths` setting.
pub fn memcpy<B: Backend>(state: &mut State<B>, dest: &Operand, src: &Operand, num_bytes: &Operand) -> Result<B::BV> {
    let dest = state.operand_to_bv(&dest)?;
    let src = state.operand_to_bv(&src)?;

    let num_bytes = state.operand_to_bv(num_bytes)?;

    // if num_bytes is `Some`, we perform the operation with that num_bytes;
    // else (if num_bytes is `None`) we assume that everything has already been
    // handled and we're done
    let num_bytes: Option<_> = match state.get_possible_solutions_for_bv(&num_bytes, 1)? {
        PossibleSolutions::Exactly(v) => Some(v.iter().next().ok_or(Error::Unsat)?.as_u64().unwrap()),
        PossibleSolutions::AtLeast(v) => {
            let num_bytes_concrete: Option<_> = match state.config.concretize_memcpy_lengths {
                Concretize::Arbitrary => Some(v.iter().next().unwrap().as_u64().unwrap()),
                Concretize::Minimum => Some(state.min_possible_solution_for_bv_as_u64(&num_bytes)?.unwrap()),
                Concretize::Maximum => Some(state.max_possible_solution_for_bv_as_u64(&num_bytes)?.unwrap()),
                Concretize::Prefer(val, _) => {
                    let val_as_bv = state.bv_from_u64(val, num_bytes.get_width());
                    if state.bvs_can_be_equal(&num_bytes, &val_as_bv)? {
                        Some(val)
                    } else if !state.sat()? {
                        return Err(Error::Unsat);
                    } else {
                        return Err(Error::UnsupportedInstruction("not implemented yet: memcpy or memmove with non-constant size in bytes, Concretize::Prefer, and needing to execute the fallback path".to_owned()));
                    }
                },
                Concretize::Symbolic => {
                    // In this case we just do the entire write here
                    let max_num_bytes = state.max_possible_solution_for_bv_as_u64(&num_bytes)?.unwrap();
                    if max_num_bytes > 0x4000 {
                        warn!("Encountered a memcpy or memmove with symbolic size, up to {} bytes. This may be slow.", max_num_bytes);
                    } else {
                        debug!("Processing a memcpy or memmove of symbolic size, up to {} bytes", max_num_bytes);
                    }
                    let mut src_addr = src.clone();
                    let mut dest_addr = dest.clone();
                    let mut bytes_written = state.zero(num_bytes.get_width());
                    for _ in 0 ..= max_num_bytes {
                        let src_val = state.read(&src_addr, 8)?;
                        let dst_val = state.read(&dest_addr, 8)?;
                        let should_write = num_bytes.ugt(&bytes_written);
                        state.write(&dest_addr, should_write.cond_bv(&src_val, &dst_val))?;
                        src_addr = src_addr.inc();
                        dest_addr = dest_addr.inc();
                        bytes_written = bytes_written.inc();
                    }
                    None
                }
            };
            if let Some(num_bytes_concrete) = num_bytes_concrete {
                num_bytes._eq(&state.bv_from_u64(num_bytes_concrete, num_bytes.get_width())).assert()?;
            }
            num_bytes_concrete
        },
    };

    if let Some(num_bytes) = num_bytes {
        // we picked a single concrete value for num_bytes: perform the operation with that value
        if num_bytes == 0 {
            debug!("Ignoring a memcpy or memmove of size 0 bytes");
        } else {
            debug!("Processing a memcpy or memmove of size {} bytes", num_bytes);
            // Do the operation as just one large read and one large write; let the memory choose the most efficient way to implement these.
            let val = state.read(&src, num_bytes as u32 * 8)?;
            state.write(&dest, val)?;
        }
    }

    Ok(dest)
}
