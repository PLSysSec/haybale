use haybale::solver_utils::PossibleSolutions;
use haybale::*;
use std::path::Path;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn simd_add() {
    let funcname = "simd_add";
    init_logging();
    let proj = Project::from_bc_path(&Path::new("tests/bcfiles/simd_cl.bc"))
        .unwrap_or_else(|e| panic!("Failed to parse simd_cl.bc module: {}", e));

    // This function effectively computes 4x + 4y + 6.
    // So with x=3 and y=5, we should have 12 + 20 + 6 = 38.
    let args = std::iter::once(3).chain(std::iter::once(5)).map(Some);
    assert_eq!(
        get_possible_return_values_of_func(funcname, args, &proj, Config::default(), None, 5),
        PossibleSolutions::exactly_one(ReturnValue::Return(38)),
    );
}

#[test]
fn simd_ops() {
    let funcname = "simd_ops";
    init_logging();
    let proj = Project::from_bc_path(&Path::new("tests/bcfiles/simd_cl.bc"))
        .unwrap_or_else(|e| panic!("Failed to parse simd_cl.bc module: {}", e));

    // We compute the function's output for x=4, y=7
    let args = std::iter::once(4).chain(std::iter::once(7)).map(Some);
    let a_1: u32 = 4;
    let a_2: u32 = 4;
    let a_3: u32 = 4;
    let a_4: u32 = 4;
    let b_1: u32 = 7;
    let b_2: u32 = 8;
    let b_3: u32 = 9;
    let b_4: u32 = 10;
    let c_1: u32 = a_1 + b_1 - 3;
    let c_2: u32 = a_2 + b_2 - 3;
    let c_3: u32 = a_3 + b_3 - 3;
    let c_4: u32 = a_4 + b_4 - 3;
    let d_1: u32 = c_1 * 17;
    let d_2: u32 = c_2 * 17;
    let d_3: u32 = c_3 * 17;
    let d_4: u32 = c_4 * 17;
    let e_1: u32 = d_1 & (!a_1) | b_1;
    let e_2: u32 = d_2 & (!a_2) | b_2;
    let e_3: u32 = d_3 & (!a_3) | b_3;
    let e_4: u32 = d_4 & (!a_4) | b_4;
    let f_1: u32 = e_1 >> 2;
    let f_2: u32 = e_2 >> 2;
    let f_3: u32 = e_3 >> 2;
    let f_4: u32 = e_4 >> 2;
    let g_1: u32 = f_1 << 2;
    let g_2: u32 = f_2 << 3;
    let g_3: u32 = f_3 << 4;
    let g_4: u32 = f_4 << 5;
    let retval: u32 = g_1 + g_2 + g_3 + g_4;
    assert_eq!(
        get_possible_return_values_of_func(funcname, args, &proj, Config::default(), None, 5),
        PossibleSolutions::exactly_one(ReturnValue::Return(retval as u64)),
    );
}

#[test]
fn simd_select() {
    let funcname = "simd_select";
    init_logging();
    let proj = Project::from_bc_path(&Path::new("tests/bcfiles/simd_cl.bc"))
        .unwrap_or_else(|e| panic!("Failed to parse simd_cl.bc module: {}", e));

    // We compute the function's output for x=4, y=3
    let args = std::iter::once(4).chain(std::iter::once(3)).map(Some);
    let a_1: u32 = 4;
    let a_2: u32 = 4;
    let a_3: u32 = 4;
    let a_4: u32 = 4;
    let b_1: u32 = 3;
    let b_2: u32 = 4;
    let b_3: u32 = 5;
    let b_4: u32 = 6;
    let c_1: u32 = if a_1 < b_1 { a_1 } else { b_1 };
    let c_2: u32 = if a_2 < b_2 { a_2 } else { b_2 };
    let c_3: u32 = if a_3 < b_3 { a_3 } else { b_3 };
    let c_4: u32 = if a_4 < b_4 { a_4 } else { b_4 };
    let retval = c_1 + c_2 + c_3 + c_4;
    assert_eq!(
        get_possible_return_values_of_func(funcname, args, &proj, Config::default(), None, 5),
        PossibleSolutions::exactly_one(ReturnValue::Return(retval as u64)),
    );
}

#[test]
fn simd_add_autovectorized() {
    let funcname = "simd_add_autovectorized";
    init_logging();
    let proj = Project::from_bc_path(&Path::new("tests/bcfiles/simd.bc"))
        .unwrap_or_else(|e| panic!("Failed to parse simd.bc module: {}", e));

    let x_sum: u32 = (0 .. 16).sum();
    let y_sum: u32 = (2 .. 18).sum();
    let z_sum: u32 = x_sum + y_sum;
    assert_eq!(
        get_possible_return_values_of_func(
            funcname,
            std::iter::empty(),
            &proj,
            Config::default(),
            None,
            5
        ),
        PossibleSolutions::exactly_one(ReturnValue::Return(z_sum as u64)),
    );
}

#[test]
fn simd_typeconversions() {
    let funcname = "simd_typeconversions";
    init_logging();
    let proj = Project::from_bc_path(&Path::new("tests/bcfiles/simd_cl.bc"))
        .unwrap_or_else(|e| panic!("Failed to parse simd_cl.bc module: {}", e));

    // We compute the function's output for x=3, y=5
    let args = std::iter::once(3).chain(std::iter::once(5)).map(Some);
    let a_1: u32 = 3;
    let a_2: u32 = 3;
    let a_3: u32 = 3;
    let a_4: u32 = 3;
    let b_1: u32 = 5;
    let b_2: u32 = 15;
    let b_3: u32 = 8;
    let b_4: u32 = 35;
    let c_1: u64 = u64::from(a_1);
    let c_2: u64 = u64::from(a_2);
    let c_3: u64 = u64::from(a_3);
    let c_4: u64 = u64::from(a_4);
    let d_1: u64 = u64::from(b_1);
    let d_2: u64 = u64::from(b_2);
    let d_3: u64 = u64::from(b_3);
    let d_4: u64 = u64::from(b_4);
    let e_1: u64 = d_1 - c_1;
    let e_2: u64 = d_2 - c_2;
    let e_3: u64 = d_3 - c_3;
    let e_4: u64 = d_4 - c_4;
    let f_1: u32 = e_1 as u32;
    let f_2: u32 = e_2 as u32;
    let f_3: u32 = e_3 as u32;
    let f_4: u32 = e_4 as u32;
    let retval = f_1 + f_2 + f_3 + f_4;
    assert_eq!(
        get_possible_return_values_of_func(funcname, args, &proj, Config::default(), None, 5),
        PossibleSolutions::exactly_one(ReturnValue::Return(retval as u64)),
    )
}
