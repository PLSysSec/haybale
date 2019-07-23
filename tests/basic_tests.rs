use llvm_ir::*;
use pitchfork_rs::*;
use std::num::Wrapping;
use std::path::Path;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_module() -> Module {
    Module::from_bc_path(&Path::new("tests/bcfiles/basic.bc"))
        .expect("Failed to parse module")
}

#[test]
fn no_args_nozero() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("no_args_nozero").expect("Failed to find function");
    assert_eq!(find_zero_of_func(func, 20), None);
}

#[test]
fn no_args_zero() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("no_args_zero").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 0);
}

#[test]
fn one_arg() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("one_arg").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    let sum: i32 = args.iter().map(|a| a.unwrap_to_i32()).sum();
    assert_eq!(sum, 3);
}

#[test]
fn two_args() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("two_args").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let sum: i32 = args.iter().map(|a| a.unwrap_to_i32()).sum();
    assert_eq!(sum, 3);
}

#[test]
fn three_args() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("three_args").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 3);
    let sum: i32 = args.iter().map(|a| a.unwrap_to_i32()).sum();
    assert_eq!(sum, 3);
}

#[test]
fn four_args() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("four_args").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 4);
    let sum: i32 = args.iter().map(|a| a.unwrap_to_i32()).sum();
    assert_eq!(sum, 3);
}

#[test]
fn five_args() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("five_args").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 5);
    let sum: i32 = args.iter().map(|a| a.unwrap_to_i32()).sum();
    assert_eq!(sum, 3);
}

#[test]
fn binops() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("binops").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of the function");
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
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("conditional_true").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let a = Wrapping(args[0].unwrap_to_i32());
    let b = Wrapping(args[1].unwrap_to_i32());
    println!("a = {}, b = {}", a, b);
    let c = if a > b { (a - Wrapping(1)) * (b - Wrapping(1)) } else { (a + b) % Wrapping(3) + Wrapping(10) };
    assert_eq!(c.0, 0);
}

#[test]
fn conditional_false() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("conditional_false").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let a = Wrapping(args[0].unwrap_to_i32());
    let b = Wrapping(args[1].unwrap_to_i32());
    println!("a = {}, b = {}", a, b);
    let c = if a > b { (a + b) % Wrapping(3) + Wrapping(10) } else { (a - Wrapping(1)) * (b - Wrapping(1)) };
    assert_eq!(c.0, 0);
}


#[test]
fn conditional_nozero() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("conditional_nozero").expect("Failed to find function");
    assert_eq!(find_zero_of_func(func, 20), None);
}

#[test]
fn conditional_with_and() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("conditional_with_and").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let a = args[0].unwrap_to_i32();
    let b = args[1].unwrap_to_i32();
    println!("a = {}, b = {}", a, b);
    assert!(a > 3);
    assert!(b > 4);
}

#[test]
fn int8t() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("int8t").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let sum: i8 = args.iter().map(|a| a.unwrap_to_i8()).sum();
    assert_eq!(sum, 3);
}

#[test]
fn int16t() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("int16t").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let sum: i16 = args.iter().map(|a| a.unwrap_to_i16()).sum();
    assert_eq!(sum, 3);
}

#[test]
fn int32t() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("int32t").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let sum: i32 = args.iter().map(|a| a.unwrap_to_i32()).sum();
    assert_eq!(sum, 3);
}

#[test]
fn int64t() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("int64t").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    let sum: i64 = args.iter().map(|a| a.unwrap_to_i64()).sum();
    assert_eq!(sum, 3);
}

#[test]
fn mixed_bitwidths() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("mixed_bitwidths").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 4);
    let arg1 = args[0].unwrap_to_i8();
    let arg2 = args[1].unwrap_to_i16();
    let arg3 = args[2].unwrap_to_i32();
    let arg4 = args[3].unwrap_to_i64();
    let sum: i64 = (arg1 as i32 + arg2 as i32 + arg3) as i64 + arg4;
    assert_eq!(sum, 3);
}
