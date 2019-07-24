use llvm_ir::*;
use pitchfork_rs::*;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

type Path<'func> = Vec<QualifiedBB>;

fn path_from_bbnames(funcname: &str, bbnames: impl IntoIterator<Item = Name>) -> Path {
    let mut vec = vec![];
    for bbname in bbnames {
        vec.push(QualifiedBB { funcname: funcname.to_string(), bbname });
    }
    vec
}

fn path_from_bbnums(funcname: &str, bbnums: impl IntoIterator<Item = usize>) -> Path {
    path_from_bbnames(funcname, bbnums.into_iter().map(|u| Name::Number(u)))
}

/// Iterator over the paths through a function
struct PathIterator<'ctx, 'func> {
    em: ExecutionManager<'ctx, 'func>,
}

impl<'ctx, 'func> PathIterator<'ctx, 'func> {
    pub fn new(ctx: &'ctx z3::Context, func: &'func Function, loop_bound: usize) -> Self {
        Self { em: symex_function(ctx, func, loop_bound) }
    }
}

impl<'ctx, 'func> Iterator for PathIterator<'ctx, 'func> {
    type Item = Path<'func>;

    fn next(&mut self) -> Option<Self::Item> {
        self.em.next().map(|_| self.em.state().path.clone())
    }
}

#[test]
fn one_block() {
    init_logging();
    let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/basic.bc"))
        .expect("Failed to parse basic.bc module");
    let func = module.get_func_by_name("one_arg").expect("Failed to find function");
    let ctx = z3::Context::new(&z3::Config::new());
    let paths: Vec<Path> = PathIterator::new(&ctx, func, 5).collect();
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0], path_from_bbnums(&func.name, vec![1]));
}

#[test]
fn two_paths() {
    init_logging();
    let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/basic.bc"))
        .expect("Failed to parse basic.bc module");
    let func = module.get_func_by_name("conditional_true").expect("Failed to find function");
    let ctx = z3::Context::new(&z3::Config::new());
    let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, func, 5)).collect();
    assert_eq!(paths.len(), 2);
    assert_eq!(paths[0], path_from_bbnums(&func.name, vec![2, 4, 12]));
    assert_eq!(paths[1], path_from_bbnums(&func.name, vec![2, 8, 12]));
}

#[test]
fn four_paths() {
    init_logging();
    let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/basic.bc"))
        .expect("Failed to parse basic.bc module");
    let func = module.get_func_by_name("conditional_nozero").expect("Failed to find function");
    let ctx = z3::Context::new(&z3::Config::new());
    let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, func, 5)).collect();
    assert_eq!(paths.len(), 4);
    assert_eq!(paths[0], path_from_bbnums(&func.name, vec![2, 4, 6, 14]));
    assert_eq!(paths[1], path_from_bbnums(&func.name, vec![2, 4, 8, 10, 14]));
    assert_eq!(paths[2], path_from_bbnums(&func.name, vec![2, 4, 8, 12, 14]));
    assert_eq!(paths[3], path_from_bbnums(&func.name, vec![2, 14]));
}

#[test]
fn while_loop() {
    init_logging();
    let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/loop.bc"))
        .expect("Failed to parse loop.bc module");
    let func = module.get_func_by_name("while_loop").expect("Failed to find function");
    let ctx = z3::Context::new(&z3::Config::new());
    let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, func, 5)).collect();
    assert_eq!(paths.len(), 5);
    assert_eq!(paths[0], path_from_bbnums(&func.name, vec![1, 6, 6, 6, 6, 6, 12]));
    assert_eq!(paths[1], path_from_bbnums(&func.name, vec![1, 6, 6, 6, 6, 12]));
    assert_eq!(paths[2], path_from_bbnums(&func.name, vec![1, 6, 6, 6, 12]));
    assert_eq!(paths[3], path_from_bbnums(&func.name, vec![1, 6, 6, 12]));
    assert_eq!(paths[4], path_from_bbnums(&func.name, vec![1, 6, 12]));
}

