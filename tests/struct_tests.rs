use haybale::*;
use std::path::Path;
use std::num::Wrapping;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_project() -> Project {
    let modname = "tests/bcfiles/struct.bc";
    Project::from_bc_path(&Path::new(modname))
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

#[test]
fn one_int() {
    let funcname = "one_int";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn two_ints_first() {
    let funcname = "two_ints_first";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn two_ints_second() {
    let funcname = "two_ints_second";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn two_ints_both() {
    let funcname = "two_ints_both";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
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
    let funcname = "three_ints";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
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
    let funcname = "zero_initialize";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
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
fn nonzero_initialize() {
    let funcname = "nonzero_initialize";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(103));
}

#[test]
fn mismatched_first() {
    let funcname = "mismatched_first";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I8(3));
}

#[test]
fn mismatched_second() {
    let funcname = "mismatched_second";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn mismatched_third() {
    let funcname = "mismatched_third";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I8(3));
}

#[test]
fn mismatched_all() {
    let funcname = "mismatched_all";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
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
    let funcname = "nested_first";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn nested_second() {
    let funcname = "nested_second";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn nested_all() {
    let funcname = "nested_all";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
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
    let funcname = "with_array";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn with_array_all() {
    let funcname = "with_array_all";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
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
    let funcname = "structptr";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    let x = Wrapping(args[0].unwrap_to_i32());
    println!("x = {}", x);
    let tiel2 = x - Wrapping(6);
    let tiel1 = tiel2 + x;
    let _tiel2 = 100;
    let result = tiel1;
    assert_eq!(result.0, 0);
}

#[test]
fn structelptr() {
    let funcname = "structelptr";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn changeptr() {
    let funcname = "changeptr";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    let x = Wrapping(args[0].unwrap_to_i32());
    println!("x = {}", x);
    let _ti1el2 = Wrapping(7);
    let ti2el2 = x - Wrapping(3) - Wrapping(0);
    let _ti1el2 = Wrapping(100);
    let result = ti2el2;
    assert_eq!(result.0, 0);
}
