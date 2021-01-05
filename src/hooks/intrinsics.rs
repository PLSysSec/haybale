//! Default hooks for some LLVM intrinsics

use crate::backend::{Backend, BV};
use crate::error::*;
use crate::function_hooks::IsCall;
use crate::hook_utils;
use crate::project::Project;
use crate::return_value::ReturnValue;
use crate::state::State;
use crate::symex::unary_on_vector;
use llvm_ir::Type;
use std::convert::TryInto;

pub fn symex_memset<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &'p dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 4);
    let addr = &call.get_arguments()[0].0;
    let val = &call.get_arguments()[1].0;
    let num_bytes = &call.get_arguments()[2].0;
    match state.type_of(addr).as_ref() {
        Type::PointerType { pointee_type, .. } => match pointee_type.as_ref() {
            Type::IntegerType { bits: 8 } => (),
            _ => {
                return Err(Error::OtherError(format!(
                    "memset: Expected address to be a pointer to i8, got pointer to {:?}",
                    pointee_type
                )))
            },
        },
        ty => {
            return Err(Error::OtherError(format!(
                "memset: Expected address to have pointer type, got {:?}",
                ty
            )))
        },
    }

    let addr = hook_utils::memset(state, addr, val, num_bytes)?;

    // if the call should return a pointer, it returns `addr`. If it's void-typed, that's fine too.
    match state.type_of(call).as_ref() {
        Type::VoidType => Ok(ReturnValue::ReturnVoid),
        Type::PointerType { .. } => Ok(ReturnValue::Return(addr)),
        ty => Err(Error::OtherError(format!(
            "Unexpected return type for a memset: {:?}",
            ty
        ))),
    }
}

pub fn symex_memcpy<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &'p dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    let dest = &call.get_arguments()[0].0;
    let src = &call.get_arguments()[1].0;
    let num_bytes = &call.get_arguments()[2].0;
    match state.type_of(dest).as_ref() {
        Type::PointerType { pointee_type, .. } => match pointee_type.as_ref() {
            Type::IntegerType { bits: 8 } => (),
            _ => {
                return Err(Error::OtherError(format!(
                    "memcpy: Expected dest to be a pointer to i8, got pointer to {:?}",
                    pointee_type
                )))
            },
        },
        ty => {
            return Err(Error::OtherError(format!(
                "memcpy: Expected dest to have pointer type, got {:?}",
                ty
            )))
        },
    }
    match state.type_of(src).as_ref() {
        Type::PointerType { pointee_type, .. } => match pointee_type.as_ref() {
            Type::IntegerType { bits: 8 } => (),
            _ => {
                return Err(Error::OtherError(format!(
                    "memcpy: Expected dest to be a pointer to i8, got pointer to {:?}",
                    pointee_type
                )))
            },
        },
        ty => {
            return Err(Error::OtherError(format!(
                "memcpy: Expected dest to have pointer type, got {:?}",
                ty
            )))
        },
    }

    let dest = hook_utils::memcpy(state, dest, src, num_bytes)?;

    // if the call should return a pointer, it returns `dest`. If it's void-typed, that's fine too.
    match state.type_of(call).as_ref() {
        Type::VoidType => Ok(ReturnValue::ReturnVoid),
        Type::PointerType { .. } => Ok(ReturnValue::Return(dest)),
        ty => Err(Error::OtherError(format!(
            "Unexpected return type for a memcpy or memmove: {:?}",
            ty
        ))),
    }
}

pub fn symex_bswap<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &'p dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 1);
    let arg = &call.get_arguments()[0].0;
    let argty = state.type_of(arg);
    let retty = state.type_of(call);
    if argty != retty {
        return Err(Error::OtherError(
            "Expected bswap argument to be the same type as its return type".to_owned(),
        ));
    }

    let arg = state.operand_to_bv(arg)?;
    match argty.as_ref() {
        Type::IntegerType { bits } => {
            assert_eq!(arg.get_width(), *bits);
            Ok(ReturnValue::Return(bswap(&arg, *bits)?))
        },
        #[cfg(feature = "llvm-11-or-greater")]
        Type::VectorType { scalable: true, .. } => {
            return Err(Error::UnsupportedInstruction("bswap on a scalable vector".into()));
        },
        Type::VectorType {
            element_type,
            num_elements,
            ..
        } => {
            let element_size = state.size_in_bits(&element_type).ok_or_else(|| Error::OtherError("llvm.bswap: argument is vector type, and vector element type contains a struct type with no definition in the Project".into()))?;
            let final_bv = unary_on_vector(&arg, (*num_elements).try_into().unwrap(), |element| {
                bswap(element, element_size)
            })?;
            Ok(ReturnValue::Return(final_bv))
        },
        _ => Err(Error::UnsupportedInstruction(format!(
            "llvm.bswap with argument type {:?}",
            argty
        ))),
    }
}

