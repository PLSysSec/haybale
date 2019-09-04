//! A simple helper function for use elsewhere

use boolector::{Btor, SolverResult};
use crate::error::*;

pub fn sat(btor: &Btor) -> Result<bool> {
    match btor.sat() {
        SolverResult::Sat => Ok(true),
        SolverResult::Unsat => Ok(false),
        SolverResult::Unknown => Err(Error::SolverError("The query was interrupted, timed out, or otherwise failed".to_owned())),
    }
}
