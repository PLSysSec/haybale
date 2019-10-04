use haybale::*;
use haybale::solver_utils::PossibleSolutions;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::path::Path;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_project() -> Project {
    let modname = "tests/bcfiles/functionptr.bc";
    Project::from_bc_path(&Path::new(modname))
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

#[test]
fn call_through_function_ptr() {
    let funcname = "fptr_driver";
    init_logging();
    let proj = get_project();
    assert_eq!(
        get_possible_return_values_of_func(funcname, std::iter::empty(), &proj, Config::default(), 5),
        PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(22))),
    );
}

#[test]
fn call_through_function_ptr_struct() {
    let funcname = "struct_driver";
    let proj = get_project();
    assert_eq!(
        get_possible_return_values_of_func(funcname, std::iter::empty(), &proj, Config::default(), 5),
        PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(15))),
    );
}
