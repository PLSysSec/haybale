//! Default hooks for some LLVM intrinsics

use crate::backend::{Backend, BV};
use crate::config::Concretize;
use crate::error::*;
use crate::function_hooks::IsCall;
use crate::layout;
use crate::project::Project;
use crate::return_value::ReturnValue;
use crate::solver_utils::PossibleSolutions;
use crate::state::State;
use llvm_ir::{Type, Typed};
use log::{debug, warn};
use reduce::Reduce;
use std::convert::TryFrom;

pub fn symex_memset<'p, B: Backend>(_proj: &'p Project, state: &mut State<'p, B>, call: &'p dyn IsCall) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 4);
    let addr = &call.get_arguments()[0].0;
    let val = &call.get_arguments()[1].0;
    let num_bytes = &call.get_arguments()[2].0;
    assert_eq!(addr.get_type(), Type::pointer_to(Type::i8()));

    let addr = state.operand_to_bv(&addr)?;
    let val = {
        let mut val = state.operand_to_bv(&val)?;
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

    // if the call should return a pointer, it returns `addr`. If it's void-typed, that's fine too.
    match call.get_type() {
       Type::VoidType => Ok(ReturnValue::ReturnVoid),
       Type::PointerType { .. } => Ok(ReturnValue::Return(addr)),
       ty => Err(Error::OtherError(format!("Unexpected return type for a memset: {:?}", ty))),
    }
}

pub fn symex_memcpy<'p, B: Backend>(_proj: &'p Project, state: &mut State<'p, B>, call: &'p dyn IsCall) -> Result<ReturnValue<B::BV>> {
    let dest = &call.get_arguments()[0].0;
    let src = &call.get_arguments()[1].0;
    let num_bytes = &call.get_arguments()[2].0;
    assert_eq!(dest.get_type(), Type::pointer_to(Type::i8()));
    assert_eq!(src.get_type(), Type::pointer_to(Type::i8()));

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

    // if the call should return a pointer, it returns `dest`. If it's void-typed, that's fine too.
    match call.get_type() {
       Type::VoidType => Ok(ReturnValue::ReturnVoid),
       Type::PointerType { .. } => Ok(ReturnValue::Return(dest)),
       ty => Err(Error::OtherError(format!("Unexpected return type for a memcpy or memmove: {:?}", ty))),
    }
}

pub fn symex_bswap<'p, B: Backend>(_proj: &'p Project, state: &mut State<'p, B>, call: &'p dyn IsCall) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 1);
    let arg = &call.get_arguments()[0].0;
    let argty = arg.get_type();
    let retty = call.get_type();
    if argty != retty {
        return Err(Error::OtherError("Expected bswap argument to be the same type as its return type".to_owned()));
    }

    let arg = state.operand_to_bv(arg)?;
    match argty {
        Type::IntegerType { bits: 16 } => {
            assert_eq!(arg.get_width(), 16);
            let high_byte = arg.slice(15, 8);
            let low_byte = arg.slice(7, 0);
            Ok(ReturnValue::Return(low_byte.concat(&high_byte)))
        },
        Type::IntegerType { bits: 32 } => {
            assert_eq!(arg.get_width(), 32);
            let byte_0 = arg.slice(7, 0);
            let byte_1 = arg.slice(15, 8);
            let byte_2 = arg.slice(23, 16);
            let byte_3 = arg.slice(31, 24);
            Ok(ReturnValue::Return(
                byte_0.concat(&byte_1).concat(&byte_2).concat(&byte_3)
            ))
        },
        Type::IntegerType { bits: 48 } => {
            assert_eq!(arg.get_width(), 48);
            let byte_0 = arg.slice(7, 0);
            let byte_1 = arg.slice(15, 8);
            let byte_2 = arg.slice(23, 16);
            let byte_3 = arg.slice(31, 24);
            let byte_4 = arg.slice(39, 32);
            let byte_5 = arg.slice(47, 40);
            Ok(ReturnValue::Return(
                byte_0.concat(&byte_1).concat(&byte_2).concat(&byte_3).concat(&byte_4).concat(&byte_5)
            ))
        },
        Type::IntegerType { bits: 64 } => {
            assert_eq!(arg.get_width(), 64);
            let byte_0 = arg.slice(7, 0);
            let byte_1 = arg.slice(15, 8);
            let byte_2 = arg.slice(23, 16);
            let byte_3 = arg.slice(31, 24);
            let byte_4 = arg.slice(39, 32);
            let byte_5 = arg.slice(47, 40);
            let byte_6 = arg.slice(55, 48);
            let byte_7 = arg.slice(63, 56);
            Ok(ReturnValue::Return(
                byte_0.concat(&byte_1).concat(&byte_2).concat(&byte_3).concat(&byte_4).concat(&byte_5).concat(&byte_6).concat(&byte_7)
            ))
        },
        _ => Err(Error::UnsupportedInstruction(format!("llvm.bswap with argument type {:?}", argty))),
    }
}

