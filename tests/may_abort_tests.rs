use haybale::solver_utils::PossibleSolutions;
use haybale::*;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_abort_project() -> Project {
    let modname = "tests/bcfiles/abort.bc";
    Project::from_bc_path(modname)
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

fn get_panic_project() -> Project {
    let modname = "tests/bcfiles/panic.bc";
    Project::from_bc_path(modname)
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

#[test]
fn may_exit() {
    let funcname = "may_exit";
    init_logging();
    let rvals = get_possible_return_values_of_func(
        funcname,
        std::iter::once(None),
        &get_abort_project(),
        Config::default(),
        None,
        3,
    );
    assert_eq!(
        rvals,
        PossibleSolutions::exactly_two(ReturnValue::Return(1), ReturnValue::Abort),
    );
}

#[test]
fn may_panic() {
    let funcname = "panic::may_panic";
    init_logging();
    let rvals = get_possible_return_values_of_func(
        funcname,
        std::iter::once(None),
        &get_panic_project(),
        Config::default(),
        None,
        3,
    );
    assert_eq!(
        rvals,
        PossibleSolutions::exactly_two(ReturnValue::Return(1), ReturnValue::Abort),
    );
}
