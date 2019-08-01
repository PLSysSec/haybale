use llvm_ir::Module;
use haybale::*;
use std::path::Path;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_module() -> Module {
    Module::from_bc_path(&Path::new("tests/bcfiles/linkedlist.bc"))
        .expect("Failed to parse module")
}

#[test]
fn simple_linked_list() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("simple_linked_list").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}


#[test]
fn indirectly_recursive_type() {
    init_logging();
    let module = get_module();
    let func = module.get_func_by_name("indirectly_recursive_type").expect("Failed to find function");
    let args = find_zero_of_func(func, &module, 20).expect("Failed to find zero of function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}