pub fn symex_objectsize<'p, B: Backend>(_proj: &'p Project, state: &mut State<'p, B>, call: &'p dyn IsCall) -> Result<ReturnValue<B::BV>> {
    // We have no way of tracking in-memory types, so we can't provide the
    // intended answers for this intrinsic. Instead, we just always return
    // 'unknown', as this is valid behavior according to the LLVM spec.
    let arg1 = state.operand_to_bv(&call.get_arguments()[1].0)?;
    let width = layout::size(&call.get_type());
    let zero = state.zero(width as u32);
    let minusone = state.ones(width as u32);
    Ok(ReturnValue::Return(arg1.cond_bv(&zero, &minusone)))
}

pub fn symex_assume<'p, B: Backend>(_proj: &'p Project, state: &mut State<'p, B>, call: &'p dyn IsCall) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 1);
    let arg = &call.get_arguments()[0].0;
    match arg.get_type() {
        Type::IntegerType { bits: 1 } => {},
        ty => return Err(Error::OtherError(format!("symex_assume: expected arg to be of type i1, got type {:?}", ty))),
    }

    if state.config.trust_llvm_assumes {
        state.operand_to_bv(arg)?.assert()?;
    } else {
        // just ignore the assume
    }

    Ok(ReturnValue::ReturnVoid)
}

pub fn symex_uadd_with_overflow<'p, B: Backend>(_proj: &'p Project, state: &mut State<'p, B>, call: &dyn IsCall) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if arg0.get_type() != arg1.get_type() {
        return Err(Error::OtherError(format!("symex_uadd_with_overflow: expected arguments to be of the same type, but got types {:?} and {:?}", arg0.get_type(), arg1.get_type())));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;
    let result = arg0.add(&arg1);
    let overflow = arg0.uaddo(&arg1);
    assert_eq!(overflow.get_width(), 1);

    Ok(ReturnValue::Return(overflow.concat(&result)))
}

pub fn symex_sadd_with_overflow<'p, B: Backend>(_proj: &'p Project, state: &mut State<'p, B>, call: &dyn IsCall) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if arg0.get_type() != arg1.get_type() {
        return Err(Error::OtherError(format!("symex_sadd_with_overflow: expected arguments to be of the same type, but got types {:?} and {:?}", arg0.get_type(), arg1.get_type())));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;
    let result = arg0.add(&arg1);
    let overflow = arg0.saddo(&arg1);
    assert_eq!(overflow.get_width(), 1);

    Ok(ReturnValue::Return(overflow.concat(&result)))
}

pub fn symex_usub_with_overflow<'p, B: Backend>(_proj: &'p Project, state: &mut State<'p, B>, call: &dyn IsCall) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if arg0.get_type() != arg1.get_type() {
        return Err(Error::OtherError(format!("symex_usub_with_overflow: expected arguments to be of the same type, but got types {:?} and {:?}", arg0.get_type(), arg1.get_type())));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;
    let result = arg0.sub(&arg1);
    let overflow = arg0.usubo(&arg1);
    assert_eq!(overflow.get_width(), 1);

    Ok(ReturnValue::Return(overflow.concat(&result)))
}