#[test]
fn for_loop() {
    init_logging();
    let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/loop.bc"))
        .expect("Failed to parse loop.bc module");
    let func = module.get_func_by_name("for_loop").expect("Failed to find function");
    let ctx = z3::Context::new(&z3::Config::new());
    let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, func, 5)).collect();
    assert_eq!(paths.len(), 6);
    assert_eq!(paths[0], path_from_bbnums(&func.name, vec![1, 6]));
    assert_eq!(paths[1], path_from_bbnums(&func.name, vec![1, 9, 6]));
    assert_eq!(paths[2], path_from_bbnums(&func.name, vec![1, 9, 9, 6]));
    assert_eq!(paths[3], path_from_bbnums(&func.name, vec![1, 9, 9, 9, 6]));
    assert_eq!(paths[4], path_from_bbnums(&func.name, vec![1, 9, 9, 9, 9, 6]));
    assert_eq!(paths[5], path_from_bbnums(&func.name, vec![1, 9, 9, 9, 9, 9, 6]));
}

#[test]
fn loop_more_blocks() {
    init_logging();
    let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/loop.bc"))
        .expect("Failed to parse loop.bc module");
    let func = module.get_func_by_name("loop_zero_iterations").expect("Failed to find function");
    let ctx = z3::Context::new(&z3::Config::new());
    let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, func, 5)).collect();
    assert_eq!(paths.len(), 7);
    assert_eq!(paths[0], path_from_bbnums(&func.name, vec![1, 5, 8, 18]));
    assert_eq!(paths[1], path_from_bbnums(&func.name, vec![1, 5, 11, 8, 18]));
    assert_eq!(paths[2], path_from_bbnums(&func.name, vec![1, 5, 11, 11, 8, 18]));
    assert_eq!(paths[3], path_from_bbnums(&func.name, vec![1, 5, 11, 11, 11, 8, 18]));
    assert_eq!(paths[4], path_from_bbnums(&func.name, vec![1, 5, 11, 11, 11, 11, 8, 18]));
    assert_eq!(paths[5], path_from_bbnums(&func.name, vec![1, 5, 11, 11, 11, 11, 11, 8, 18]));
    assert_eq!(paths[6], path_from_bbnums(&func.name, vec![1, 18]));
}

#[test]
fn loop_more_blocks_in_body() {
    init_logging();
    let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/loop.bc"))
        .expect("Failed to parse loop.bc module");
    let func = module.get_func_by_name("loop_with_cond").expect("Failed to find function");
    let ctx = z3::Context::new(&z3::Config::new());
    let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, func, 5)).collect();
    assert_eq!(paths.len(), 5);
    assert_eq!(paths[0], path_from_bbnums(&func.name, vec![1, 6, 13, 16,
                                                              6, 10, 16,
                                                              6, 10, 16,
                                                              6, 13, 16,
                                                              6, 10, 16, 20]));
    assert_eq!(paths[1], path_from_bbnums(&func.name, vec![1, 6, 13, 16,
                                                              6, 10, 16,
                                                              6, 10, 16,
                                                              6, 13, 16, 20]));
    assert_eq!(paths[2], path_from_bbnums(&func.name, vec![1, 6, 13, 16,
                                                              6, 10, 16,
                                                              6, 10, 16, 20]));
    assert_eq!(paths[3], path_from_bbnums(&func.name, vec![1, 6, 13, 16,
                                                              6, 10, 16, 20]));
    assert_eq!(paths[4], path_from_bbnums(&func.name, vec![1, 6, 13, 16, 20]));
}

#[test]
fn two_loops() {
    init_logging();
    let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/loop.bc"))
        .expect("Failed to parse loop.bc module");
    let func = module.get_func_by_name("sum_of_array").expect("Failed to find function");
    let ctx = z3::Context::new(&z3::Config::new());
    let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, func, 30)).collect();
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0], path_from_bbnums(&func.name, vec![1, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
                                                           11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 9]));
}

#[test]
fn nested_loop() {
    init_logging();
    let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/loop.bc"))
        .expect("Failed to parse loop.bc module");
    let func = module.get_func_by_name("nested_loop").expect("Failed to find function");
    let ctx = z3::Context::new(&z3::Config::new());
    let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, func, 30)).collect();
    assert_eq!(paths.len(), 4);
    assert_eq!(paths[0], path_from_bbnums(&func.name, vec![1, 5, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                                                           10, 5, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                                                           10, 5, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                                                           10, 7]));
    assert_eq!(paths[1], path_from_bbnums(&func.name, vec![1, 5, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                                                           10, 5, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                                                           10, 7]));
    assert_eq!(paths[2], path_from_bbnums(&func.name, vec![1, 5, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                                                           10, 7]));
    assert_eq!(paths[3], path_from_bbnums(&func.name, vec![1, 7]));
}