fn bswap<V: BV>(bv: &V, bits: u32) -> Result<V> {
    match bits {
        16 => {
            let high_byte = bv.slice(15, 8);
            let low_byte = bv.slice(7, 0);
            Ok(low_byte.concat(&high_byte))
        },
        32 => {
            let byte_0 = bv.slice(7, 0);
            let byte_1 = bv.slice(15, 8);
            let byte_2 = bv.slice(23, 16);
            let byte_3 = bv.slice(31, 24);
            Ok(byte_0.concat(&byte_1).concat(&byte_2).concat(&byte_3))
        },
        48 => {
            let byte_0 = bv.slice(7, 0);
            let byte_1 = bv.slice(15, 8);
            let byte_2 = bv.slice(23, 16);
            let byte_3 = bv.slice(31, 24);
            let byte_4 = bv.slice(39, 32);
            let byte_5 = bv.slice(47, 40);
            Ok(byte_0
                .concat(&byte_1)
                .concat(&byte_2)
                .concat(&byte_3)
                .concat(&byte_4)
                .concat(&byte_5))
        },
        64 => {
            let byte_0 = bv.slice(7, 0);
            let byte_1 = bv.slice(15, 8);
            let byte_2 = bv.slice(23, 16);
            let byte_3 = bv.slice(31, 24);
            let byte_4 = bv.slice(39, 32);
            let byte_5 = bv.slice(47, 40);
            let byte_6 = bv.slice(55, 48);
            let byte_7 = bv.slice(63, 56);
            Ok(byte_0
                .concat(&byte_1)
                .concat(&byte_2)
                .concat(&byte_3)
                .concat(&byte_4)
                .concat(&byte_5)
                .concat(&byte_6)
                .concat(&byte_7))
        },
        _ => Err(Error::UnsupportedInstruction(format!(
            "bswap on bitwidth {}",
            bits
        ))),
    }
}

pub fn symex_objectsize<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &'p dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    // We have no way of tracking in-memory types, so we can't provide the
    // intended answers for this intrinsic. Instead, we just always return
    // 'unknown', as this is valid behavior according to the LLVM spec.
    let arg1 = state.operand_to_bv(&call.get_arguments()[1].0)?;
    let width = state.size_in_bits(&state.type_of(call)).ok_or_else(||
        Error::OtherError("symex_objectsize: return value of this call involves a struct type with no definition in the Project".into())
    )?;
    if width == 0 {
        return Err(Error::OtherError(
            "symex_objectsize: didn't expect return type to have size 0 bits".into(),
        ));
    }
    let zero = state.zero(width);
    let minusone = state.ones(width);
    Ok(ReturnValue::Return(arg1.cond_bv(&zero, &minusone)))
}

pub fn symex_assume<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &'p dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 1);
    let arg = &call.get_arguments()[0].0;
    match state.type_of(arg).as_ref() {
        Type::IntegerType { bits: 1 } => {},
        ty => {
            return Err(Error::OtherError(format!(
                "symex_assume: expected arg to be of type i1, got type {:?}",
                ty
            )))
        },
    }

    if state.config.trust_llvm_assumes {
        state.operand_to_bv(arg)?.assert()?;
    } else {
        // just ignore the assume
    }

    Ok(ReturnValue::ReturnVoid)
}

pub fn symex_uadd_with_overflow<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if state.type_of(arg0) != state.type_of(arg1) {
        return Err(Error::OtherError(format!("symex_uadd_with_overflow: expected arguments to be of the same type, but got types {:?} and {:?}", state.type_of(arg0), state.type_of(arg1))));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;
    let result = arg0.add(&arg1);
    let overflow = arg0.uaddo(&arg1);
    assert_eq!(overflow.get_width(), 1);

    Ok(ReturnValue::Return(overflow.concat(&result)))
}

pub fn symex_sadd_with_overflow<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if state.type_of(arg0) != state.type_of(arg1) {
        return Err(Error::OtherError(format!("symex_sadd_with_overflow: expected arguments to be of the same type, but got types {:?} and {:?}", state.type_of(arg0), state.type_of(arg1))));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;
    let result = arg0.add(&arg1);
    let overflow = arg0.saddo(&arg1);
    assert_eq!(overflow.get_width(), 1);

    Ok(ReturnValue::Return(overflow.concat(&result)))
}

pub fn symex_usub_with_overflow<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if state.type_of(arg0) != state.type_of(arg1) {
        return Err(Error::OtherError(format!("symex_usub_with_overflow: expected arguments to be of the same type, but got types {:?} and {:?}", state.type_of(arg0), state.type_of(arg1))));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;
    let result = arg0.sub(&arg1);
    let overflow = arg0.usubo(&arg1);
    assert_eq!(overflow.get_width(), 1);

    Ok(ReturnValue::Return(overflow.concat(&result)))
}

pub fn symex_ssub_with_overflow<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if state.type_of(arg0) != state.type_of(arg1) {
        return Err(Error::OtherError(format!("symex_ssub_with_overflow: expected arguments to be of the same type, but got types {:?} and {:?}", state.type_of(arg0), state.type_of(arg1))));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;
    let result = arg0.sub(&arg1);
    let overflow = arg0.ssubo(&arg1);
    assert_eq!(overflow.get_width(), 1);

    Ok(ReturnValue::Return(overflow.concat(&result)))
}