pub fn symex_ssub_with_overflow<'p, B: Backend>(_proj: &'p Project, state: &mut State<'p, B>, call: &dyn IsCall) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if arg0.get_type() != arg1.get_type() {
        return Err(Error::OtherError(format!("symex_ssub_with_overflow: expected arguments to be of the same type, but got types {:?} and {:?}", arg0.get_type(), arg1.get_type())));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;
    let result = arg0.sub(&arg1);
    let overflow = arg0.ssubo(&arg1);
    assert_eq!(overflow.get_width(), 1);

    Ok(ReturnValue::Return(overflow.concat(&result)))
}

pub fn symex_umul_with_overflow<'p, B: Backend>(_proj: &'p Project, state: &mut State<'p, B>, call: &dyn IsCall) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if arg0.get_type() != arg1.get_type() {
        return Err(Error::OtherError(format!("symex_umul_with_overflow: expected arguments to be of the same type, but got types {:?} and {:?}", arg0.get_type(), arg1.get_type())));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;
    let result = arg0.mul(&arg1);
    let overflow = arg0.umulo(&arg1);
    assert_eq!(overflow.get_width(), 1);

    Ok(ReturnValue::Return(overflow.concat(&result)))
}

pub fn symex_smul_with_overflow<'p, B: Backend>(_proj: &'p Project, state: &mut State<'p, B>, call: &dyn IsCall) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if arg0.get_type() != arg1.get_type() {
        return Err(Error::OtherError(format!("symex_smul_with_overflow: expected arguments to be of the same type, but got types {:?} and {:?}", arg0.get_type(), arg1.get_type())));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;
    let result = arg0.mul(&arg1);
    let overflow = arg0.smulo(&arg1);
    assert_eq!(overflow.get_width(), 1);

    Ok(ReturnValue::Return(overflow.concat(&result)))
}

pub fn symex_uadd_sat<'p, B: Backend>(_proj: &'p Project, state: &mut State<'p, B>, call: &dyn IsCall) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if arg0.get_type() != arg1.get_type() {
        return Err(Error::OtherError(format!("symex_uadd_sat: expected arguments to be of the same type, but got types {:?} and {:?}", arg0.get_type(), arg1.get_type())));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;

    Ok(ReturnValue::Return(arg0.uadds(&arg1)))
}

pub fn symex_sadd_sat<'p, B: Backend>(_proj: &'p Project, state: &mut State<'p, B>, call: &dyn IsCall) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if arg0.get_type() != arg1.get_type() {
        return Err(Error::OtherError(format!("symex_sadd_sat: expected arguments to be of the same type, but got types {:?} and {:?}", arg0.get_type(), arg1.get_type())));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;

    Ok(ReturnValue::Return(arg0.sadds(&arg1)))
}

pub fn symex_usub_sat<'p, B: Backend>(_proj: &'p Project, state: &mut State<'p, B>, call: &dyn IsCall) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if arg0.get_type() != arg1.get_type() {
        return Err(Error::OtherError(format!("symex_usub_sat: expected arguments to be of the same type, but got types {:?} and {:?}", arg0.get_type(), arg1.get_type())));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;

    Ok(ReturnValue::Return(arg0.usubs(&arg1)))
}

