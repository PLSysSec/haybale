//! Default hooks for malloc-related functions

use crate::alloc_utils;
use crate::backend::Backend;
use crate::error::*;
use crate::function_hooks::IsCall;
use crate::return_value::*;
use crate::state::State;
use llvm_ir::*;

pub fn malloc_hook<'p, B: Backend + 'p>(
    state: &mut State<'p, B>,
    call: &'p dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 1);
    let bytes = &call.get_arguments()[0].0;
    match state.type_of(bytes).as_ref() {
        Type::IntegerType { .. } => {},
        ty => {
            return Err(Error::OtherError(format!(
                "malloc_hook: expected argument to have integer type, but got {:?}",
                ty
            )))
        },
    };
    match state.type_of(call).as_ref() {
        Type::PointerType { .. } => {},
        ty => {
            return Err(Error::OtherError(format!(
                "malloc_hook: expected return type to be a pointer type, but got {:?}",
                ty
            )))
        },
    };

    let addr = alloc_utils::malloc(state, bytes)?;
    Ok(ReturnValue::Return(addr))
}

pub fn calloc_hook<'p, B: Backend + 'p>(
    state: &mut State<'p, B>,
    call: &'p dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let num = &call.get_arguments()[0].0;
    let size = &call.get_arguments()[1].0;
    match state.type_of(num).as_ref() {
        Type::IntegerType { .. } => {},
        ty => {
            return Err(Error::OtherError(format!(
                "calloc_hook: expected first argument to have integer type, but got {:?}",
                ty
            )))
        },
    };
    match state.type_of(size).as_ref() {
        Type::IntegerType { .. } => {},
        ty => {
            return Err(Error::OtherError(format!(
                "calloc_hook: expected second argument to have integer type, but got {:?}",
                ty
            )))
        },
    };
    match state.type_of(call).as_ref() {
        Type::PointerType { .. } => {},
        ty => {
            return Err(Error::OtherError(format!(
                "calloc_hook: expected return type to be a pointer type, but got {:?}",
                ty
            )))
        },
    };

    let addr = alloc_utils::calloc(state, num, size)?;
    Ok(ReturnValue::Return(addr))
}

pub fn free_hook<'p, B: Backend + 'p>(
    _state: &mut State<'p, B>,
    _call: &'p dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    // The simplest implementation of free() is a no-op.
    // Our allocator won't ever reuse allocated addresses anyway.
    Ok(ReturnValue::ReturnVoid)
}

pub fn realloc_hook<'p, B: Backend + 'p>(
    state: &mut State<'p, B>,
    call: &'p dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let addr = &call.get_arguments()[0].0;
    let new_size = &call.get_arguments()[1].0;
    match state.type_of(addr).as_ref() {
        Type::PointerType { .. } => {},
        ty => {
            return Err(Error::OtherError(format!(
                "realloc_hook: expected first argument to be a pointer type, but got {:?}",
                ty
            )))
        },
    };
    match state.type_of(new_size).as_ref() {
        Type::IntegerType { .. } => {},
        ty => {
            return Err(Error::OtherError(format!(
                "realloc_hook: expected second argument to be an integer type, but got {:?}",
                ty
            )))
        },
    };
    match state.type_of(call).as_ref() {
        Type::PointerType { .. } => {},
        ty => {
            return Err(Error::OtherError(format!(
                "realloc_hook: expected return type to be a pointer type, but got {:?}",
                ty
            )))
        },
    };

    let addr = alloc_utils::realloc(state, addr, new_size)?;
    Ok(ReturnValue::Return(addr))
}
