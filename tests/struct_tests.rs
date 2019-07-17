use llvm_ir::*;
use pitchfork_rs::*;
use std::path::Path;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_module() -> Module {
    Module::from_bc_path(&Path::new("c_examples/struct/struct.bc"))
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
    let x = args[0].unwrap_to_i32();
    let _tiel1 = x + 2;
    let tiel2 = x + 3;
    let tiel1 = tiel2 - 10;
    let tiel2 = tiel1 + 7;
    assert_eq!(tiel2 - 3, 0);
}

#[test]
fn three_ints() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("three_ints").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    let x = args[0].unwrap_to_i32();
    let y = args[1].unwrap_to_i32();
    let tiel1 = x + y;
    let tiel2 = x - y;
    let tiel3 = tiel1 + tiel2;
    let _tiel2 = tiel3 - 2 * tiel1;
    let tiel1 = tiel3 - x;
    assert_eq!(tiel1 - 3, 0);
}

#[test]
fn mismatched() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("mismatched").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    let x = args[0].unwrap_to_i8();
    let y = args[1].unwrap_to_i32();
    let mmel1 = x + 3;
    let mmel2 = y - 3;
    let mmel3 = mmel1 - x;
    let _mmel1 = (mmel2 - i32::from(mmel3)) as i8;
    let mmel2 = mmel3 + 4;
    let mmel1 = mmel2 - x;
    let _mmel3 = mmel2 - 5;
    let mmel2 = i32::from(mmel1) + y;
    assert_eq!(mmel2 + 3 * i32::from(x), 0);
}

#[test]
fn nested() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("nested").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    let x = args[0].unwrap_to_i8();
    let y = args[1].unwrap_to_i32();
    let nmmel2 = 0;
    let _ntiel2 = y + 3;
    let nmmel1 = x - 4;
    let ntiel1 = nmmel2 + y;
    let nmmel3 = nmmel1 + 10;
    let _nmmel2 = nmmel3 + nmmel1;
    let ntiel2 = i32::from(nmmel3) + ntiel1;
    assert_eq!(ntiel2 - y, 0);
}

#[test]
fn with_array() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("with_array").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    let x = args[0].unwrap_to_i32();
    let waarr2 = x + 4;
    let waarr4 = -3;
    let _wammel2 = waarr2;
    let wamm2el2 = waarr2 + x;
    assert_eq!(waarr4 - wamm2el2, 0);
}

#[test]
fn structptr() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("structptr").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    let x = args[0].unwrap_to_i32();
    let mmel3 = 0;
    let mmel2 = x + 4;
    let mmel1 = mmel3 + x;
    assert_eq!(mmel2 + mmel1, 0);
}

#[test]
fn ptrs() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("ptrs").expect("Failed to find function");
    let args = find_zero_of_func(func, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    let x = args[0].unwrap_to_i32();
    let wa1mmel2 = 0;
    let wa2arr7 = 0;
    let wa1arr3 = x + 4;
    let wa2arr4 = x + 7;
    let wa2mm2el2 = wa1mmel2 + 3;
    let _wa1arr7 = wa2arr4 + wa1arr3;
    let wa2arr1 = wa2arr7 - wa2mm2el2;
    let wa1arr5 = wa1mmel2 + wa1arr3;
    let wa2mmel2 = wa2mm2el2 + 3;
    assert_eq!(wa2mmel2 + wa2arr1 + wa1arr5 + wa1arr5, 0);
}
