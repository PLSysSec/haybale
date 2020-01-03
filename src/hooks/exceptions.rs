//! Default hooks for C++ exception-related functions

use crate::alloc_utils;
use crate::backend::Backend;
use crate::error::*;
use crate::function_hooks::IsCall;
use crate::project::Project;
use crate::return_value::*;
use crate::state::State;
use llvm_ir::*;

pub fn cxa_allocate_exception_hook<B: Backend>(
    _proj: &Project,
    state: &mut State<B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 1);
    let bytes = &call.get_arguments()[0].0;

    // sanity-check argument types
    match bytes.get_type() {
        Type::IntegerType { .. } => {},
        ty => return Err(Error::OtherError(format!("cxa_allocate_exception_hook: expected argument to have integer type, but got {:?}", ty))),
    };
    match call.get_type() {
        Type::PointerType { .. } => {},
        ty => return Err(Error::OtherError(format!("cxa_allocate_exception_hook: expected return type to be a pointer type, but got {:?}", ty))),
    };

    let addr = alloc_utils::zalloc(state, bytes)?;
    Ok(ReturnValue::Return(addr))
}

pub fn cxa_throw_hook<B: Backend>(
    _proj: &Project,
    state: &mut State<B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 3);
    let thrown_ptr = &call.get_arguments()[0].0;
    let type_info = &call.get_arguments()[1].0;

    // sanity-check argument types
    match thrown_ptr.get_type() {
        Type::PointerType { .. } => {},
        ty => return Err(Error::OtherError(format!("__cxa_throw: expected first argument to be some pointer type, got {:?}", ty))),
    }
    match type_info.get_type() {
        Type::PointerType { .. } => {},
        ty => return Err(Error::OtherError(format!("__cxa_throw: expected second argument to be some pointer type, got {:?}", ty))),
    }

    let thrown_ptr = state.operand_to_bv(thrown_ptr)?;
    Ok(ReturnValue::Throw(thrown_ptr))
}
