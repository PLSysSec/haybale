//! Utility functions for performing memset or memcpy operations.
//! These may be useful in implementing hooks for other functions.

use crate::backend::{Backend, BV};
use crate::config::Concretize;
use crate::error::*;
use crate::solver_utils::PossibleSolutions;
use crate::state::State;
use llvm_ir::Operand;
use log::{debug, info, warn};
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

    match get_memcpy_length(state, &num_bytes, &state.config.concretize_memcpy_lengths)? {
        MemcpyLength::Concrete(0) => debug!("Ignoring a memset of size 0 bytes"),
        MemcpyLength::Concrete(length_bytes) => {
            debug!("Processing a memset of size {} bytes", length_bytes);
            // Do the operation as just one large write; let the memory choose the most efficient way to implement that.
            assert_eq!(val.get_width(), 8);
            let big_val = if state.bvs_must_be_equal(&val, &state.zero(8))? {
                // optimize this special case
                state.zero(8 * u32::try_from(length_bytes).map_err(|e| Error::OtherError(format!("memset too big: {} bytes (error: {})", length_bytes, e)))?)
            } else if state.bvs_must_be_equal(&val, &state.ones(8))? {
                // optimize this special case
                state.ones(8 * u32::try_from(length_bytes).map_err(|e| Error::OtherError(format!("memset too big: {} bytes (error: {})", length_bytes, e)))?)
            } else {
                std::iter::repeat(val).take(length_bytes as usize).reduce(|a,b| a.concat(&b)).unwrap()
            };
            state.write(&addr, big_val)?;
        },
        MemcpyLength::Symbolic => {
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
        },
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

    match get_memcpy_length(state, &num_bytes, &state.config.concretize_memcpy_lengths)? {
        MemcpyLength::Concrete(0) => debug!("Ignoring a memcpy or memmove of size 0 bytes"),
        MemcpyLength::Concrete(length_bytes) => {
            debug!("Processing a memcpy or memmove of size {} bytes", length_bytes);
            // Do the operation as just one large read and one large write; let the memory choose the most efficient way to implement these.
            let val = state.read(&src, length_bytes as u32 * 8)?;
            state.write(&dest, val)?;
        },
        MemcpyLength::Symbolic => {
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
        },
    }

    Ok(dest)
}

enum MemcpyLength {
    /// Use this concrete value as the memcpy length, in bytes
    Concrete(u64),
    /// The memcpy length has symbolic, i.e. non-constant, value
    Symbolic,
}

/// For a `memcpy`, `memset`, or `memmove` operation with the given `num_bytes`
/// parameter, return a `MemcpyLength` describing the length of the operation
/// that should be performed, considering the given `Concretize` option.
///
/// Also accounts for the `max_memcpy_length` option in `state.config`.
fn get_memcpy_length<B: Backend>(state: &State<B>, num_bytes: &B::BV, concretize: &Concretize) -> Result<MemcpyLength> {
    match state.get_possible_solutions_for_bv(num_bytes, 1)? {
        PossibleSolutions::Exactly(v) => {
            let single_val = v.iter().next().ok_or(Error::Unsat)?.as_u64().unwrap();
            match state.config.max_memcpy_length {
                Some(max_memcpy_length) if single_val > max_memcpy_length => {
                    Err(Error::OtherError(format!("Encountered a memcpy/memset/memmove of length exactly {} bytes, larger than max_memcpy_length {} bytes", single_val, max_memcpy_length)))
                },
                _ => Ok(MemcpyLength::Concrete(single_val)),
            }
        },
        PossibleSolutions::AtLeast(v) => {
            if let Some(max_memcpy_length) = state.config.max_memcpy_length {
                let max_memcpy_length_bv = state.bv_from_u64(max_memcpy_length, num_bytes.get_width());
                if !state.sat_with_extra_constraints(std::iter::once(&num_bytes.ulte(&max_memcpy_length_bv)))? {
                    let arbitrary_val = v.iter().next().unwrap().as_u64().unwrap();
                    return Err(Error::OtherError(format!("Encountered a memcpy/memset/memmove with multiple possible lengths, but all of them are larger than max_memcpy_length {} bytes. One possible length is {} bytes.", max_memcpy_length, arbitrary_val)));
                }
                if state.sat_with_extra_constraints(std::iter::once(&num_bytes.ugt(&max_memcpy_length_bv)))? {
                    warn!("Encountered a memcpy/memset/memmove with multiple possible lengths, some of which are larger than max_memcpy_length {} bytes. Constraining the length to be at most {} bytes.", max_memcpy_length, max_memcpy_length);
                    num_bytes.ulte(&max_memcpy_length_bv).assert()?;
                }
            }
            let num_bytes_concrete = match concretize {
                Concretize::Arbitrary => {
                    match state.config.max_memcpy_length {
                        None => v.iter().next().unwrap().as_u64().unwrap(),
                        Some(max_memcpy_length) => {
                            match v.iter().map(|val| val.as_u64().unwrap()).find(|val| *val <= max_memcpy_length) {
                                Some(val) => val,
                                None => state.get_a_solution_for_bv(num_bytes)?.unwrap().as_u64().unwrap()
                            }
                        },
                    }
                },
                Concretize::Minimum => state.min_possible_solution_for_bv_as_u64(num_bytes)?.unwrap(),
                Concretize::Maximum => state.max_possible_solution_for_bv_as_u64(num_bytes)?.unwrap(),
                Concretize::Prefer(val, backup) => {
                    let val_as_bv = state.bv_from_u64(*val, num_bytes.get_width());
                    if state.bvs_can_be_equal(&num_bytes, &val_as_bv)? {
                        *val
                    } else if !state.sat()? {
                        return Err(Error::Unsat);
                    } else {
                        return get_memcpy_length(state, num_bytes, &**backup);
                    }
                },
                Concretize::Symbolic => return Ok(MemcpyLength::Symbolic),
            };
            info!("Encountered a memcpy/memset/memmove with multiple possible lengths; according to the concretization policy {:?}, chose a length of {} bytes and will constrain the length argument to be {} going forward", concretize, num_bytes_concrete, num_bytes_concrete);
            // actually constrain that `num_bytes` has to now be equal to our chosen concrete value
            num_bytes._eq(&state.bv_from_u64(num_bytes_concrete, num_bytes.get_width())).assert()?;
            Ok(MemcpyLength::Concrete(num_bytes_concrete))
        }
    }
}
