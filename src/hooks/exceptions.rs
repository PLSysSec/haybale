//! Default hooks for C++ exception-related functions

use crate::alloc_utils;
use crate::backend::{Backend, BV};
use crate::error::*;
use crate::function_hooks::IsCall;
use crate::return_value::*;
use crate::state::State;
use llvm_ir::*;

pub fn cxa_allocate_exception<B: Backend>(
    state: &mut State<B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 1);
    let bytes = &call.get_arguments()[0].0;

    // sanity-check argument types
    match state.type_of(bytes).as_ref() {
        Type::IntegerType { .. } => {},
        ty => {
            return Err(Error::OtherError(format!(
                "__cxa_allocate_exception: expected argument to have integer type, but got {:?}",
                ty
            )))
        },
    };
    match state.type_of(call).as_ref() {
        Type::PointerType { .. } => {},
        ty => {
            return Err(Error::OtherError(format!(
                "__cxa_allocate_exception: expected return type to be a pointer type, but got {:?}",
                ty
            )))
        },
    };

    let addr = alloc_utils::zalloc(state, bytes)?;
    Ok(ReturnValue::Return(addr))
}

pub fn cxa_throw<B: Backend>(
    state: &mut State<B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 3);
    let thrown_ptr = &call.get_arguments()[0].0;
    let type_info = &call.get_arguments()[1].0;

    // sanity-check argument types
    match state.type_of(thrown_ptr).as_ref() {
        Type::PointerType { .. } => {},
        ty => {
            return Err(Error::OtherError(format!(
                "__cxa_throw: expected first argument to be some pointer type, got {:?}",
                ty
            )))
        },
    }
    match state.type_of(type_info).as_ref() {
        Type::PointerType { .. } => {},
        ty => {
            return Err(Error::OtherError(format!(
                "__cxa_throw: expected second argument to be some pointer type, got {:?}",
                ty
            )))
        },
    }

    let thrown_ptr = state.operand_to_bv(thrown_ptr)?;
    Ok(ReturnValue::Throw(thrown_ptr))
}

pub fn cxa_begin_catch<B: Backend>(
    state: &mut State<B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 1);

    // Since we're not doing anything fancy with exception frames (our exceptions are just pointers to the thrown values),
    // our __cxa_begin_catch() just returns its argument directly.
    // See [this section of the LLVM docs on exception handling](https://releases.llvm.org/9.0.0/docs/ExceptionHandling.html#try-catch).
    let arg = &call.get_arguments()[0].0;
    let arg = state.operand_to_bv(arg)?;
    Ok(ReturnValue::Return(arg))
}

pub fn cxa_end_catch<B: Backend>(
    _state: &mut State<B>,
    _call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    // Since we don't free things anyway, we don't need to worry about freeing the exception, and
    // __cxa_end_catch() can be a no-op
    Ok(ReturnValue::ReturnVoid)
}

pub fn llvm_eh_typeid_for<B: Backend>(
    state: &mut State<B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 1);

    // for now we ignore the argument and return an unconstrained value
    // (unconstrained except for the constraint that the value is positive, as specified in LLVM docs)
    let retval = state.new_bv_with_name(Name::from("llvm_eh_typeid_for_retval"), 32)?;
    retval.sgte(&state.zero(32)).assert()?;
    Ok(ReturnValue::Return(retval))
}
