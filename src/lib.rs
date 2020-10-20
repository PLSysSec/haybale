//! For an introduction to the crate and how to get started,
//! see the [crate's README](https://github.com/PLSysSec/haybale/blob/master/README.md).

// this ensures that crate users generating docs with --no-deps will still
// properly get links to the public docs for haybale's types
// it was especially necessary when the docs.rs docs weren't working for any
// llvm-sys consumers; now that we have docs.rs as the official docs, I'm not
// sure if this is necessary or helpful anymore
#![doc(html_root_url = "https://docs.rs/haybale/0.6.2")]

use llvm_ir::Type;
use std::collections::HashSet;

mod project;
pub use project::Project;

mod symex;
pub use symex::*;

pub mod config;
pub use config::Config;

mod error;
pub use error::*;

mod return_value;
pub use return_value::ReturnValue;

mod alloc;
pub mod alloc_utils;
pub mod backend;
pub mod callbacks;
pub mod cell_memory;
mod demangling;
mod double_keyed_map;
pub mod function_hooks;
mod global_allocations;
pub mod hook_utils;
mod hooks;
pub mod simple_memory;
pub mod solver_utils;
mod state;
mod varmap;
pub mod watchpoints;

use backend::*;
use itertools::Itertools;
use solver_utils::PossibleSolutions;

