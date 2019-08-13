use llvm_ir::Type;

mod project;
pub use project::Project;

mod symex;
pub use symex::*;

mod size;
pub use size::size;
mod extend;

mod config;
pub use config::Config;
mod default_hooks;
pub use default_hooks::*;

mod state;
pub mod memory;
mod alloc;
pub mod solver;
mod varmap;
mod double_keyed_map;

pub mod backend;
use backend::*;

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
///
/// For detailed descriptions of the arguments, see [`symex_function`](fn.symex_function.html).
///
/// Returns `None` if there are no values of the inputs such that the function returns zero.
///
/// Note: `find_zero_of_func()` may be of some use itself, but is included in the
/// crate more as an example of how you can use the other public functions in the
/// crate.
pub fn find_zero_of_func<'ctx, 'p>(ctx: &'ctx z3::Context, funcname: &str, project: &'p Project, config: Config<'ctx, Z3Backend<'ctx>>) -> Option<Vec<SolutionValue>> {
    let mut em: ExecutionManager<Z3Backend> = symex_function(ctx, funcname, project, config);
    let start_func = em.state().cur_loc.func;
    let returnwidth = size(&start_func.return_type);
    let zero = z3::ast::BV::from_u64(ctx, 0, returnwidth as u32);
    let mut found = false;
    while let Some(z3rval) = em.next() {
        match z3rval {
            SymexResult::ReturnedVoid => panic!("Function shouldn't return void"),
            SymexResult::Returned(z3rval) => {
                let state = em.mut_state();
                state.assert(&z3rval._eq(&zero));
                if state.check().unwrap() {
                    found = true;
                    break;
                }
            },
        }
    }

    let param_bvs: Vec<_> = em.param_bvs().clone();
    let state = em.mut_state();
    if found {
        // in this case state.check() must have passed
        Some(start_func.parameters.iter().zip(param_bvs.iter()).map(|(p, bv)| {
            let param_as_u64 = state.get_a_solution_for_bv(bv).unwrap()
                .expect("since state.check() passed, expected a solution for each var");
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash)]
pub enum ReturnValues {
    ExactlyOnePossibleValue(u64),
    MultiplePossibleValues,
    NoPossibleValues,
}

/// Get a description of the possible return values of a function, for given
/// argument values.
/// Considers all possible paths through the function given these arguments.
///
/// `args`: For each function parameter, either a concrete value for that
/// parameter, or `None` to have the analysis consider all possible values of the
/// parameter.
///
/// For detailed descriptions of the other arguments, see [`symex_function`](fn.symex_function.html).
pub fn get_possible_return_values_of_func<'ctx, 'p>(
    ctx: &'ctx z3::Context,
    funcname: &str,
    args: impl IntoIterator<Item = Option<u64>>,
    project: &'p Project,
    config: Config<'ctx, Z3Backend<'ctx>>,
) -> Result<ReturnValues, &'static str> {
    let mut em: ExecutionManager<Z3Backend> = symex_function(ctx, funcname, project, config);

    let (func, _) = project.get_func_by_name(funcname).expect("Failed to find function");
    for (param, arg) in func.parameters.iter().zip(args.into_iter()) {
        if let Some(val) = arg {
            em.mut_state().overwrite_latest_version_of_bv(&param.name, BV::from_u64(ctx, val, size(&param.ty) as u32));
        }
    }

    let mut candidate_value = None;
    while let Some(z3rval) = em.next() {
        match z3rval {
            SymexResult::ReturnedVoid => panic!("This function shouldn't be called with functions that return void"),
            SymexResult::Returned(z3rval) => {
                let state = em.mut_state();
                if let Some(val) = state.get_a_solution_for_bv(&z3rval)? {
                    match candidate_value {
                        None => candidate_value = Some(val),  // this will be our new candidate value
                        Some(c) if c != val => return Ok(ReturnValues::MultiplePossibleValues),  // we clearly have at least two possibilities for the value
                        Some(_) => {},  // this value was already our candidate value; do nothing
                    };
                    // now check for other possible values along this path
                    state.assert(&z3rval._eq(&BV::from_u64(ctx, val, z3rval.get_size())).not());
                    if state.check().unwrap() {
                        return Ok(ReturnValues::MultiplePossibleValues);
                    }
                }
            }
        }
    }
    match candidate_value {
        Some(val) => Ok(ReturnValues::ExactlyOnePossibleValue(val)),
        None => Ok(ReturnValues::NoPossibleValues),
    }
}