pub fn symex_umul_with_overflow<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if state.type_of(arg0) != state.type_of(arg1) {
        return Err(Error::OtherError(format!("symex_umul_with_overflow: expected arguments to be of the same type, but got types {:?} and {:?}", state.type_of(arg0), state.type_of(arg1))));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;
    let result = arg0.mul(&arg1);
    let overflow = arg0.umulo(&arg1);
    assert_eq!(overflow.get_width(), 1);

    Ok(ReturnValue::Return(overflow.concat(&result)))
}

pub fn symex_smul_with_overflow<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if state.type_of(arg0) != state.type_of(arg1) {
        return Err(Error::OtherError(format!("symex_smul_with_overflow: expected arguments to be of the same type, but got types {:?} and {:?}", state.type_of(arg0), state.type_of(arg1))));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;
    let result = arg0.mul(&arg1);
    let overflow = arg0.smulo(&arg1);
    assert_eq!(overflow.get_width(), 1);

    Ok(ReturnValue::Return(overflow.concat(&result)))
}

pub fn symex_uadd_sat<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if state.type_of(arg0) != state.type_of(arg1) {
        return Err(Error::OtherError(format!("symex_uadd_sat: expected arguments to be of the same type, but got types {:?} and {:?}", state.type_of(arg0), state.type_of(arg1))));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;

    Ok(ReturnValue::Return(arg0.uadds(&arg1)))
}

pub fn symex_sadd_sat<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if state.type_of(arg0) != state.type_of(arg1) {
        return Err(Error::OtherError(format!("symex_sadd_sat: expected arguments to be of the same type, but got types {:?} and {:?}", state.type_of(arg0), state.type_of(arg1))));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;

    Ok(ReturnValue::Return(arg0.sadds(&arg1)))
}

pub fn symex_usub_sat<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if state.type_of(arg0) != state.type_of(arg1) {
        return Err(Error::OtherError(format!("symex_usub_sat: expected arguments to be of the same type, but got types {:?} and {:?}", state.type_of(arg0), state.type_of(arg1))));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;

    Ok(ReturnValue::Return(arg0.usubs(&arg1)))
}

pub fn symex_ssub_sat<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if state.type_of(arg0) != state.type_of(arg1) {
        return Err(Error::OtherError(format!("symex_ssub_sat: expected arguments to be of the same type, but got types {:?} and {:?}", state.type_of(arg0), state.type_of(arg1))));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;

    Ok(ReturnValue::Return(arg0.ssubs(&arg1)))
}

