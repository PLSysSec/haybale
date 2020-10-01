#![cfg(not(feature = "llvm-9"))] // With LLVM 9 and earlier, Haybale doesn't support AtomicRMW

use haybale::backend::DefaultBackend;
use haybale::solver_utils::PossibleSolutions;
use haybale::*;
use llvm_ir::Name;

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
        &proj,
        Config::default(),
        Some(vec![
            ParameterVal::ExactValue(0xFF00),
            ParameterVal::ExactValue(0x00FF),
        ]),
        None,
        10,
    );
    assert_eq!(
        ret,
        PossibleSolutions::exactly_one(ReturnValue::Return(0xFF00))
    );

    let mut em = symex_function(
        &funcname,
        &proj,
        Config::<DefaultBackend>::default(),
        Some(vec![
            ParameterVal::ExactValue(0xFF00),
            ParameterVal::Range(1, 3),
        ]),
    ).unwrap();
    let _ = em
        .next()
        .unwrap()
        .map_err(|e| em.state().full_error_message_with_context(e))
        .unwrap();
    let var0 = em.state().get_bv_by_irname(&funcname, &Name::from(0));
    let var1 = em.state().get_bv_by_irname(&funcname, &Name::from(1));
    let var3 = em.state().get_bv_by_irname(&funcname, &Name::from(3));
    let var4 = em.state().get_bv_by_irname(&funcname, &Name::from(4));
    let var5 = em.state().get_bv_by_irname(&funcname, &Name::from(5));
    let var6 = em.state().get_bv_by_irname(&funcname, &Name::from(6));
    let var7 = em.state().get_bv_by_irname(&funcname, &Name::from(7));
    let var8 = em.state().get_bv_by_irname(&funcname, &Name::from(8));
    assert!(em.state().bvs_must_be_equal(var3, var0).unwrap());
    assert!(em.state().bvs_must_be_equal(var4, var1).unwrap());
    assert!(em.state().bvs_must_be_equal(var5, &var0.add(var1)).unwrap());
    assert!(em.state().bvs_must_be_equal(var6, var0).unwrap());
    assert!(em.state().bvs_must_be_equal(var7, &em.state().zero(var7.get_width())).unwrap()); // given the values we provided for %0 and %1, %7 must always be 0
    let sol8 = em.state().get_possible_solutions_for_bv(var8, 6).unwrap().as_u64_solutions().unwrap();
    assert_eq!(sol8, PossibleSolutions::exactly_one(3));
}
