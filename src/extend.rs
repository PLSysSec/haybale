use crate::backend::BV;

/// Zero-extend a `BV` to the specified number of bits.
/// The input `BV` can be already the desired size (in which case this function is a no-op)
/// or smaller (in which case this function will extend),
/// but not larger (in which case this function will panic).
pub fn zero_extend_to_bits<V: BV>(bv: V, bits: u32) -> V {
    let cur_bits = bv.get_width();
    if cur_bits == bits {
        bv
    } else if cur_bits < bits {
        bv.zext(bits - cur_bits)
    } else {
        panic!("tried to zero-extend to {} bits, but already had {} bits", bits, cur_bits)
    }
}

/// Sign-extend a `BV` to the specified number of bits.
/// The input `BV` can be already the desired size (in which case this function is a no-op)
/// or smaller (in which case this function will extend),
/// but not larger (in which case this function will panic).
pub fn sign_extend_to_bits<V: BV>(bv: V, bits: u32) -> V {
    let cur_bits = bv.get_width();
    if cur_bits == bits {
        bv
    } else if cur_bits < bits {
        bv.sext(bits - cur_bits)
    } else {
        panic!("tried to sign-extend to {} bits, but already had {} bits", bits, cur_bits)
    }
}