pub fn symex_ctlz<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    // we ignore the second argument: we always return the bitwidth for a 0 input
    let arg0 = &call.get_arguments()[0].0;

    // Boolector doesn't provide an efficient implementation of ctlz, as of this
    // writing. Wikipedia https://en.wikipedia.org/wiki/Find_first_set suggests
    // a number of software implementations; it's unclear which of them would be
    // most efficient in Boolector. We use binary search for now.
    // Here's the 32-bit implementation in pseudocode (from Wikipedia):
    // function ctlz(x)
    //   if x = 0 return 32
    //   n <- 0
    //   if (x & 0xFFFF0000) = 0: n <- n + 16, x <- x << 16
    //   if (x & 0xFF000000) = 0: n <- n +  8, x <- x <<  8
    //   if (x & 0xF0000000) = 0: n <- n +  4, x <- x <<  4
    //   if (x & 0xC0000000) = 0: n <- n +  2, x <- x <<  2
    //   if (x & 0x80000000) = 0: n <- n +  1
    //   return n

    let x = state.operand_to_bv(arg0)?;
    let width = x.get_width();
    // some constants we'll need
    let zero = state.zero(width);
    let one = state.one(width);
    let two = state.bv_from_u32(2, width);
    let four = state.bv_from_u32(4, width);
    let eight = state.bv_from_u32(8, width);
    let sixteen = state.bv_from_u32(16, width);

    // if x = 0 we'll eventually return `width`
    let x_eq_0 = x._eq(&zero);
    // n <- 0
    let n = zero.clone();

    match width {
        32 => {
            // if (x & 0xFFFF0000) = 0: n <- n + 16, x <- x << 16
            let cond = x.and(&state.bv_from_u32(0xFFFF0000, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&sixteen), &n);
            let x = cond.cond_bv(&x.sll(&sixteen), &x);
            // if (x & 0xFF000000) = 0: n <- n +  8, x <- x <<  8
            let cond = x.and(&state.bv_from_u32(0xFF000000, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&eight), &n);
            let x = cond.cond_bv(&x.sll(&eight), &x);
            // if (x & 0xF0000000) = 0: n <- n +  4, x <- x <<  4
            let cond = x.and(&state.bv_from_u32(0xF0000000, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&four), &n);
            let x = cond.cond_bv(&x.sll(&four), &x);
            // if (x & 0xC0000000) = 0: n <- n +  2, x <- x <<  2
            let cond = x.and(&state.bv_from_u32(0xC0000000, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&two), &n);
            let x = cond.cond_bv(&x.sll(&two), &x);
            // if (x & 0x80000000) = 0: n <- n +  1
            let cond = x.and(&state.bv_from_u32(0x80000000, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&one), &n);
            // if x = 0 return width, else return n
            Ok(ReturnValue::Return(
                x_eq_0.cond_bv(&state.bv_from_u32(width, width), &n),
            ))
        },
        16 => {
            // if (x & 0xFF00) = 0: n <- n + 8, x <- x << 8
            let cond = x.and(&state.bv_from_u32(0xFF00, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&eight), &n);
            let x = cond.cond_bv(&x.sll(&eight), &x);
            // if (x & 0xF000) = 0: n <- n + 4, x <- x << 4
            let cond = x.and(&state.bv_from_u32(0xF000, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&four), &n);
            let x = cond.cond_bv(&x.sll(&four), &x);
            // if (x & 0xC000) = 0: n <- n + 2, x <- x << 2
            let cond = x.and(&state.bv_from_u32(0xC000, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&two), &n);
            let x = cond.cond_bv(&x.sll(&two), &x);
            // if (x & 0x8000) = 0: n <- n + 1
            let cond = x.and(&state.bv_from_u32(0x8000, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&one), &n);
            // if x = 0 return width, else return n
            Ok(ReturnValue::Return(
                x_eq_0.cond_bv(&state.bv_from_u32(width, width), &n),
            ))
        },
        8 => {
            // if (x & 0xF0) = 0: n <- n + 4, x <- x << 4
            let cond = x.and(&state.bv_from_u32(0xF0, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&four), &n);
            let x = cond.cond_bv(&x.sll(&four), &x);
            // if (x & 0xC0) = 0: n <- n + 2, x <- x << 2
            let cond = x.and(&state.bv_from_u32(0xC0, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&two), &n);
            let x = cond.cond_bv(&x.sll(&two), &x);
            // if (x & 0x80) = 0: n <- n + 1
            let cond = x.and(&state.bv_from_u32(0x80, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&one), &n);
            // if x = 0 return width, else return n
            Ok(ReturnValue::Return(
                x_eq_0.cond_bv(&state.bv_from_u32(width, width), &n),
            ))
        },
        w => Err(Error::UnsupportedInstruction(format!(
            "ctlz intrinsic on an operand of width {} bits",
            w
        ))),
    }
}

