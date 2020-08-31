use haybale::solver_utils::PossibleSolutions;
use haybale::*;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_project() -> Project {
    let modname = "tests/bcfiles/globals.bc";
    Project::from_bc_path(modname)
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

fn get_cross_module_project() -> Project {
    Project::from_bc_paths(&["tests/bcfiles/globals.bc", "tests/bcfiles/crossmod.bc"])
        .unwrap_or_else(|e| panic!("Failed to parse modules: {}", e))
}

#[test]
fn read_global() {
    let funcname = "read_global";
    init_logging();
    let proj = get_project();
    assert_eq!(
        get_possible_return_values_of_func(
            funcname,
            std::iter::empty(),
            &proj,
            Config::default(),
            None,
            5
        ),
        PossibleSolutions::exactly_one(ReturnValue::Return(3)),
    );
}

#[test]
fn modify_global() {
    let funcname = "modify_global";
    init_logging();
    let proj = get_project();
    assert_eq!(
        get_possible_return_values_of_func(
            funcname,
            std::iter::once(Some(3)),
            &proj,
            Config::default(),
            None,
            5
        ),
        PossibleSolutions::exactly_one(ReturnValue::Return(3)),
    )
}

#[test]
fn modify_global_with_call() {
    let funcname = "modify_global_with_call";
    init_logging();
    let proj = get_project();
    assert_eq!(
        get_possible_return_values_of_func(
            funcname,
            std::iter::once(Some(3)),
            &proj,
            Config::default(),
            None,
            5
        ),
        PossibleSolutions::exactly_one(ReturnValue::Return(3)),
    )
}

#[test]
fn dont_confuse_globals() {
    let funcname = "dont_confuse_globals";
    init_logging();
    let proj = get_project();
    assert_eq!(
        get_possible_return_values_of_func(
            funcname,
            std::iter::once(Some(3)),
            &proj,
            Config::default(),
            None,
            5
        ),
        PossibleSolutions::exactly_one(ReturnValue::Return(3)),
    )
}

// The following tests essentially assume that the simple cross-module call tests are passing

#[test]
fn cross_module_read_global() {
    let funcname = "cross_module_read_global";
    init_logging();
    let proj = get_cross_module_project();
    assert_eq!(
        get_possible_return_values_of_func(
            funcname,
            std::iter::empty(),
            &proj,
            Config::default(),
            None,
            5
        ),
        PossibleSolutions::exactly_one(ReturnValue::Return(3)),
    );
}

#[test]
fn cross_module_read_global_via_call() {
    let funcname = "cross_module_read_global_via_call";
    init_logging();
    let proj = get_cross_module_project();
    assert_eq!(
        get_possible_return_values_of_func(
            funcname,
            std::iter::empty(),
            &proj,
            Config::default(),
            None,
            5
        ),
        PossibleSolutions::exactly_one(ReturnValue::Return(3)),
    );
}

#[test]
fn cross_module_modify_global() {
    let funcname = "cross_module_modify_global";
    init_logging();
    let proj = get_cross_module_project();
    assert_eq!(
        get_possible_return_values_of_func(
            funcname,
            std::iter::once(Some(3)),
            &proj,
            Config::default(),
            None,
            5
        ),
        PossibleSolutions::exactly_one(ReturnValue::Return(3)),
    );
}

#[test]
fn cross_module_modify_global_via_call() {
    let funcname = "cross_module_modify_global_via_call";
    init_logging();
    let proj = get_cross_module_project();
    assert_eq!(
        get_possible_return_values_of_func(
            funcname,
            std::iter::once(Some(3)),
            &proj,
            Config::default(),
            None,
            5
        ),
        PossibleSolutions::exactly_one(ReturnValue::Return(3)),
    );
}

#[test]
fn globals_initialization() {
    let modnames = &[
        "tests/bcfiles/globals_initialization_1.bc",
        "tests/bcfiles/globals_initialization_2.bc",
    ];
    let funcname = "foo";
    init_logging();
    let proj = Project::from_bc_paths(modnames)
        .unwrap_or_else(|e| panic!("Failed to create project: {}", e));
    assert_eq!(
        get_possible_return_values_of_func(
            funcname,
            std::iter::empty(),
            &proj,
            Config::default(),
            None,
            5
        ),
        PossibleSolutions::exactly_one(ReturnValue::Return(1052)),
    )
}
