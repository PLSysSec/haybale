use llvm_ir::*;
use pitchfork_rs::*;
use std::path::Path;

fn get_module() -> Module {
    Module::from_bc_path(&Path::new("c_examples/loop/loop.bc"))
        .expect("Failed to parse module")
}

#[test]
fn while_loop() {
    let module = get_module();
    let func = module.get_func_by_name("while_loop").expect("Failed to find function");
    let args = find_zero_of_func(func).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn for_loop() {
    let module = get_module();
    let func = module.get_func_by_name("for_loop").expect("Failed to find function");
    let args = find_zero_of_func(func).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn loop_zero_iterations() {
    let module = get_module();
    let func = module.get_func_by_name("loop_zero_iterations").expect("Failed to find function");
    let args = find_zero_of_func(func).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn loop_with_cond() {
    let module = get_module();
    let func = module.get_func_by_name("loop_with_cond").expect("Failed to find function");
    let args = find_zero_of_func(func).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(7));
}

#[test]
fn loop_inside_cond() {
    let module = get_module();
    let func = module.get_func_by_name("loop_inside_cond").expect("Failed to find function");
    let args = find_zero_of_func(func).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert!(args[0].unwrap_to_i32() > 7);
}

#[test]
fn loop_over_array() {
    let module = get_module();
    let func = module.get_func_by_name("loop_over_array").expect("Failed to find function");
    let args = find_zero_of_func(func).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn sum_of_array() {
    let module = get_module();
    let func = module.get_func_by_name("sum_of_array").expect("Failed to find function");
    let args = find_zero_of_func(func).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn search_array() {
    let module = get_module();
    let func = module.get_func_by_name("search_array").expect("Failed to find function");
    let args = find_zero_of_func(func).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn nested_loop() {
    let module = get_module();
    let func = module.get_func_by_name("nested_loop").expect("Failed to find function");
    let args = find_zero_of_func(func).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}
