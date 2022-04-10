use haybale::solver_utils::PossibleSolutions;
use haybale::*;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_multi_panic_project() -> Project {
    let modname = "tests/bcfiles/multipanic.bc";
    Project::from_bc_path(modname)
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

fn get_multi_throw_project() -> Project {
    let modname = "tests/bcfiles/multithrow.bc";
    Project::from_bc_path(modname)
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

fn get_panic_project() -> Project {
    let modname = "tests/bcfiles/panic.bc";
    Project::from_bc_path(modname)
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

fn get_throw_project() -> Project {
    let modname = "tests/bcfiles/throwcatch.bc";
    Project::from_bc_path(modname)
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

#[test]
fn throw_debug_position() {
    let funcname = "throw_uncaught";
    init_logging();
    let rvals = get_possible_return_values_of_func(
        funcname,
        &get_throw_project(),
        Config::default(),
        None,
        Some(32),
        3,
    );
    
    let hs = match rvals {
	PossibleSolutions::Exactly(v) => v,
	PossibleSolutions::AtLeast(v) => v
    };
    let mut found = false;
    for rval in hs.iter() {
	match rval {
	    ReturnValue::Throw(_, Some(dbg)) => {
		// These are hardcoded. Unsure of a better way to do this
		assert!(dbg.line == 37);
		assert!(dbg.col == Some(9));
		assert!(dbg.filename == "throwcatch.cpp");
		found = true;
	    },
	    _ => {}
	}
    }
    if !found {
	panic!("Did not find debug info. Make sure that throwcatch.bc was compiled with -g");
    }
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

#[test]
fn multi_panic_debug() {
    let funcname = "multipanic::multipanic";
    init_logging();
    let rvals = get_possible_return_values_of_func(
        funcname,
        &get_multi_panic_project(),
        Config::default(),
        Some(vec![ParameterVal::Unconstrained]),
        None,
        3,
    );

    // There are two ways to test for equality here.
    // The first is through simple equality.
    // This will merge all Abort(_)'s into one -- it ignores debug info.
    
    assert_eq!(
        rvals,
        PossibleSolutions::exactly_two(ReturnValue::Return(1), ReturnValue::Abort(None)),
    );

    // The second is by checking the individial debug values.
    // This is probably not what you want to do, because two different aborts
    // behave the same, but don't look the same.

    let mut aborts = vec![];
    
    let hs = match rvals {
	PossibleSolutions::Exactly(v) => v,
	PossibleSolutions::AtLeast(v) => v
    };
    for rval in hs.iter() {
	match rval {
	    ReturnValue::Abort(Some(dbg)) => {
		aborts.push(dbg);
	    },
	    _ => {}
	}
    }
    if aborts.len() == 0 {
	panic!("Did not find debug info. Make sure that panic.bc was compiled with debug info (rustc -C debuginfo=1)");
    }
    if aborts.len() != 2 {
	panic!("Incorrect number of abort-possible paths");
    }
    // abort 1
    assert!(aborts.iter().any(|a| {
	a.line == 3
	    && a.col == Some(9)
	    && a.filename == "multipanic.rs"
    }));
    // abort 2
    assert!(aborts.iter().any(|a| {
	a.line == 5
	    && a.col == Some(9)
	    && a.filename == "multipanic.rs"
    }));
}

#[test]
fn multi_throw_debug() {
    let funcname = "multi_throw_debug";
    init_logging();
    let rvals = get_possible_return_values_of_func(
        funcname,
        &get_multi_throw_project(),
        Config::default(),
        None,
        Some(32),
        5,
    );

    // Similar to above, there are two ways to check this
    // First is below:
    
    assert_eq!(
        rvals,
        PossibleSolutions::Exactly(
            vec![
                ReturnValue::Return(1),
                ReturnValue::Return(2),
                ReturnValue::Throw(3, None),
                ReturnValue::Throw(4, None),
            ]
            .into_iter()
            .collect()
        )
    );

    // Second is the distinct option
    
    let mut aborts = vec![];
    
    let hs = match rvals {
	PossibleSolutions::Exactly(v) => v,
	PossibleSolutions::AtLeast(v) => v
    };
    for rval in hs.iter() {
	match rval {
	    ReturnValue::Throw(_, Some(dbg)) => {
		aborts.push(dbg);
	    },
	    _ => {}
	}
    }
    if aborts.len() == 0 {
	panic!("Did not find debug info. Make sure that multithrow.bc was compiled with debug info (rustc -C debuginfo=1)");
    }
    if aborts.len() != 3 {
	panic!("Incorrect number of abort-possible paths");
    }
    assert!(aborts.iter().any(|a| {
	a.line == 7
	    && a.col == Some(17)
	    && a.filename == "multithrow.cpp"
    }));
    assert!(aborts.iter().any(|a| {
	a.line == 8
	    && a.col == Some(17)
	    && a.filename == "multithrow.cpp"
    }));
    assert!(aborts.iter().any(|a| {
	a.line == 9
	    && a.col == Some(18)
	    && a.filename == "multithrow.cpp"
    }));
}
