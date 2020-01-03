use haybale::*;
use haybale::solver_utils::PossibleSolutions;
use std::path::Path;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_project() -> Project {
    let modname = "tests/bcfiles/throwcatch.bc";
    Project::from_bc_path(&Path::new(modname))
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

#[test]
fn doesnt_throw() {
    let funcname = "doesnt_throw";
    init_logging();
    let rvals = get_possible_return_values_of_func(
        funcname,
        std::iter::once(None),
        &get_project(),
        Config::default(),
        Some(32),
        3,
    );
    match rvals {
        PossibleSolutions::Exactly(hs) => {
            for rval in hs {
                match rval {
                    ReturnValue::Return(rval) => assert!(rval > 0),
                    ReturnValue::ReturnVoid => panic!("Function shouldn't return void"),
                    ReturnValue::Throw(throwval) => panic!("Function shouldn't throw, but it threw {:?}", throwval),
                }
            }
        },
        PossibleSolutions::AtLeast(hs) => panic!("Too many possible solutions: {:?}", hs),
    }
}

#[test]
fn throw_uncaught() {
    let funcname = "throw_uncaught";
    init_logging();
    let rvals = get_possible_return_values_of_func(
        funcname,
        std::iter::once(None),
        &get_project(),
        Config::default(),
        Some(32),
        3,
    );
    assert_eq!(rvals, PossibleSolutions::Exactly(vec![
        ReturnValue::Return(2),
        ReturnValue::Throw(20),
    ].into_iter().collect()));
}

#[test]
fn throw_multiple_values() {
    let funcname = "throw_multiple_values";
    init_logging();
    let rvals = get_possible_return_values_of_func(
        funcname,
        std::iter::once(None),
        &get_project(),
        Config::default(),
        Some(32),
        5,
    );
    assert_eq!(rvals, PossibleSolutions::Exactly(vec![
        ReturnValue::Return(1),
        ReturnValue::Return(2),
        ReturnValue::Throw(3),
        ReturnValue::Throw(4),
    ].into_iter().collect()));
}

#[test]
fn throw_uncaught_wrongtype() {
    let funcname = "throw_uncaught_wrongtype";
    init_logging();
    let rvals = get_possible_return_values_of_func(
        funcname,
        std::iter::once(None),
        &get_project(),
        Config::default(),
        Some(32),
        3,
    );
    assert_eq!(rvals, PossibleSolutions::Exactly(vec![
        ReturnValue::Return(2),
        ReturnValue::Throw(20),
        // TODO: This function shouldn't actually be able to Return(10), but
        // since our matching of catch blocks is currently imprecise, our
        // current symex allows the exception to be either caught or not-caught
        ReturnValue::Return(10),
    ].into_iter().collect()));
}

#[test]
fn throw_uncaught_caller() {
    let funcname = "throw_uncaught_caller";
    init_logging();
    let rvals = get_possible_return_values_of_func(
        funcname,
        std::iter::once(None),
        &get_project(),
        Config::default(),
        Some(32),
        3,
    );
    assert_eq!(rvals, PossibleSolutions::Exactly(vec![
        ReturnValue::Return(1),
        ReturnValue::Throw(20),
    ].into_iter().collect()));
}

#[test]
fn throw_and_catch_wildcard() {
    let funcname = "throw_and_catch_wildcard";
    init_logging();
    let rvals = get_possible_return_values_of_func(
        funcname,
        std::iter::once(None),
        &get_project(),
        Config::default(),
        Some(32),
        3,
    );
    assert_eq!(rvals, PossibleSolutions::Exactly(vec![
        ReturnValue::Return(2),
        ReturnValue::Return(5),
    ].into_iter().collect()));
}

#[test]
fn throw_and_catch_val() {
    let funcname = "throw_and_catch_val";
    init_logging();
    let rvals = get_possible_return_values_of_func(
        funcname,
        std::iter::once(None),
        &get_project(),
        Config::default(),
        Some(32),
        3,
    );
    assert_eq!(rvals, PossibleSolutions::Exactly(vec![
        ReturnValue::Return(2),
        ReturnValue::Return(20),
        // TODO: This function shouldn't actually be able to Throw(20), but
        // since our matching of catch blocks is currently imprecise, our
        // current symex allows the exception to be either caught or not-caught
        ReturnValue::Throw(20),
    ].into_iter().collect()));
}

#[test]
fn throw_and_catch_in_caller() {
    let funcname = "throw_and_catch_in_caller";
    init_logging();
    let rvals = get_possible_return_values_of_func(
        funcname,
        std::iter::once(None),
        &get_project(),
        Config::default(),
        Some(32),
        3,
    );
    assert_eq!(rvals, PossibleSolutions::Exactly(vec![
        ReturnValue::Return(2),
        ReturnValue::Return(20),
        // TODO: This function shouldn't actually be able to Throw(20), but
        // since our matching of catch blocks is currently imprecise, our
        // current symex allows the exception to be either caught or not-caught
        ReturnValue::Throw(20),
    ].into_iter().collect()));
}

#[test]
// TODO: We don't currently support __cxa_rethrow
#[should_panic(expected = "__cxa_rethrow")]
fn throw_and_rethrow_in_caller() {
    let funcname = "throw_and_rethrow_in_caller";
    init_logging();
    let rvals = get_possible_return_values_of_func(
        funcname,
        std::iter::once(None),
        &get_project(),
        Config::default(),
        Some(32),
        3,
    );
    assert_eq!(rvals, PossibleSolutions::Exactly(vec![
        ReturnValue::Return(2),
        ReturnValue::Throw(20),
    ].into_iter().collect()));
}
