use llvm_ir::*;
use pitchfork_rs::*;
use std::num::Wrapping;
use std::path::Path;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_module() -> Module {
    Module::from_bc_path(&Path::new("tests/bcfiles/call.bc"))
        .expect("Failed to parse module")
}

#[test]
fn simple_call() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("simple_caller").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn conditional_call() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("conditional_caller").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[0], SolutionValue::I32(3));
    assert!(args[1].unwrap_to_i32() > 5);
}

#[test]
fn call_twice() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("twice_caller").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn nested_call() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("nested_caller").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    let x = Wrapping(args[0].unwrap_to_i32());
    let y = Wrapping(args[1].unwrap_to_i32());
    println!("x = {}, y = {}", x, y);
    assert_eq!((x + y).0, 3);
}

#[test]
fn call_of_loop() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("caller_of_loop").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn call_in_loop() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("caller_with_loop").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn recursive_simple() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("recursive_simple").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    let x = Wrapping(args[0].unwrap_to_i32());
    println!("x = {}", x.0);
    assert_eq!(recursive_simple_dummy(x).0, 0);
    assert_eq!(args[0], SolutionValue::I32(19));
}

fn recursive_simple_dummy(x: Wrapping<i32>) -> Wrapping<i32> {
    let y = x * Wrapping(2);
    if y > Wrapping(25) {
        y
    } else {
        recursive_simple_dummy(y) - Wrapping(38)
    }
}

#[test]
fn recursive_more_complicated() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("recursive_more_complicated").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(-15));
}

#[test]
fn recursive_not_tail() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("recursive_not_tail").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    let x = Wrapping(args[0].unwrap_to_i32());
    println!("x = {}", x.0);
    assert_eq!(recursive_not_tail_dummy(x).0, 0);
}

fn recursive_not_tail_dummy(x: Wrapping<i32>) -> Wrapping<i32> {
    if x > Wrapping(100) {
        x + Wrapping(10)
    } else {
        let a = recursive_not_tail_dummy(x + Wrapping(20));
        if a % Wrapping(2) == Wrapping(0) {
            a % Wrapping(3)
        } else {
            (a % Wrapping(5)) - Wrapping(8)
        }
    }
}

#[test]
fn recursive_and_normal_call() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("recursive_and_normal_caller").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(19));
}

#[test]
fn mutually_recursive_functions() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("mutually_recursive_a").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 5).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);

}
