use haybale::*;
use haybale::backend::{Backend, BV};
use llvm_ir::*;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::path::Path;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

// Hook call.c's "simple_callee" to just return 5 instead of executing its actual body
fn hook_for_simple_callee<'p, B: Backend>(state: &mut State<'p, B>, call: &'p instruction::Call) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.arguments.len(), 2);
    Ok(ReturnValue::Return(B::BV::from_u32(state.btor.clone(), 5, layout::size(&call.get_type()) as u32)))
}

#[test]
fn hook_a_function() {
    init_logging();
    let proj = Project::from_bc_path(&Path::new("tests/bcfiles/call.bc"))
        .unwrap_or_else(|e| panic!("Failed to parse module call.bc: {}", e));
    let mut config = Config::default();
    config.function_hooks.add("simple_callee", &hook_for_simple_callee);
    // with that hook, simple_caller should always return 5 regardless of the value of its argument
    assert_eq!(
        get_possible_return_values_of_func("simple_caller", std::iter::once(None), &proj, config, 3),
        PossibleSolutions::PossibleSolutions(HashSet::from_iter(std::iter::once(5))),
    );
}

// Hook functionptr.c's "get_function_ptr" to return a pointer to our hook "target_hook" instead of "foo" or "bar" like it normally does
fn hook_for_get_function_ptr<'p, B: Backend>(state: &mut State<'p, B>, call: &'p instruction::Call) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.arguments.len(), 1);
    state.get_pointer_to_function_hook("asdfjkl")
        .cloned()
        .ok_or_else(|| Error::OtherError("Failed to get a pointer to function hook".to_owned()))
        .map(ReturnValue::Return)
}

fn target_hook<'p, B: Backend>(state: &mut State<'p, B>, call: &'p instruction::Call) -> Result<ReturnValue<B::BV>> {
    assert_eq!(call.arguments.len(), 2);
    Ok(ReturnValue::Return(B::BV::from_u32(state.btor.clone(), 5, layout::size(&call.get_type()) as u32)))
}

#[test]
fn hook_a_function_ptr() {
    init_logging();
    let proj = Project::from_bc_path(&Path::new("tests/bcfiles/functionptr.bc"))
        .unwrap_or_else(|e| panic!("Failed to parse module functionptr.bc: {}", e));
    let mut config = Config::default();
    config.function_hooks.add("get_function_ptr", &hook_for_get_function_ptr);
    // With the current API, in order for `state.get_pointer_to_function_hook()`
    // to work inside `hook_for_get_function_ptr`, we need to register
    // `target_hook` as the hook for some function name, doesn't matter what
    config.function_hooks.add("asdfjkl", &target_hook);
    // with these hooks, now `get_function_ptr` should return a pointer to `target_hook` instead of `foo` like it normally does,
    // and therefore fptr_driver() should return 15 instead of 22
    assert_eq!(
        get_possible_return_values_of_func("fptr_driver", std::iter::empty(), &proj, config, 3),
        PossibleSolutions::PossibleSolutions(HashSet::from_iter(std::iter::once(15))),
    );
}