pub fn symex_ssub_sat<'p, B: Backend>(_proj: &'p Project, state: &mut State<'p, B>, call: &dyn IsCall) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let arg0 = &call.get_arguments()[0].0;
    let arg1 = &call.get_arguments()[1].0;
    if arg0.get_type() != arg1.get_type() {
        return Err(Error::OtherError(format!("symex_ssub_sat: expected arguments to be of the same type, but got types {:?} and {:?}", arg0.get_type(), arg1.get_type())));
    }

    let arg0 = state.operand_to_bv(arg0)?;
    let arg1 = state.operand_to_bv(arg1)?;

    Ok(ReturnValue::Return(arg0.ssubs(&arg1)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::function_hooks::Argument;
    use crate::test_utils::*;
    use either::Either;
    use llvm_ir::*;

    /// just something to implement `IsCall`
    struct DummyCall {
        args: Vec<Argument>,
    }

    impl DummyCall {
        fn new_twoarg_call(arg0: Operand, arg1: Operand) -> Self {
            Self {
                args: vec![
                    (arg0, vec![]),
                    (arg1, vec![]),
                ]
            }
        }
    }

    impl Typed for DummyCall {
        fn get_type(&self) -> Type {
            unimplemented!()
        }
    }

    impl IsCall for DummyCall {
        fn get_called_func(&self) -> &Either<instruction::InlineAssembly, Operand> { unimplemented!() }
        fn get_arguments(&self) -> &Vec<Argument> {
            &self.args
        }
        fn get_return_attrs(&self) -> &Vec<function::ParameterAttribute> { unimplemented!() }
        fn get_fn_attrs(&self) -> &Vec<function::FunctionAttribute> { unimplemented!() }
        fn get_calling_convention(&self) -> function::CallingConvention { unimplemented!() }
    }

    #[test]
    fn sadd_with_overflow() {
        let project = blank_project("test_mod", blank_function("test_func", vec![Name::from("test_bb")]));
        let mut state = blank_state(&project, "test_func");

        let four = Operand::ConstantOperand(Constant::Int { bits: 8, value: 4 });
        let sixty_four = Operand::ConstantOperand(Constant::Int { bits: 8, value: 64 });
        let one_hundred = Operand::ConstantOperand(Constant::Int { bits: 8, value: 100 });

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
                    assert_eq!(result, 164);  // 164 unsigned, which is a negative value for 8-bit signed
                    assert_eq!(overflow, 1);
                },
                ret => panic!("Unexpected return value: {:?}", ret),
            }
        }
    }

    #[test]
    fn umul_with_overflow() {
        let project = blank_project("test_mod", blank_function("test_func", vec![Name::from("test_bb")]));
        let mut state = blank_state(&project, "test_func");

        let four = Operand::ConstantOperand(Constant::Int { bits: 8, value: 4 });
        let eight = Operand::ConstantOperand(Constant::Int { bits: 8, value: 8 });
        let sixty_four = Operand::ConstantOperand(Constant::Int { bits: 8, value: 64 });

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
        let project = blank_project("test_mod", blank_function("test_func", vec![Name::from("test_bb")]));
        let mut state = blank_state(&project, "test_func");

        // these are the examples from the LLVM 9 docs
        let two = Operand::ConstantOperand(Constant::Int { bits: 4, value: 2 });
        let one = Operand::ConstantOperand(Constant::Int { bits: 4, value: 1 });
        let six = Operand::ConstantOperand(Constant::Int { bits: 4, value: 6 });

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
        let project = blank_project("test_mod", blank_function("test_func", vec![Name::from("test_bb")]));
        let mut state = blank_state(&project, "test_func");

        // these are the examples from the LLVM 9 docs
        let one = Operand::ConstantOperand(Constant::Int { bits: 4, value: 1 });
        let two = Operand::ConstantOperand(Constant::Int { bits: 4, value: 2 });
        let five = Operand::ConstantOperand(Constant::Int { bits: 4, value: 5 });
        let six = Operand::ConstantOperand(Constant::Int { bits: 4, value: 6 });
        let minusfour = Operand::ConstantOperand(Constant::Int { bits: 4, value: (-4_i64) as u64 });
        let minusfive = Operand::ConstantOperand(Constant::Int { bits: 4, value: (-5_i64) as u64 });

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
                assert_eq!(bv.as_u64().unwrap(), u64::from_str_radix("1110", 2).unwrap());  // -2
            },
            ret => panic!("Unexpected return value: {:?}", ret),
        }

        let call = DummyCall::new_twoarg_call(minusfour.clone(), minusfive.clone());
        match symex_sadd_sat(&project, &mut state, &call).unwrap() {
            ReturnValue::Return(bv) => {
                assert_eq!(bv.as_u64().unwrap(), u64::from_str_radix("1000", 2).unwrap());  // -8
            },
            ret => panic!("Unexpected return value: {:?}", ret),
        }
    }
}
