use haybale::*;
use std::num::Wrapping;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_project() -> Project {
    let modname = "tests/bcfiles/call.bc";
    Project::from_bc_path(modname)
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

#[test]
fn simple_call() {
    let funcname = "simple_caller";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn cross_module_simple_call() {
    let callee_modname = "tests/bcfiles/call.bc";
    let caller_modname = "tests/bcfiles/crossmod.bc";
    let funcname = "cross_module_simple_caller";
    init_logging();
    let proj = Project::from_bc_paths(&[callee_modname, caller_modname])
        .unwrap_or_else(|e| panic!("Failed to parse modules: {}", e));
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn conditional_call() {
    let funcname = "conditional_caller";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[0], SolutionValue::I32(3));
    assert!(args[1].unwrap_to_i32() > 5);
}

#[test]
fn call_twice() {
    let funcname = "twice_caller";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn cross_module_call_twice() {
    let callee_modname = "tests/bcfiles/call.bc";
    let caller_modname = "tests/bcfiles/crossmod.bc";
    let funcname = "cross_module_twice_caller";
    init_logging();
    let proj = Project::from_bc_paths(&[callee_modname, caller_modname])
        .unwrap_or_else(|e| panic!("Failed to parse modules: {}", e));
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn nested_call() {
    let funcname = "nested_caller";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let x = Wrapping(args[0].unwrap_to_i32());
    let y = Wrapping(args[1].unwrap_to_i32());
    println!("x = {}, y = {}", x, y);
    assert_eq!((x + y).0, 3);
}

#[test]
fn cross_module_nested_near_call() {
    let callee_modname = "tests/bcfiles/call.bc";
    let caller_modname = "tests/bcfiles/crossmod.bc";
    let funcname = "cross_module_nested_near_caller";
    init_logging();
    let proj = Project::from_bc_paths(&[callee_modname, caller_modname])
        .unwrap_or_else(|e| panic!("Failed to parse modules: {}", e));
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let x = Wrapping(args[0].unwrap_to_i32());
    let y = Wrapping(args[1].unwrap_to_i32());
    println!("x = {}, y = {}", x, y);
    assert_eq!((x + y).0, 3);
}

#[test]
fn cross_module_nested_far_call() {
    let callee_modname = "tests/bcfiles/call.bc";
    let caller_modname = "tests/bcfiles/crossmod.bc";
    let funcname = "cross_module_nested_far_caller";
    init_logging();
    let proj = Project::from_bc_paths(&[callee_modname, caller_modname])
        .unwrap_or_else(|e| panic!("Failed to parse modules: {}", e));
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let x = Wrapping(args[0].unwrap_to_i32());
    let y = Wrapping(args[1].unwrap_to_i32());
    println!("x = {}, y = {}", x, y);
    assert_eq!((x + y).0, 3);
}

#[test]
fn call_of_loop() {
    let funcname = "caller_of_loop";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn call_in_loop() {
    let funcname = "caller_with_loop";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn recursive_simple() {
    let funcname = "recursive_simple";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    let x = Wrapping(args[0].unwrap_to_i32());
    println!("x = {}", x.0);
    assert_eq!(recursive_simple_dummy(x).0, 0);
    assert_eq!(args[0], SolutionValue::I32(11));
}

fn recursive_simple_dummy(x: Wrapping<i32>) -> Wrapping<i32> {
    let y = x * Wrapping(2);
    if y > Wrapping(25) {
        y
    } else {
        recursive_simple_dummy(y) - Wrapping(44)
    }
}

#[test]
fn recursive_double() {
    let funcname = "recursive_double";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(-6));
}

#[test]
fn recursive_not_tail() {
    let funcname = "recursive_not_tail";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
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
    let funcname = "recursive_and_normal_caller";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(11));
}

#[test]
fn mutually_recursive_functions() {
    let funcname = "mutually_recursive_a";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    //assert_eq!(args[0], SolutionValue::I32(3))
}
