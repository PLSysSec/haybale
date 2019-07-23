use llvm_ir::*;
use pitchfork_rs::*;
use std::path::Path;
use std::num::Wrapping;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_module() -> Module {
    Module::from_bc_path(&Path::new("tests/bcfiles/struct.bc"))
        .expect("Failed to parse module")
}

#[test]
fn one_int() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("one_int").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn two_ints_first() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("two_ints_first").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn two_ints_second() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("two_ints_second").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn two_ints_both() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("two_ints_both").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    let x = Wrapping(args[0].unwrap_to_i32());
    println!("x = {}", x);
    let _tiel1 = x + Wrapping(2);
    let tiel2 = x + Wrapping(3);
    let tiel1 = tiel2 - Wrapping(10);
    let tiel2 = tiel1 + Wrapping(7);
    let result = tiel2 - Wrapping(3);
    assert_eq!(result.0, 0);
}

#[test]
fn three_ints() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("three_ints").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    let x = Wrapping(args[0].unwrap_to_i32());
    let y = Wrapping(args[1].unwrap_to_i32());
    println!("x = {}, y = {}", x, y);
    let tiel1 = x + y;
    let tiel2 = x - y;
    let tiel3 = tiel1 + tiel2;
    let _tiel2 = tiel3 - Wrapping(2) * tiel1;
    let tiel1 = tiel3 - x;
    let result = tiel1 - Wrapping(3);
    assert_eq!(result.0, 0);
}

#[test]
fn zero_initialize() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("zero_initialize").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    let x = Wrapping(args[0].unwrap_to_i32());
    let a = Wrapping(2);
    let b = Wrapping(4);
    let c = Wrapping(6);
    let tiel2 = a + b + c;
    let result: Wrapping<i32> = x - tiel2;
    assert_eq!(result.0, 0);
}

#[test]
fn mismatched_first() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("mismatched_first").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I8(3));
}

#[test]
fn mismatched_second() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("mismatched_second").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn mismatched_third() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("mismatched_third").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I8(3));
}

#[test]
fn mismatched_all() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("mismatched_all").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    let x = Wrapping(args[0].unwrap_to_i8() as u8);
    let y = Wrapping(args[1].unwrap_to_i32());
    println!("x = {}, y = {}", x, y);
    let mmel1 = x + Wrapping(3);
    let mmel2 = y - Wrapping(3);
    let mmel3 = mmel1 - x;
    let mmel1 = mmel3 - x;
    let mmel2 = mmel2 + Wrapping(4);
    let mmel1 = mmel1 - x;
    let mmel3 = mmel3 - Wrapping(5);
    let mmel2 = mmel2 + y;
    println!("mmel1 = {}, mmel2 = {}, mmel3 = {}", mmel1, mmel2, mmel3);
    let result = Wrapping(i32::from(mmel1.0)) + mmel2 + Wrapping(i32::from(mmel3.0));
    assert_eq!(result.0, 0);
}

#[test]
fn nested_first() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("nested_first").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn nested_second() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("nested_second").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn nested_all() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("nested_all").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    let x = Wrapping(args[0].unwrap_to_i8() as u8);
    let y = Wrapping(args[1].unwrap_to_i32());
    println!("x = {}, y = {}", x, y);
    let nmmel2 = Wrapping(0);
    let _ntiel2 = y + Wrapping(3);
    let nmmel1 = x - Wrapping(4);
    let ntiel1 = nmmel2 + y;
    let nmmel3 = nmmel1 + Wrapping(10);
    let _nmmel2 = nmmel3 + nmmel1;
    let ntiel2 = Wrapping(i32::from(nmmel3.0)) + ntiel1;
    let result = ntiel2 - y;
    assert_eq!(result.0, 0);
}

#[test]
fn with_array() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("with_array").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn with_array_all() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("with_array_all").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    let x = Wrapping(args[0].unwrap_to_i32());
    println!("x = {}", x);
    let waarr2 = x - Wrapping(4);
    let waarr4 = Wrapping(-3);
    let _wammel2 = waarr2;
    let wamm2el2 = waarr2 + x + Wrapping(1);
    let result = waarr4 + wamm2el2;
    assert_eq!(result.0, 0);
}

#[test]
fn structptr() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("structptr").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    let x = Wrapping(args[0].unwrap_to_i32());
    println!("x = {}", x);
    let mmel3 = Wrapping(0);
    let mmel2 = x + Wrapping(4);
    let mmel1 = mmel3 + x;
    let result = mmel2 + mmel1;
    assert_eq!(result.0, 0);
}

#[test]
fn ptrs() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("ptrs").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    let x = Wrapping(args[0].unwrap_to_i32());
    println!("x = {}", x);
    let wa1mmel2 = Wrapping(0);
    let wa2arr7 = Wrapping(0);
    let wa1arr3 = x + Wrapping(4);
    let wa2arr4 = x + Wrapping(7);
    let wa2mm2el2 = wa1mmel2 + Wrapping(3);
    let _wa1arr7 = wa2arr4 + wa1arr3;
    let wa2arr1 = wa2arr7 - wa2mm2el2;
    let wa1arr5 = wa1mmel2 + wa1arr3;
    let wa2mmel2 = wa2mm2el2 + Wrapping(3);
    let result = wa2mmel2 + wa2arr1 + wa1arr5 + wa1arr5;
    assert_eq!(result.0, 0);
}
