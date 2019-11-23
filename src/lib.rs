//! For an introduction to the crate and how to get started,
//! see the [crate's README](https://github.com/PLSysSec/haybale/blob/master/README.md).

use llvm_ir::{Type, Typed};
use std::collections::HashSet;

mod project;
pub use project::Project;

mod symex;
pub use symex::*;

pub mod layout;
use layout::*;

mod config;
pub use config::*;
mod default_hooks;
pub mod alloc_utils;

mod state;
pub mod memory;
pub mod simple_memory;
mod alloc;
mod varmap;
mod double_keyed_map;
mod global_allocations;

pub mod solver_utils;
use solver_utils::PossibleSolutions;
mod return_value;
pub use return_value::ReturnValue;
mod error;
pub use error::*;
mod extend;

pub mod backend;
use backend::*;

/// A simple enum describing either an integer value or a pointer
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SolutionValue {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Ptr(u64),
}

impl SolutionValue {
    pub fn unwrap_to_i8(self) -> i8 {
        match self {
            SolutionValue::I8(i) => i,
            _ => panic!("unwrap_to_i8 on {:?}", self),
        }
    }

    pub fn unwrap_to_i16(self) -> i16 {
        match self {
            SolutionValue::I16(i) => i,
            _ => panic!("unwrap_to_i16 on {:?}", self),
        }
    }

    pub fn unwrap_to_i32(self) -> i32 {
        match self {
            SolutionValue::I32(i) => i,
            _ => panic!("unwrap_to_i32 on {:?}", self),
        }
    }

    pub fn unwrap_to_i64(self) -> i64 {
        match self {
            SolutionValue::I64(i) => i,
            _ => panic!("unwrap_to_i64 on {:?}", self),
        }
    }

    pub fn unwrap_to_ptr(self) -> u64 {
        match self {
            SolutionValue::Ptr(u) => u,
            _ => panic!("unwrap_to_ptr on {:?}", self),
        }
    }
}

/// Given a function, find values of its inputs such that it returns zero.
/// Assumes function takes (some number of) integer and/or pointer arguments, and returns an integer.
/// Pointer arguments will be assumed to be never NULL.
///
/// For detailed descriptions of the arguments, see [`symex_function`](fn.symex_function.html).
///
/// Returns `None` if there are no values of the inputs such that the function returns zero.
///
/// Note: `find_zero_of_func()` may be of some use itself, but is included in the
/// crate more as an example of how you can use the other public functions in the
/// crate.
pub fn find_zero_of_func<'p>(funcname: &str, project: &'p Project, config: Config<'p, BtorBackend>) -> Option<Vec<SolutionValue>> {
    let mut em: ExecutionManager<BtorBackend> = symex_function(funcname, project, config);

    // constrain pointer arguments to be not-null
    let (func, _) = project.get_func_by_name(funcname).unwrap_or_else(|| panic!("Failed to find function named {:?}", funcname));
    for (param, bv) in func.parameters.iter().zip(em.param_bvs()) {
        if let Type::PointerType { .. } = param.get_type() {
            bv._ne(&BV::zero(em.state().solver.clone(), bv.get_width())).assert();
        }
    }

    let returnwidth = size(&func.return_type);
    let zero = BV::zero(em.state().solver.clone(), returnwidth as u32);
    let mut found = false;
    while let Some(bvretval) = em.next() {
        match bvretval.unwrap() {
            ReturnValue::ReturnVoid => panic!("Function shouldn't return void"),
            ReturnValue::Return(bvretval) => {
                let state = em.mut_state();
                bvretval._eq(&zero).assert();
                if state.sat().unwrap() {
                    found = true;
                    break;
                }
            },
        }
    }

    let param_bvs: Vec<_> = em.param_bvs().clone();
    let state = em.mut_state();
    if found {
        // in this case state.sat() must have passed
        Some(func.parameters.iter().zip(param_bvs.iter()).map(|(p, bv)| {
            let param_as_u64 = state.get_a_solution_for_bv(bv).unwrap()
                .expect("since state.sat() passed, expected a solution for each var")
                .as_u64()
                .expect("parameter more than 64 bits wide");
            match &p.ty {
                Type::IntegerType { bits: 8 } => SolutionValue::I8(param_as_u64 as i8),
                Type::IntegerType { bits: 16 } => SolutionValue::I16(param_as_u64 as i16),
                Type::IntegerType { bits: 32 } => SolutionValue::I32(param_as_u64 as i32),
                Type::IntegerType { bits: 64 } => SolutionValue::I64(param_as_u64 as i64),
                Type::PointerType { .. } => SolutionValue::Ptr(param_as_u64),
                ty => unimplemented!("Function parameter with type {:?}", ty)
            }
        }).collect())
    } else {
        None
    }
}

/// Get a description of the possible return values of a function, for given
/// argument values.
/// Considers all possible paths through the function given these arguments.
///
/// `args`: For each function parameter, either a concrete value for that
/// parameter, or `None` to have the analysis consider all possible values of the
/// parameter.
///
/// `n`: Maximum number of distinct solutions to check for.
/// If there are more than `n` possible solutions, this returns a
/// `PossibleSolutions::AtLeast` containing at least `n+1` solutions.
///
/// For detailed descriptions of the other arguments, see [`symex_function`](fn.symex_function.html).
pub fn get_possible_return_values_of_func<'p>(
    funcname: &str,
    args: impl IntoIterator<Item = Option<u64>>,
    project: &'p Project,
    config: Config<'p, BtorBackend>,
    n: usize,
) -> PossibleSolutions<u64> {
    let mut em: ExecutionManager<BtorBackend> = symex_function(funcname, project, config);

    let (func, _) = project.get_func_by_name(funcname).expect("Failed to find function");
    for (param, arg) in func.parameters.iter().zip(args.into_iter()) {
        if let Some(val) = arg {
            let val = BV::from_u64(em.state().solver.clone(), val, size(&param.ty) as u32);
            em.mut_state().overwrite_latest_version_of_bv(&param.name, val);
        }
    }

    let return_width = size(&func.return_type);
    let mut candidate_values = HashSet::<u64>::new();
    while let Some(bvretval) = em.next() {
        match bvretval.unwrap() {
            ReturnValue::ReturnVoid => panic!("This function shouldn't be called with functions that return void"),
            ReturnValue::Return(bvretval) => {
                let state = em.mut_state();
                // rule out all the values we already have - we're interested in new values
                for candidate in candidate_values.iter() {
                    bvretval._ne(&BV::from_u64(state.solver.clone(), *candidate, return_width as u32)).assert();
                }
                match state.get_possible_solutions_for_bv(&bvretval, n).unwrap() {
                    PossibleSolutions::Exactly(v) => {
                        candidate_values.extend(v.iter().map(|bvsol| bvsol.as_u64().unwrap()));
                        if candidate_values.len() > n {
                            break;
                        }
                    },
                    PossibleSolutions::AtLeast(v) => {
                        candidate_values.extend(v.iter().map(|bvsol| bvsol.as_u64().unwrap()));
                        break;  // the total must be over n at this point
                    },
                };
            }
        }
    }
    if candidate_values.len() > n {
        PossibleSolutions::AtLeast(candidate_values)
    } else {
        PossibleSolutions::Exactly(candidate_values)
    }
}
