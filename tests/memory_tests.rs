use inkwell::module::Module;
use pitchfork_rs::*;
use std::path::Path;

fn get_module() -> Module {
    Module::parse_bitcode_from_path(&Path::new("c_examples/memory/memory.bc"))
        .expect("Failed to parse module")
}

#[test]
fn load_and_store() {
    let module = get_module();
    let func = module.get_function("load_and_store").expect("Failed to find function");
    let args = find_zero_of_func(func).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}

#[test]
fn local_ptr() {
    let module = get_module();
    let func = module.get_function("local_ptr").expect("Failed to find function");
    let args = find_zero_of_func(func).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn overwrite() {
    let module = get_module();
    let func = module.get_function("overwrite").expect("Failed to find function");
    let args = find_zero_of_func(func).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}

#[test]
fn load_and_store_mult() {
    let module = get_module();
    let func = module.get_function("load_and_store_mult").expect("Failed to find function");
    let args = find_zero_of_func(func).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}

#[test]
fn array() {
    let module = get_module();
    let func = module.get_function("array").expect("Failed to find function");
    let args = find_zero_of_func(func).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}

#[test]
fn pointer_arith() {
    let module = get_module();
    let func = module.get_function("pointer_arith").expect("Failed to find function");
    let args = find_zero_of_func(func).expect("Failed to find zero of function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}
