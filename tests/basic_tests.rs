use haybale::solver_utils::PossibleSolutions;
use haybale::*;
use std::num::Wrapping;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_project() -> Project {
    let modname = "tests/bcfiles/basic.bc";
    Project::from_bc_path(modname)
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

fn get_issue_4_project() -> Project {
    let modname = "tests/bcfiles/issue_4.bc";
    Project::from_bc_path(modname)
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

fn get_issue_4_32bit_project() -> Project {
    let modname = "tests/bcfiles/32bit/issue_4.bc";
    Project::from_bc_path(modname)
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

fn get_issue_9_project() -> Project {
    let modname = "tests/bcfiles/issue_9.bc";
    Project::from_bc_path(modname)
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

fn get_issue_10_project() -> Project {
    let modname = "tests/bcfiles/issue_10.bc";
    Project::from_bc_path(modname)
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

#[test]
fn no_args_nozero() {
    let funcname = "no_args_nozero";
    init_logging();
    let proj = get_project();
    let args =
        find_zero_of_func(funcname, &proj, Config::default()).unwrap_or_else(|r| panic!("{}", r));
    assert_eq!(args, None);
}

#[test]
fn no_args_zero() {
    let funcname = "no_args_zero";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default())
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 0);
}

#[test]
fn one_arg() {
    let funcname = "one_arg";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default())
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    let sum: i32 = args.iter().map(|a| a.unwrap_to_i32()).sum();
    assert_eq!(sum, 3);
}

#[test]
fn two_args() {
    let funcname = "two_args";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default())
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let sum: i32 = args.iter().map(|a| a.unwrap_to_i32()).sum();
    assert_eq!(sum, 3);
}

#[test]
fn three_args() {
    let funcname = "three_args";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default())
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 3);
    let sum: i32 = args.iter().map(|a| a.unwrap_to_i32()).sum();
    assert_eq!(sum, 3);
}

#[test]
fn four_args() {
    let funcname = "four_args";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default())
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 4);
    let sum: i32 = args.iter().map(|a| a.unwrap_to_i32()).sum();
    assert_eq!(sum, 3);
}

#[test]
fn five_args() {
    let funcname = "five_args";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default())
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 5);
    let sum: i32 = args.iter().map(|a| a.unwrap_to_i32()).sum();
    assert_eq!(sum, 3);
}

#[test]
fn binops() {
    let funcname = "binops";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default())
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let a = Wrapping(args[0].unwrap_to_i32());
    let b = Wrapping(args[1].unwrap_to_i32());
    println!("a = {}, b = {}", a, b);
    let c = a + b - (Wrapping(77) * a) + Wrapping(1);
    let d = (c & Wrapping(23)) / (a | Wrapping(99));
    let e = (d ^ a) % (c << 3);
    assert_eq!((e >> (d.0 as usize)).0, 0);
}

#[test]
fn conditional_true() {
    let funcname = "conditional_true";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default())
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let a = Wrapping(args[0].unwrap_to_i32());
    let b = Wrapping(args[1].unwrap_to_i32());
    println!("a = {}, b = {}", a, b);
    let c = if a > b {
        (a - Wrapping(1)) * (b - Wrapping(1))
    } else {
        (a + b) % Wrapping(3) + Wrapping(10)
    };
    assert_eq!(c.0, 0);
}

#[test]
fn conditional_false() {
    let funcname = "conditional_false";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default())
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let a = Wrapping(args[0].unwrap_to_i32());
    let b = Wrapping(args[1].unwrap_to_i32());
    println!("a = {}, b = {}", a, b);
    let c = if a > b {
        (a + b) % Wrapping(3) + Wrapping(10)
    } else {
        (a - Wrapping(1)) * (b - Wrapping(1))
    };
    assert_eq!(c.0, 0);
}

