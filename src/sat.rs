//! Simple wrappers for interacting with the solver

use boolector::{Btor, SolverResult};
use crate::backend::BV;
use crate::error::*;
use std::ops::Deref;

/// Returns `true` if current constraints are satisfiable, `false` if not.
///
/// Returns `Error::SolverError` if the query failed (e.g., was interrupted or timed out).
pub fn sat(btor: &Btor) -> Result<bool> {
    match btor.sat() {
        SolverResult::Sat => Ok(true),
        SolverResult::Unsat => Ok(false),
        SolverResult::Unknown => Err(Error::SolverError("The query was interrupted, timed out, or otherwise failed".to_owned())),
    }
}

/// Returns `true` if the current constraints plus the additional constraints `conds`
/// are together satisfiable, or `false` if not.
///
/// Returns `Error::SolverError` if the query failed (e.g., was interrupted or timed out).
///
/// Does not permanently add the constraints in `conds` to the solver.
pub fn sat_with_extra_constraints<I, B>(btor: &Btor, constraints: impl IntoIterator<Item = I>) -> Result<bool>
    where I: Deref<Target = B>, B: BV
{
    btor.push(1);
    for constraint in constraints {
        constraint.assert();
    }
    let retval = sat(btor);
    btor.pop(1);
    retval
}
