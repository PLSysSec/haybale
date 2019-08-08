use llvm_ir::*;
use haybale::*;
use std::path::Path;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_module() -> Module {
    Module::from_bc_path(&Path::new("tests/bcfiles/memory.bc"))
        .expect("Failed to parse module")
}

#[test]
fn load_and_store() {
    init_logging();
    let ctx = z3::Context::new(&z3::Config::new());
    let module = get_module();
    let func = module.get_func_by_name("load_and_store").expect("Failed to find function");
    let args = find_zero_of_func(&ctx, func, &module, &Config::default()).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}

#[test]
fn local_ptr() {
    init_logging();
    let ctx = z3::Context::new(&z3::Config::new());
    let module = get_module();
    let func = module.get_func_by_name("local_ptr").expect("Failed to find function");
    let args = find_zero_of_func(&ctx, func, &module, &Config::default()).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn overwrite() {
    init_logging();
    let ctx = z3::Context::new(&z3::Config::new());
    let module = get_module();
    let func = module.get_func_by_name("overwrite").expect("Failed to find function");
    let args = find_zero_of_func(&ctx, func, &module, &Config::default()).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}

#[test]
fn load_and_store_mult() {
    init_logging();
    let ctx = z3::Context::new(&z3::Config::new());
    let module = get_module();
    let func = module.get_func_by_name("load_and_store_mult").expect("Failed to find function");
    let args = find_zero_of_func(&ctx, func, &module, &Config::default()).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}

#[test]
fn array() {
    init_logging();
    let ctx = z3::Context::new(&z3::Config::new());
    let module = get_module();
    let func = module.get_func_by_name("array").expect("Failed to find function");
    let args = find_zero_of_func(&ctx, func, &module, &Config::default()).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}

#[test]
fn pointer_arith() {
    init_logging();
    let ctx = z3::Context::new(&z3::Config::new());
    let module = get_module();
    let func = module.get_func_by_name("pointer_arith").expect("Failed to find function");
    let args = find_zero_of_func(&ctx, func, &module, &Config::default()).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}