pub fn symex_cttz<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    // we ignore the second argument: we always return the bitwidth for a 0 input
    let arg0 = &call.get_arguments()[0].0;

    // Boolector doesn't provide an efficient implementation of cttz, as of this
    // writing. Wikipedia https://en.wikipedia.org/wiki/Find_first_set suggests
    // a number of software implementations; it's unclear which of them would be
    // most efficient in Boolector. We use binary search for now.
    // Here's the 32-bit implementation in pseudocode (from Wikipedia):
    // function cttz(x)
    //   if x = 0 return 32
    //   n <- 0
    //   if (x & 0x0000FFFF) = 0: n <- n + 16, x <- x >> 16
    //   if (x & 0x000000FF) = 0: n <- n +  8, x <- x >>  8
    //   if (x & 0x0000000F) = 0: n <- n +  4, x <- x >>  4
    //   if (x & 0x00000003) = 0: n <- n +  2, x <- x >>  2
    //   if (x & 0x00000001) = 0: n <- n +  1
    //   return n

    let x = state.operand_to_bv(arg0)?;
    let width = x.get_width();
    // some constants we'll need
    let zero = state.zero(width);
    let one = state.one(width);
    let two = state.bv_from_u32(2, width);
    let four = state.bv_from_u32(4, width);
    let eight = state.bv_from_u32(8, width);
    let sixteen = state.bv_from_u32(16, width);

    // if x = 0 we'll eventually return `width`
    let x_eq_0 = x._eq(&zero);
    // n <- 0
    let n = zero.clone();

    match width {
        32 => {
            // if (x & 0x0000FFFF) = 0: n <- n + 16, x <- x >> 16
            let cond = x.and(&state.bv_from_u32(0x0000FFFF, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&sixteen), &n);
            let x = cond.cond_bv(&x.srl(&sixteen), &x);
            // if (x & 0x000000FF) = 0: n <- n +  8, x <- x >>  8
            let cond = x.and(&state.bv_from_u32(0x000000FF, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&eight), &n);
            let x = cond.cond_bv(&x.srl(&eight), &x);
            // if (x & 0x0000000F) = 0: n <- n +  4, x <- x >>  4
            let cond = x.and(&state.bv_from_u32(0x0000000F, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&four), &n);
            let x = cond.cond_bv(&x.srl(&four), &x);
            // if (x & 0x00000003) = 0: n <- n +  2, x <- x >>  2
            let cond = x.and(&state.bv_from_u32(0x00000003, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&two), &n);
            let x = cond.cond_bv(&x.srl(&two), &x);
            // if (x & 0x00000001) = 0: n <- n +  1
            let cond = x.and(&state.bv_from_u32(0x00000001, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&one), &n);
            // if x = 0 return width, else return n
            Ok(ReturnValue::Return(
                x_eq_0.cond_bv(&state.bv_from_u32(width, width), &n),
            ))
        },
        16 => {
            // if (x & 0x00FF) = 0: n <- n + 8, x <- x >> 8
            let cond = x.and(&state.bv_from_u32(0x00FF, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&eight), &n);
            let x = cond.cond_bv(&x.srl(&eight), &x);
            // if (x & 0x000F) = 0: n <- n + 4, x <- x >> 4
            let cond = x.and(&state.bv_from_u32(0x000F, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&four), &n);
            let x = cond.cond_bv(&x.srl(&four), &x);
            // if (x & 0x0003) = 0: n <- n + 2, x <- x >> 2
            let cond = x.and(&state.bv_from_u32(0x0003, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&two), &n);
            let x = cond.cond_bv(&x.srl(&two), &x);
            // if (x & 0x0001) = 0: n <- n + 1
            let cond = x.and(&state.bv_from_u32(0x0001, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&one), &n);
            // if x = 0 return width, else return n
            Ok(ReturnValue::Return(
                x_eq_0.cond_bv(&state.bv_from_u32(width, width), &n),
            ))
        },
        8 => {
            // if (x & 0x0F) = 0: n <- n + 4, x <- x >> 4
            let cond = x.and(&state.bv_from_u32(0x0F, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&four), &n);
            let x = cond.cond_bv(&x.srl(&four), &x);
            // if (x & 0x03) = 0: n <- n + 2, x <- x >> 2
            let cond = x.and(&state.bv_from_u32(0x03, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&two), &n);
            let x = cond.cond_bv(&x.srl(&two), &x);
            // if (x & 0x01) = 0: n <- n + 1
            let cond = x.and(&state.bv_from_u32(0x01, width))._eq(&zero);
            let n = cond.cond_bv(&n.add(&one), &n);
            // if x = 0 return width, else return n
            Ok(ReturnValue::Return(
                x_eq_0.cond_bv(&state.bv_from_u32(width, width), &n),
            ))
        },
        w => Err(Error::UnsupportedInstruction(format!(
            "cttz intrinsic on an operand of width {} bits",
            w
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::DefaultBackend;
    use crate::function_hooks::Argument;
    use crate::test_utils::*;
    use either::Either;
    use llvm_ir::types::{Typed, Types};
    use llvm_ir::*;

    fn constant_operand(c: Constant) -> Operand {
        Operand::ConstantOperand(ConstantRef::new(c))
    }

    /// just something to implement `IsCall`
    struct DummyCall {
        args: Vec<Argument>,
    }

    impl DummyCall {
        fn new_twoarg_call(arg0: Operand, arg1: Operand) -> Self {
            Self {
                args: vec![(arg0, vec![]), (arg1, vec![])],
            }
        }
    }

    impl Typed for DummyCall {
        fn get_type(&self, _types: &Types) -> TypeRef {
            unimplemented!()
        }
    }

    impl IsCall for DummyCall {
        fn get_called_func(&self) -> &Either<instruction::InlineAssembly, Operand> {
            unimplemented!()
        }
        fn get_arguments(&self) -> &Vec<Argument> {
            &self.args
        }
        fn get_return_attrs(&self) -> &Vec<function::ParameterAttribute> {
            unimplemented!()
        }
        fn get_fn_attrs(&self) -> &Vec<function::FunctionAttribute> {
            unimplemented!()
        }
        fn get_calling_convention(&self) -> function::CallingConvention {
            unimplemented!()
        }
    }

    #[test]
    fn sadd_with_overflow() {
        let project = blank_project(
            "test_mod",
            blank_function("test_func", vec![Name::from("test_bb")]),
        );
        let mut state = blank_state(&project, "test_func");

        let four = constant_operand(Constant::Int {
            bits: 8,
            value: 4,
        });
        let sixty_four = constant_operand(Constant::Int {
            bits: 8,
            value: 64
        });
        let one_hundred = constant_operand(Constant::Int {
            bits: 8,
            value: 100,
        });

        {
            let call = DummyCall::new_twoarg_call(four.clone(), one_hundred.clone());
            match symex_sadd_with_overflow(&project, &mut state, &call).unwrap() {
                ReturnValue::Return(bv) => {
                    let result = bv.slice(7, 0).as_u64().unwrap();
                    let overflow = bv.slice(8, 8).as_u64().unwrap();
                    assert_eq!(result, 104);
                    assert_eq!(overflow, 0);
                },
                ret => panic!("Unexpected return value: {:?}", ret),
            }
        }

        {
            let call = DummyCall::new_twoarg_call(sixty_four.clone(), one_hundred.clone());
            match symex_sadd_with_overflow(&project, &mut state, &call).unwrap() {
                ReturnValue::Return(bv) => {
                    let result = bv.slice(7, 0).as_u64().unwrap();
                    let overflow = bv.slice(8, 8).as_u64().unwrap();
                    assert_eq!(result, 164); // 164 unsigned, which is a negative value for 8-bit signed
                    assert_eq!(overflow, 1);
                },
                ret => panic!("Unexpected return value: {:?}", ret),
            }
        }
    }

    #[test]
    fn umul_with_overflow() {
        let project = blank_project(
            "test_mod",
            blank_function("test_func", vec![Name::from("test_bb")]),
        );
        let mut state = blank_state(&project, "test_func");

        let four = constant_operand(Constant::Int { bits: 8, value: 4 });
        let eight = constant_operand(Constant::Int { bits: 8, value: 8 });
        let sixty_four = constant_operand(Constant::Int { bits: 8, value: 64 });

        {
            let call = DummyCall::new_twoarg_call(four.clone(), eight.clone());
            match symex_umul_with_overflow(&project, &mut state, &call).unwrap() {
                ReturnValue::Return(bv) => {
                    let result = bv.slice(7, 0).as_u64().unwrap();
                    let overflow = bv.slice(8, 8).as_u64().unwrap();
                    assert_eq!(result, 32);
                    assert_eq!(overflow, 0);
                },
                ret => panic!("Unexpected return value: {:?}", ret),
            }
        }

        {
            let call = DummyCall::new_twoarg_call(eight.clone(), sixty_four.clone());
            match symex_umul_with_overflow(&project, &mut state, &call).unwrap() {
                ReturnValue::Return(bv) => {
                    let result = bv.slice(7, 0).as_u64().unwrap();
                    let overflow = bv.slice(8, 8).as_u64().unwrap();
                    assert_eq!(result, 0);
                    assert_eq!(overflow, 1);
                },
                ret => panic!("Unexpected return value: {:?}", ret),
            }
        }
    }

    #[test]
    fn usubs() {
        let project = blank_project(
            "test_mod",
            blank_function("test_func", vec![Name::from("test_bb")]),
        );
        let mut state = blank_state(&project, "test_func");

        // these are the examples from the LLVM 9 docs
        let two = constant_operand(Constant::Int { bits: 4, value: 2 });
        let one = constant_operand(Constant::Int { bits: 4, value: 1 });
        let six = constant_operand(Constant::Int { bits: 4, value: 6 });

        let call = DummyCall::new_twoarg_call(two.clone(), one.clone());
        match symex_usub_sat(&project, &mut state, &call).unwrap() {
            ReturnValue::Return(bv) => {
                assert_eq!(bv.as_u64().unwrap(), 1);
            },
            ret => panic!("Unexpected return value: {:?}", ret),
        }

        let call = DummyCall::new_twoarg_call(two.clone(), six.clone());
        match symex_usub_sat(&project, &mut state, &call).unwrap() {
            ReturnValue::Return(bv) => {
                assert_eq!(bv.as_u64().unwrap(), 0);
            },
            ret => panic!("Unexpected return value: {:?}", ret),
        }
    }

    #[test]
    fn sadds() {
        let project = blank_project(
            "test_mod",
            blank_function("test_func", vec![Name::from("test_bb")]),
        );
        let mut state = blank_state(&project, "test_func");

        // these are the examples from the LLVM 9 docs
        let one = constant_operand(Constant::Int { bits: 4, value: 1 });
        let two = constant_operand(Constant::Int { bits: 4, value: 2 });
        let five = constant_operand(Constant::Int { bits: 4, value: 5 });
        let six = constant_operand(Constant::Int { bits: 4, value: 6 });
        let minusfour = constant_operand(Constant::Int {
            bits: 4,
            value: (-4_i64) as u64,
        });
        let minusfive = constant_operand(Constant::Int {
            bits: 4,
            value: (-5_i64) as u64,
        });

        let call = DummyCall::new_twoarg_call(one.clone(), two.clone());
        match symex_sadd_sat(&project, &mut state, &call).unwrap() {
            ReturnValue::Return(bv) => {
                assert_eq!(bv.as_u64().unwrap(), 3);
            },
            ret => panic!("Unexpected return value: {:?}", ret),
        }

        let call = DummyCall::new_twoarg_call(five.clone(), six.clone());
        match symex_sadd_sat(&project, &mut state, &call).unwrap() {
            ReturnValue::Return(bv) => {
                assert_eq!(bv.as_u64().unwrap(), 7);
            },
            ret => panic!("Unexpected return value: {:?}", ret),
        }

        let call = DummyCall::new_twoarg_call(minusfour.clone(), two.clone());
        match symex_sadd_sat(&project, &mut state, &call).unwrap() {
            ReturnValue::Return(bv) => {
                assert_eq!(
                    bv.as_u64().unwrap(),
                    u64::from_str_radix("1110", 2).unwrap()
                ); // -2
            },
            ret => panic!("Unexpected return value: {:?}", ret),
        }

        let call = DummyCall::new_twoarg_call(minusfour.clone(), minusfive.clone());
        match symex_sadd_sat(&project, &mut state, &call).unwrap() {
            ReturnValue::Return(bv) => {
                assert_eq!(
                    bv.as_u64().unwrap(),
                    u64::from_str_radix("1000", 2).unwrap()
                ); // -8
            },
            ret => panic!("Unexpected return value: {:?}", ret),
        }
    }

    fn test_ctlz<'p>(
        proj: &'p Project,
        state: &mut State<'p, DefaultBackend>,
        width: u32,
        input: u32,
        output: u32,
    ) {
        let call = DummyCall::new_twoarg_call(
            constant_operand(Constant::Int {
                bits: width,
                value: input.into(),
            }),
            constant_operand(Constant::Int { bits: 1, value: 1 }),
        );
        match symex_ctlz(proj, state, &call).unwrap() {
            ReturnValue::Return(bv) => {
                let outval = bv.as_u64().unwrap();
                assert_eq!(
                    outval,
                    output.into(),
                    "Expected {}-bit ctlz({:#x}) = {}, got {}",
                    width,
                    input,
                    output,
                    outval
                );
            },
            ret => panic!("Unexpected return value: {:?}", ret),
        }
    }

    fn test_cttz<'p>(
        proj: &'p Project,
        state: &mut State<'p, DefaultBackend>,
        width: u32,
        input: u32,
        output: u32,
    ) {
        let call = DummyCall::new_twoarg_call(
            constant_operand(Constant::Int {
                bits: width,
                value: input.into(),
            }),
            constant_operand(Constant::Int { bits: 1, value: 1 }),
        );
        match symex_cttz(proj, state, &call).unwrap() {
            ReturnValue::Return(bv) => {
                let outval = bv.as_u64().unwrap();
                assert_eq!(
                    outval,
                    output.into(),
                    "Expected {}-bit cttz({:#x}) = {}, got {}",
                    width,
                    input,
                    output,
                    outval
                );
            },
            ret => panic!("Unexpected return value: {:?}", ret),
        }
    }

    #[test]
    fn ctlz() {
        let proj = blank_project(
            "test_mod",
            blank_function("test_func", vec![Name::from("test_bb")]),
        );
        let mut state = blank_state(&proj, "test_func");

        // 32-bit ctlz(0) = 32
        test_ctlz(&proj, &mut state, 32, 0, 32);
        // 16-bit ctlz(0) = 16
        test_ctlz(&proj, &mut state, 16, 0, 16);
        // 8-bit ctlz(0) = 8
        test_ctlz(&proj, &mut state, 8, 0, 8);

        // 32-bit ctlz(1) = 31
        test_ctlz(&proj, &mut state, 32, 1, 31);
        // 16-bit ctlz(1) = 15
        test_ctlz(&proj, &mut state, 16, 1, 15);
        // 8-bit ctlz(1) = 7
        test_ctlz(&proj, &mut state, 8, 1, 7);

        // 32-bit ctlz(0x0000_000F) = 28
        test_ctlz(&proj, &mut state, 32, 0x0000_000F, 28);
        // 16-bit ctlz(0x000F) = 12
        test_ctlz(&proj, &mut state, 16, 0x000F, 12);
        // 8-bit ctlz(0x0F) = 4
        test_ctlz(&proj, &mut state, 8, 0x0F, 4);

        // 32-bit ctlz(0x0000_0008) = 28
        test_ctlz(&proj, &mut state, 32, 0x0000_0008, 28);
        // 16-bit ctlz(0x0008) = 12
        test_ctlz(&proj, &mut state, 16, 0x0008, 12);
        // 8-bit ctlz(0x08) = 4
        test_ctlz(&proj, &mut state, 8, 0x08, 4);

        // 32-bit ctlz(0x0001_0000) = 15
        test_ctlz(&proj, &mut state, 32, 0x0001_0000, 15);
        // 16-bit ctlz(0x0100) = 7
        test_ctlz(&proj, &mut state, 16, 0x0100, 7);
        // 8-bit ctlz(0x10) = 3
        test_ctlz(&proj, &mut state, 8, 0x10, 3);

        // 32-bit ctlz(0x0010_1234) = 11
        test_ctlz(&proj, &mut state, 32, 0x0010_1234, 11);
        // 16-bit ctlz(0x037B) = 6
        test_ctlz(&proj, &mut state, 16, 0x037B, 6);
        // 8-bit ctlz(0x37) = 2
        test_ctlz(&proj, &mut state, 8, 0x37, 2);

        // 32-bit ctlz(0x5555_AAAA) = 1
        test_ctlz(&proj, &mut state, 32, 0x5555_AAAA, 1);
        // 16-bit ctlz(0x55AA) = 1
        test_ctlz(&proj, &mut state, 16, 0x55AA, 1);
        // 8-bit ctlz(0x5A) = 1
        test_ctlz(&proj, &mut state, 8, 0x5A, 1);

        // 32-bit ctlz(0xFFFF_FFFF) = 0
        test_ctlz(&proj, &mut state, 32, 0xFFFF_FFFF, 0);
        // 16-bit ctlz(0xFFFF) = 0
        test_ctlz(&proj, &mut state, 16, 0xFFFF, 0);
        // 8-bit ctlz(0xFF) = 0
        test_ctlz(&proj, &mut state, 8, 0xFF, 0);

        // 32-bit ctlz(0x8000_000F) = 0
        test_ctlz(&proj, &mut state, 32, 0x8000_000F, 0);
        // 16-bit ctlz(0x800F) = 0
        test_ctlz(&proj, &mut state, 16, 0x800F, 0);
        // 8-bit ctlz(0x8F) = 0
        test_ctlz(&proj, &mut state, 8, 0x8F, 0);
    }

    #[test]
    fn cttz() {
        let proj = blank_project(
            "test_mod",
            blank_function("test_func", vec![Name::from("test_bb")]),
        );
        let mut state = blank_state(&proj, "test_func");

        // 32-bit cttz(0) = 32
        test_cttz(&proj, &mut state, 32, 0, 32);
        // 16-bit cttz(0) = 16
        test_cttz(&proj, &mut state, 16, 0, 16);
        // 8-bit cttz(0) = 8
        test_cttz(&proj, &mut state, 8, 0, 8);

        // 32-bit cttz(1) = 0
        test_cttz(&proj, &mut state, 32, 1, 0);
        // 16-bit cttz(1) = 0
        test_cttz(&proj, &mut state, 16, 1, 0);
        // 8-bit cttz(1) = 0
        test_cttz(&proj, &mut state, 8, 1, 0);

        // 32-bit cttz(0x8000_0000) = 31
        test_cttz(&proj, &mut state, 32, 0x8000_0000, 31);
        // 16-bit cttz(0x8000) = 15
        test_cttz(&proj, &mut state, 16, 0x8000, 15);
        // 8-bit cttz(0x80) = 7
        test_cttz(&proj, &mut state, 8, 0x80, 7);

        // 32-bit cttz(0xF000_0000) = 28
        test_cttz(&proj, &mut state, 32, 0xF000_0000, 28);
        // 16-bit cttz(0xF000) = 12
        test_cttz(&proj, &mut state, 16, 0xF000, 12);
        // 8-bit cttz(0xF0) = 4
        test_cttz(&proj, &mut state, 8, 0xF0, 4);

        // 32-bit cttz(0x1000_0000) = 28
        test_cttz(&proj, &mut state, 32, 0x1000_0000, 28);
        // 16-bit cttz(0x1000) = 12
        test_cttz(&proj, &mut state, 16, 0x1000, 12);
        // 8-bit cttz(0x10) = 4
        test_cttz(&proj, &mut state, 8, 0x10, 4);

        // 32-bit cttz(0x0000_F000) = 12
        test_cttz(&proj, &mut state, 32, 0x0000_F000, 12);
        // 16-bit cttz(0x00F0) = 4
        test_cttz(&proj, &mut state, 16, 0x00F0, 4);
        // 8-bit cttz(0x0C) = 2
        test_cttz(&proj, &mut state, 8, 0x0C, 2);

        // 32-bit cttz(0x4321_FA00) = 9
        test_cttz(&proj, &mut state, 32, 0x4321_FA00, 9);
        // 16-bit cttz(0x43A0) = 5
        test_cttz(&proj, &mut state, 16, 0x43A0, 5);
        // 8-bit cttz(0x48) = 3
        test_cttz(&proj, &mut state, 8, 0x48, 3);

        // 32-bit cttz(0x5555_AAAA) = 1
        test_cttz(&proj, &mut state, 32, 0x5555_AAAA, 1);
        // 16-bit cttz(0x55AA) = 1
        test_cttz(&proj, &mut state, 16, 0x55AA, 1);
        // 8-bit cttz(0x5A) = 1
        test_cttz(&proj, &mut state, 8, 0x5A, 1);

        // 32-bit cttz(0xFFFF_FFFF) = 0
        test_cttz(&proj, &mut state, 32, 0xFFFF_FFFF, 0);
        // 16-bit cttz(0xFFFF) = 0
        test_cttz(&proj, &mut state, 16, 0xFFFF, 0);
        // 8-bit cttz(0xFF) = 0
        test_cttz(&proj, &mut state, 8, 0xFF, 0);

        // 32-bit cttz(0xF000_0001) = 0
        test_cttz(&proj, &mut state, 32, 0xF000_0001, 0);
        // 16-bit cttz(0xF001) = 0
        test_cttz(&proj, &mut state, 16, 0xF001, 0);
        // 8-bit cttz(0xF1) = 0
        test_cttz(&proj, &mut state, 8, 0xF1, 0);
    }
}