#[cfg(test)]
mod test_utils;

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
/// Assumes that the function takes (some number of) integer and/or pointer
/// arguments, and returns an integer.
/// Pointer arguments will be assumed to be never NULL.
///
/// `project`: The `Project` (set of LLVM modules) in which symbolic execution
/// should take place. In the absence of function hooks (see
/// [`Config`](struct.Config.html)), we will try to enter calls to any functions
/// defined in the `Project`.
///
/// Returns `Ok(None)` if there are no values of the inputs such that the
/// function returns zero.
///
/// Note: `find_zero_of_func()` may be of some use itself, but also serves as an
/// example of how you can use the other public functions in the crate.
pub fn find_zero_of_func<'p>(
    funcname: &str,
    project: &'p Project,
    config: Config<'p, DefaultBackend>,
) -> std::result::Result<Option<Vec<SolutionValue>>, String> {
    let mut em: ExecutionManager<DefaultBackend> = symex_function(funcname, project, config);

    // constrain pointer arguments to be not-null
    let (func, _) = project
        .get_func_by_name(funcname)
        .unwrap_or_else(|| panic!("Failed to find function named {:?}", funcname));
    for (param, bv) in func.parameters.iter().zip(em.param_bvs()) {
        if let Type::PointerType { .. } = em.state().type_of(param).as_ref() {
            bv._ne(&em.state().zero(bv.get_width())).assert();
        }
    }

    let returnwidth = match func.return_type.as_ref() {
        Type::VoidType => {
            return Err("find_zero_of_func: function has void type".into());
        },
        ty => {
            let width = project
                .size_in_bits(&ty)
                .expect("Function return type shouldn't be an opaque struct type");
            assert_ne!(width, 0, "Function return type has width 0 bits but isn't void type"); // void type was handled above
            width
        },
    };
    let zero = em.state().zero(returnwidth);
    let mut found = false;
    while let Some(bvretval) = em.next() {
        match bvretval {
            Ok(ReturnValue::ReturnVoid) => panic!("Function shouldn't return void"),
            Ok(ReturnValue::Throw(_)) => continue, // we're looking for values that result in _returning_ zero, not _throwing_ zero
            Ok(ReturnValue::Abort) => continue,
            Ok(ReturnValue::Return(bvretval)) => {
                let state = em.mut_state();
                bvretval._eq(&zero).assert();
                if state.sat()? {
                    found = true;
                    break;
                }
            },
            Err(Error::LoopBoundExceeded(_)) => continue, // ignore paths that exceed the loop bound, keep looking
            Err(e) => return Err(em.state().full_error_message_with_context(e)),
        }
    }

    let param_bvs: Vec<_> = em.param_bvs().clone();
    let state = em.mut_state();
    if found {
        // in this case state.sat() must have passed
        Ok(Some(
            func.parameters
                .iter()
                .zip_eq(param_bvs.iter())
                .map(|(p, bv)| {
                    let param_as_u64 = state
                        .get_a_solution_for_bv(bv)?
                        .expect("since state.sat() passed, expected a solution for each var")
                        .as_u64()
                        .expect("parameter more than 64 bits wide");
                    Ok(match p.ty.as_ref() {
                        Type::IntegerType { bits: 8 } => SolutionValue::I8(param_as_u64 as i8),
                        Type::IntegerType { bits: 16 } => SolutionValue::I16(param_as_u64 as i16),
                        Type::IntegerType { bits: 32 } => SolutionValue::I32(param_as_u64 as i32),
                        Type::IntegerType { bits: 64 } => SolutionValue::I64(param_as_u64 as i64),
                        Type::PointerType { .. } => SolutionValue::Ptr(param_as_u64),
                        ty => unimplemented!("Function parameter with type {:?}", ty),
                    })
                })
                .collect::<Result<_>>()?,
        ))
    } else {
        Ok(None)
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
/// `project`: The `Project` (set of LLVM modules) in which symbolic execution
/// should take place. In the absence of function hooks (see
/// [`Config`](struct.Config.html)), we will try to enter calls to any functions
/// defined in the `Project`.
///
/// `thrown_size`:
///   If this is `None`, then no attempt will be made to distinguish
///     between different values being thrown. A maximum of one `ReturnValue::Throw`
///     will be returned, and it will contain one possible pointer value.
///   If this is not `None`, then it indicates the size in bits
///     of the value or object expected to be thrown. Many different
///     `ReturnValue::Throw`s may be returned, each containing a distinct possible
///     value or object (not pointer) which may be thrown.
///
/// `n`: Maximum number of distinct solutions to check for.
/// If there are more than `n` possible solutions, this returns a
/// `PossibleSolutions::AtLeast` containing at least `n+1` solutions.
///
/// Note: `get_possible_return_values_of_func()` may be of some use itself, but
/// also serves as an example of how you can use the other public functions in
/// the crate.
pub fn get_possible_return_values_of_func<'p>(
    funcname: &str,
    args: impl IntoIterator<Item = Option<u64>>,
    project: &'p Project,
    config: Config<'p, DefaultBackend>,
    thrown_size: Option<u32>,
    n: usize,
) -> PossibleSolutions<ReturnValue<u64>> {
    let mut em: ExecutionManager<DefaultBackend> = symex_function(funcname, project, config);

    let (func, _) = project
        .get_func_by_name(funcname)
        .expect("Failed to find function");
    for (param, arg) in func.parameters.iter().zip(args.into_iter()) {
        if let Some(val) = arg {
            let param_size_bits = project
                .size_in_bits(&param.ty)
                .expect("Parameter type shouldn't be opaque struct type");
            assert_ne!(param_size_bits, 0, "Parameter {} shouldn't have size 0 bits", &param.name);
            let val = em.state().bv_from_u64(val, param_size_bits);
            em.mut_state()
                .overwrite_latest_version_of_bv(&param.name, val);
        }
    }

    let return_width = project
        .size_in_bits(&func.return_type)
        .expect("Function return type shouldn't be opaque struct type");
    let mut candidate_values = HashSet::<ReturnValue<u64>>::new();
    let mut have_throw = false; // is there at least one `ReturnValue::Throw` in the `candidate_values`
    while let Some(bvretval) = em.next() {
        match bvretval {
            Err(e) => panic!("{}", em.state().full_error_message_with_context(e)),
            Ok(ReturnValue::ReturnVoid) => {
                candidate_values.insert(ReturnValue::ReturnVoid);
                if candidate_values.len() > n {
                    break;
                }
            },
            Ok(ReturnValue::Abort) => {
                candidate_values.insert(ReturnValue::Abort);
                if candidate_values.len() > n {
                    break;
                }
            },
            Ok(ReturnValue::Return(bvretval)) => {
                assert_eq!(bvretval.get_width(), return_width);
                let state = em.mut_state();
                // rule out all the returned values we already have - we're interested in new values
                for candidate in candidate_values.iter() {
                    if let ReturnValue::Return(candidate) = candidate {
                        bvretval
                            ._ne(&state.bv_from_u64(*candidate, return_width))
                            .assert();
                    }
                }
                match state.get_possible_solutions_for_bv(&bvretval, n).unwrap() {
                    PossibleSolutions::Exactly(v) => {
                        candidate_values.extend(
                            v.iter()
                                .map(|bvsol| ReturnValue::Return(bvsol.as_u64().unwrap())),
                        );
                        if candidate_values.len() > n {
                            break;
                        }
                    },
                    PossibleSolutions::AtLeast(v) => {
                        candidate_values.extend(
                            v.iter()
                                .map(|bvsol| ReturnValue::Return(bvsol.as_u64().unwrap())),
                        );
                        break; // the total must be over n at this point
                    },
                };
            },
            Ok(ReturnValue::Throw(bvptr)) => {
                let state = em.mut_state();
                match thrown_size {
                    None => {
                        if !have_throw {
                            candidate_values.insert(ReturnValue::Throw(bvptr.as_u64().unwrap()));
                            have_throw = true;
                            if candidate_values.len() > n {
                                break;
                            }
                        }
                    },
                    Some(thrown_size) => {
                        let thrown_value = state.read(&bvptr, thrown_size).unwrap();
                        // rule out all the thrown values we already have - we're interested in new values
                        for candidate in candidate_values.iter() {
                            if let ReturnValue::Throw(candidate) = candidate {
                                thrown_value
                                    ._ne(&state.bv_from_u64(*candidate, thrown_size))
                                    .assert();
                            }
                        }
                        match state
                            .get_possible_solutions_for_bv(&thrown_value, n)
                            .unwrap()
                        {
                            PossibleSolutions::Exactly(v) => {
                                candidate_values.extend(
                                    v.iter()
                                        .map(|bvsol| ReturnValue::Throw(bvsol.as_u64().unwrap())),
                                );
                                if candidate_values.len() > n {
                                    break;
                                }
                            },
                            PossibleSolutions::AtLeast(v) => {
                                candidate_values.extend(
                                    v.iter()
                                        .map(|bvsol| ReturnValue::Throw(bvsol.as_u64().unwrap())),
                                );
                                break; // the total must be over n at this point
                            },
                        }
                    },
                }
            },
        }
    }
    if candidate_values.len() > n {
        PossibleSolutions::AtLeast(candidate_values)
    } else {
        PossibleSolutions::Exactly(candidate_values)
    }
}
