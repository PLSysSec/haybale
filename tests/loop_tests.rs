use haybale::*;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_project() -> Project {
    let modname = "tests/bcfiles/loop.bc";
    Project::from_bc_path(modname)
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

#[test]
fn while_loop() {
    let funcname = "while_loop";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn for_loop() {
    let funcname = "for_loop";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn loop_zero_iterations() {
    let funcname = "loop_zero_iterations";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(0));
}

#[test]
fn loop_with_cond() {
    let funcname = "loop_with_cond";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(7));
}

#[test]
fn loop_inside_cond() {
    let funcname = "loop_inside_cond";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert!(args[0].unwrap_to_i32() > 7);
}

#[test]
fn loop_over_array() {
    let funcname = "loop_over_array";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn sum_of_array() {
    let funcname = "sum_of_array";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}

#[test]
fn search_array() {
    let funcname = "search_array";
    init_logging();
    let proj = get_project();
    let args = find_zero_of_func(funcname, &proj, Config::default(), None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(4));
}

#[test]
fn nested_loop() {
    let funcname = "nested_loop";
    init_logging();
    let proj = get_project();
    let mut config = Config::default();
    config.loop_bound = 50;
    let args = find_zero_of_func(funcname, &proj, config, None)
        .unwrap_or_else(|r| panic!("{}", r))
        .expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}
