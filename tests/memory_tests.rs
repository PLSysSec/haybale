use haybale::*;
use std::path::Path;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_project() -> Project {
    let modname = "tests/bcfiles/memory.bc";
    Project::from_bc_path(&Path::new(modname))
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

#[test]
fn load_and_store() {
    let funcname = "load_and_store";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}

#[test]
fn local_ptr() {
    let funcname = "local_ptr";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn overwrite() {
    let funcname = "overwrite";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}

#[test]
fn load_and_store_mult() {
    let funcname = "load_and_store_mult";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}

#[test]
fn array() {
    let funcname = "array";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}

#[test]
fn pointer_arith() {
    let funcname = "pointer_arith";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}

#[test]
fn pointer_compare() {
    let funcname = "pointer_compare";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}
