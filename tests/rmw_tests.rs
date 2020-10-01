#![cfg(not(feature = "llvm-9"))] // With LLVM 9 and earlier, Haybale doesn't support AtomicRMW

use haybale::solver_utils::PossibleSolutions;
use haybale::*;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_project() -> Project {
    let modname = "tests/bcfiles/atomicrmw.bc";
    Project::from_bc_path(modname)
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

#[test]
fn atomicrmw() {
    init_logging();
    let proj = get_project();
    let funcname: String = "atomicrmwops".into();
    let ret = get_possible_return_values_of_func(
        &funcname,
        vec![Some(0xFF00), Some(0x00FF)],
        &proj,
        Config::default(),
        None,
        10,
    );
    assert_eq!(
        ret,
        PossibleSolutions::exactly_one(ReturnValue::Return(0xFF00))
    );
}
