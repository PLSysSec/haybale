use boolector::BVSolution;
use crate::backend::BV;
use crate::error::*;
use crate::sat::sat;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PossibleSolutions<V: Eq + Hash> {
    /// This is exactly the set of possible solutions; there are no others.
    /// Note that an empty set here indicates there are no possible solutions.
    PossibleSolutions(HashSet<V>),
    /// There are more than `n` possible solutions, where `n` is the value
    /// contained here.
    MoreThanNPossibleSolutions(usize),
}

impl PossibleSolutions<BVSolution> {
    /// Convert a `PossibleSolutions` over `BVSolution` into a
    /// `PossibleSolutions` over `u64`, by applying `as_u64()` to each
    /// `BVSolution`.
    /// If `as_u64()` fails for any individual solution, this returns `None`.
    pub fn as_u64_solutions(&self) -> Option<PossibleSolutions<u64>> {
        match self {
            PossibleSolutions::PossibleSolutions(v) => {
                let opt = v.iter().map(|bvs| bvs.as_u64()).collect::<Option<HashSet<u64>>>();
                opt.map(PossibleSolutions::PossibleSolutions)
            },
            PossibleSolutions::MoreThanNPossibleSolutions(n) => Some(PossibleSolutions::MoreThanNPossibleSolutions(*n)),
        }
    }
}

/// Get a description of the possible solutions for the `BV`.
///
/// `n`: Maximum number of distinct solutions to return.
/// If there are more than `n` possible solutions, this simply
/// returns `PossibleSolutions::MoreThanNPossibleSolutions(n)`.
///
/// These solutions will be disambiguated - see docs on `boolector::BVSolution`.
///
/// Only returns `Err` if the solver query itself fails.
pub fn get_possible_solutions_for_bv<V: BV>(solver: V::SolverRef, bv: &V, n: usize) -> Result<PossibleSolutions<BVSolution>> {
    let mut solutions = HashSet::new();
    solver.push(1);
    while solutions.len() <= n && sat(&solver.clone())? {
        let val = bv.get_a_solution().disambiguate();
        solutions.insert(val.clone());
        // Temporarily constrain that the solution can't be `val`, to see if there is another solution
        bv._ne(&BV::from_binary_str(solver.clone(), val.as_01x_str())).assert();
    }
    solver.pop(1);
    if solutions.len() > n {
        Ok(PossibleSolutions::MoreThanNPossibleSolutions(n))
    } else {
        Ok(PossibleSolutions::PossibleSolutions(solutions))
    }
}
