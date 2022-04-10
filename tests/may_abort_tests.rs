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
        &get_abort_project(),
        Config::default(),
        Some(vec![ParameterVal::Unconstrained]),
        None,
        3,
    );
    assert_eq!(
        rvals,
        PossibleSolutions::exactly_two(ReturnValue::Return(1), ReturnValue::Abort(None)),
    );
}

#[test]
fn may_panic() {
    let funcname = "panic::may_panic";
    init_logging();
    let rvals = get_possible_return_values_of_func(
        funcname,
        &get_panic_project(),
        Config::default(),
        Some(vec![ParameterVal::Unconstrained]),
        None,
        3,
    );
    assert_eq!(
        rvals,
        PossibleSolutions::exactly_two(ReturnValue::Return(1), ReturnValue::Abort(None)),
    );
}

#[test]
fn may_panic_debug() {
    let funcname = "panic::may_panic";
    init_logging();
    let rvals = get_possible_return_values_of_func(
        funcname,
        &get_panic_project(),
        Config::default(),
        Some(vec![ParameterVal::Unconstrained]),
        None,
        3,
    );
    
    let hs = match rvals {
	PossibleSolutions::Exactly(v) => v,
	PossibleSolutions::AtLeast(v) => v
    };
    let mut found = false;
    for rval in hs.iter() {
	match rval {
	    ReturnValue::Abort(Some(dbg)) => {
		// These are hardcoded. Unsure of a better way to do this
		assert!(dbg.line == 3);
		assert!(dbg.col == Some(9));
		assert!(dbg.filename == "panic.rs");
		found = true;
	    },
	    _ => {}
	}
    }
    if !found {
	panic!("Did not find debug info. Make sure that panic.bc was compiled with debug info (rustc -C debuginfo=1)");
    }
}
