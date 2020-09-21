use haybale::backend::DefaultBackend;
use haybale::config::NullPointerChecking;
use haybale::*;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_project() -> Project {
    let modname = "tests/bcfiles/memory.bc";
    Project::from_bc_path(modname)
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

#[test]
fn load_and_store() {
    let funcname = "load_and_store";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), Some(vec![ParameterVal::NonNullPointer, ParameterVal::Unconstrained]))
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}

#[test]
fn local_ptr() {
    let funcname = "local_ptr";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn overwrite() {
    let funcname = "overwrite";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), Some(vec![ParameterVal::NonNullPointer, ParameterVal::Unconstrained]))
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}

#[test]
fn load_and_store_mult() {
    let funcname = "load_and_store_mult";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), Some(vec![ParameterVal::NonNullPointer, ParameterVal::Unconstrained]))
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}

#[test]
fn array() {
    let funcname = "array";
    init_logging();
    let proj = get_project();
    let config_no_npc: Config<DefaultBackend> = {
        let mut config = Config::default();
        config.null_pointer_checking = NullPointerChecking::None; // otherwise this test fails, as ptr[10] could be NULL for the correct value of ptr
        config
    };
    let args = find_zero_of_func(funcname, &proj, config_no_npc, None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
    // alternately, it should also work to allocate the parameter and still use null-pointer checking
    let args = find_zero_of_func(funcname, &proj, Config::default(), Some(vec![ParameterVal::PointerToAllocated(54 * 4), ParameterVal::Unconstrained]))
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}

#[test]
fn pointer_arith() {
    let funcname = "pointer_arith";
    init_logging();
    let proj = get_project();
    let config_no_npc: Config<DefaultBackend> = {
        let mut config = Config::default();
        config.null_pointer_checking = NullPointerChecking::None; // otherwise this test fails, as e.g. ptr[2] or ptr[5] or something could be NULL, for the correct value of ptr
        config
    };
    let args = find_zero_of_func(funcname, &proj, config_no_npc, None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
    // alternately, it should also work to allocate the parameter and still use null-pointer checking
    let args = find_zero_of_func(funcname, &proj, Config::default(), Some(vec![ParameterVal::PointerToAllocated(54 * 4), ParameterVal::Unconstrained]))
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 2);
    assert_eq!(args[1], SolutionValue::I32(3));
}

#[test]
fn pointer_compare() {
    let funcname = "pointer_compare";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}
