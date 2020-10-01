use haybale::backend::Backend;
use haybale::function_hooks::IsCall;
use haybale::solver_utils::PossibleSolutions;
use haybale::*;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

// Hook call.c's "simple_callee" to just return 5 instead of executing its actual body
fn hook_for_simple_callee<'p, B: Backend>(
    state: &mut State<'p, B>,
    call: &'p dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let ret_size = state.size_in_bits(&state.type_of(call)).ok_or_else(|| {
        Error::OtherError("simple_callee shouldn't return opaque struct type".into())
    })?;
    assert_ne!(ret_size, 0);
    Ok(ReturnValue::Return(state.bv_from_u32(5, ret_size)))
}

#[test]
fn hook_a_function() {
    init_logging();
    let proj = Project::from_bc_path("tests/bcfiles/call.bc")
        .unwrap_or_else(|e| panic!("Failed to parse module call.bc: {}", e));
    let mut config = Config::default();
    config
        .function_hooks
        .add("simple_callee", &hook_for_simple_callee);
    // with that hook, simple_caller should always return 5 regardless of the value of its argument
    assert_eq!(
        get_possible_return_values_of_func("simple_caller", &proj, config, None, None, 3),
        PossibleSolutions::exactly_one(ReturnValue::Return(5)),
    );
}

// Hook functionptr.c's "get_function_ptr" to return a pointer to our hook "target_hook" instead of "foo" or "bar" like it normally does
fn hook_for_get_function_ptr<'p, B: Backend>(
    state: &mut State<'p, B>,
    call: &'p dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 1);
    state
        .get_pointer_to_function_hook("asdfjkl")
        .cloned()
        .ok_or_else(|| Error::OtherError("Failed to get a pointer to function hook".to_owned()))
        .map(ReturnValue::Return)
}

fn target_hook<'p, B: Backend>(
    state: &mut State<'p, B>,
    call: &'p dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.get_arguments().len(), 2);
    let ret_size = state.size_in_bits(&state.type_of(call)).ok_or_else(|| {
        Error::OtherError("target_hook: call return type shouldn't be opaque struct type".into())
    })?;
    assert_ne!(ret_size, 0);
    Ok(ReturnValue::Return(state.bv_from_u32(5, ret_size)))
}

#[test]
fn hook_a_function_ptr() {
    init_logging();
    let proj = Project::from_bc_path("tests/bcfiles/functionptr.bc")
        .unwrap_or_else(|e| panic!("Failed to parse module functionptr.bc: {}", e));
    let mut config = Config::default();
    config
        .function_hooks
        .add("get_function_ptr", &hook_for_get_function_ptr);
    // With the current API, in order for `state.get_pointer_to_function_hook()`
    // to work inside `hook_for_get_function_ptr`, we need to register
    // `target_hook` as the hook for some function name, doesn't matter what
    config.function_hooks.add("asdfjkl", &target_hook);
    // with these hooks, now `get_function_ptr` should return a pointer to `target_hook` instead of `foo` like it normally does,
    // and therefore fptr_driver() should return 15 instead of 22
    assert_eq!(
        get_possible_return_values_of_func("fptr_driver", &proj, config, None, None, 3),
        PossibleSolutions::exactly_one(ReturnValue::Return(15)),
    );
}