#[test]
fn conditional_nozero() {
    let funcname = "conditional_nozero";
    init_logging();
    let proj = get_project();
    let args =
        find_zero_of_func(funcname, &proj, Config::default()).unwrap_or_else(|r| panic!("{}", r));
    assert_eq!(args, None);
}

#[test]
fn conditional_with_and() {
    let funcname = "conditional_with_and";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default())
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let a = args[0].unwrap_to_i32();
    let b = args[1].unwrap_to_i32();
    println!("a = {}, b = {}", a, b);
    assert!(a > 3);
    assert!(b > 4);
}

#[test]
fn switch() {
    let funcname = "has_switch";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default())
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let a = args[0].unwrap_to_i32();
    let b = args[1].unwrap_to_i32();
    println!("a = {}, b = {}", a, b);
    assert_eq!(a, 3);
    assert_eq!(b, 1);
}

#[test]
fn int8t() {
    let funcname = "int8t";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default())
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let sum: i8 = args.iter().map(|a| a.unwrap_to_i8()).sum();
    assert_eq!(sum, 3);
}

#[test]
fn int16t() {
    let funcname = "int16t";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default())
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let sum: i16 = args.iter().map(|a| a.unwrap_to_i16()).sum();
    assert_eq!(sum, 3);
}

#[test]
fn int32t() {
    let funcname = "int32t";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default())
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let sum: i32 = args.iter().map(|a| a.unwrap_to_i32()).sum();
    assert_eq!(sum, 3);
}

#[test]
fn int64t() {
    let funcname = "int64t";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default())
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let sum: i64 = args.iter().map(|a| a.unwrap_to_i64()).sum();
    assert_eq!(sum, 3);
}

#[test]
fn mixed_bitwidths() {
    let funcname = "mixed_bitwidths";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default())
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 4);
    let arg1 = args[0].unwrap_to_i8();
    let arg2 = args[1].unwrap_to_i16();
    let arg3 = args[2].unwrap_to_i32();
    let arg4 = args[3].unwrap_to_i64();
    let sum: i64 = i64::from(i32::from(arg1) + i32::from(arg2) + arg3) + arg4;
    assert_eq!(sum, 3);
}

#[test]
fn issue_4() {
    let funcname = "issue_4::ez";
    init_logging();
    let proj = get_issue_4_project();
    let ret = get_possible_return_values_of_func(
        funcname,
        vec![Some(1)],
        &proj,
        Config::default(),
        None,
        10,
    );
    assert_eq!(
        ret,
        PossibleSolutions::exactly_two(ReturnValue::Return(2), ReturnValue::Abort)
    );
}

#[test]
fn issue_4_32bit() {
    let funcname = "issue_4::ez";
    init_logging();
    let proj = get_issue_4_32bit_project();
    let ret = get_possible_return_values_of_func(
        funcname,
        vec![Some(1)],
        &proj,
        Config::default(),
        None,
        10,
    );
    assert_eq!(
        ret,
        PossibleSolutions::exactly_two(ReturnValue::Return(2), ReturnValue::Abort)
    );
}

#[test]
fn issue_9() {
    let funcname = "issue_9::Foo::ez3";
    init_logging();
    let proj = get_issue_9_project();
    let ret = get_possible_return_values_of_func(
        funcname,
        vec![Some(1)],
        &proj,
        Config::default(),
        None,
        10,
    );
    assert_eq!(
        ret,
        PossibleSolutions::exactly_two(ReturnValue::Return(1), ReturnValue::Abort)
    );
}

#[test]
fn issue_10() {
    let funcname = "issue_10::panic_if_not_zero";
    init_logging();
    let proj = get_issue_10_project();
    let ret = get_possible_return_values_of_func(
        funcname,
        vec![None],
        &proj,
        Config::default(),
        None,
        10,
    );
    assert_eq!(
        ret,
        PossibleSolutions::exactly_two(ReturnValue::ReturnVoid, ReturnValue::Abort)
    );
}
