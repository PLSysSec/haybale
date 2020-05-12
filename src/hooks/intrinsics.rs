//! Default hooks for some LLVM intrinsics

use crate::backend::{Backend, BV};
use crate::error::*;
use crate::function_hooks::IsCall;
use crate::hook_utils;
use crate::layout;
use crate::project::Project;
use crate::return_value::ReturnValue;
use crate::state::State;
use crate::symex::unary_on_vector;
use llvm_ir::{Type, Typed};

pub fn symex_memset<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &'p dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 4);
    let addr = &call.get_arguments()[0].0;
    let val = &call.get_arguments()[1].0;
    let num_bytes = &call.get_arguments()[2].0;
    assert_eq!(addr.get_type(), Type::pointer_to(Type::i8()));

    let addr = hook_utils::memset(state, addr, val, num_bytes)?;

    // if the call should return a pointer, it returns `addr`. If it's void-typed, that's fine too.
    match call.get_type() {
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
    assert_eq!(dest.get_type(), Type::pointer_to(Type::i8()));
    assert_eq!(src.get_type(), Type::pointer_to(Type::i8()));

    let dest = hook_utils::memcpy(state, dest, src, num_bytes)?;

    // if the call should return a pointer, it returns `dest`. If it's void-typed, that's fine too.
    match call.get_type() {
        Type::VoidType => Ok(ReturnValue::ReturnVoid),
        Type::PointerType { .. } => Ok(ReturnValue::Return(dest)),
        ty => Err(Error::OtherError(format!(
            "Unexpected return type for a memcpy or memmove: {:?}",
            ty
        ))),
    }
}

pub fn symex_bswap<'p, B: Backend>(
    proj: &'p Project,
    state: &mut State<'p, B>,
    call: &'p dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 1);
    let arg = &call.get_arguments()[0].0;
    let argty = arg.get_type();
    let retty = call.get_type();
    if argty != retty {
        return Err(Error::OtherError(
            "Expected bswap argument to be the same type as its return type".to_owned(),
        ));
    }

    let arg = state.operand_to_bv(arg)?;
    match argty {
        Type::IntegerType { bits } => {
            assert_eq!(arg.get_width(), bits);
            Ok(ReturnValue::Return(bswap(&arg, bits)?))
        },
        Type::VectorType {
            element_type,
            num_elements,
        } => {
            let element_size = layout::size_opaque_aware(&element_type, proj).ok_or(Error::OtherError("llvm.bswap: argument is vector type, and vector element type contains a struct type with no definition in the Project".into()))?;
            let final_bv = unary_on_vector(&arg, num_elements as u32, |element| {
                bswap(element, element_size as u32)
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
    let width = layout::size(&call.get_type());
    let zero = state.zero(width as u32);
    let minusone = state.ones(width as u32);
    Ok(ReturnValue::Return(arg1.cond_bv(&zero, &minusone)))
}

pub fn symex_assume<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &'p dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 1);
    let arg = &call.get_arguments()[0].0;
    match arg.get_type() {
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

pub fn symex_sadd_with_overflow<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
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

pub fn symex_usub_with_overflow<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
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

pub fn symex_ssub_with_overflow<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
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

pub fn symex_umul_with_overflow<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
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

pub fn symex_smul_with_overflow<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
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

pub fn symex_uadd_sat<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
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

pub fn symex_sadd_sat<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
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

pub fn symex_usub_sat<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
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

pub fn symex_ssub_sat<'p, B: Backend>(
    _proj: &'p Project,
    state: &mut State<'p, B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
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
                args: vec![(arg0, vec![]), (arg1, vec![])],
            }
        }
    }

    impl Typed for DummyCall {
        fn get_type(&self) -> Type {
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

        let four = Operand::ConstantOperand(Constant::Int { bits: 8, value: 4 });
        let sixty_four = Operand::ConstantOperand(Constant::Int { bits: 8, value: 64 });
        let one_hundred = Operand::ConstantOperand(Constant::Int {
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
        let project = blank_project(
            "test_mod",
            blank_function("test_func", vec![Name::from("test_bb")]),
        );
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
        let project = blank_project(
            "test_mod",
            blank_function("test_func", vec![Name::from("test_bb")]),
        );
        let mut state = blank_state(&project, "test_func");

        // these are the examples from the LLVM 9 docs
        let one = Operand::ConstantOperand(Constant::Int { bits: 4, value: 1 });
        let two = Operand::ConstantOperand(Constant::Int { bits: 4, value: 2 });
        let five = Operand::ConstantOperand(Constant::Int { bits: 4, value: 5 });
        let six = Operand::ConstantOperand(Constant::Int { bits: 4, value: 6 });
        let minusfour = Operand::ConstantOperand(Constant::Int {
            bits: 4,
            value: (-4_i64) as u64,
        });
        let minusfive = Operand::ConstantOperand(Constant::Int {
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
}
