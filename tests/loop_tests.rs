use llvm_ir::*;
use haybale::*;
use std::path::Path;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_module() -> Module {
    Module::from_bc_path(&Path::new("tests/bcfiles/loop.bc"))
        .expect("Failed to parse module")
}

#[test]
fn while_loop() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("while_loop").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn for_loop() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("for_loop").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn loop_zero_iterations() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("loop_zero_iterations").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(0));
}

#[test]
fn loop_with_cond() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("loop_with_cond").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(7));
}

#[test]
fn loop_inside_cond() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("loop_inside_cond").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert!(args[0].unwrap_to_i32() > 7);
}

#[test]
fn loop_over_array() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("loop_over_array").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn sum_of_array() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("sum_of_array").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn search_array() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("search_array").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(4));
}

#[test]
fn nested_loop() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("nested_loop").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 50).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}
