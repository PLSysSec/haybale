//! Simple utilities for interacting with the solver

use boolector::{Btor, BVSolution, SolverResult};
use boolector::option::{BtorOption, ModelGen};
use crate::backend::BV;
use crate::error::*;
use std::collections::HashSet;
use std::hash::Hash;
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
        constraint.assert()?;
    }
    let retval = sat(btor);
    btor.pop(1);
    retval
}

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
/// Only returns `Err` if a solver query itself fails.
pub fn get_possible_solutions_for_bv<V: BV>(solver: V::SolverRef, bv: &V, n: usize) -> Result<PossibleSolutions<BVSolution>> {
    if n == 0 {
        if sat(&solver.clone())? {
            Ok(PossibleSolutions::MoreThanNPossibleSolutions(0))
        } else {
            Ok(PossibleSolutions::PossibleSolutions(HashSet::new()))
        }
    } else {
        match bv.as_binary_str() {
            Some(bstr) => Ok(PossibleSolutions::PossibleSolutions(
                std::iter::once(BVSolution::from_01x_str(bstr)).collect()
            )),
            None => {
                let mut solutions = HashSet::new();
                solver.push(1);
                solver.set_opt(BtorOption::ModelGen(ModelGen::All));
                while solutions.len() <= n && sat(&solver.clone())? {
                    let val = bv.get_a_solution().disambiguate();
                    solutions.insert(val.clone());
                    // Temporarily constrain that the solution can't be `val`, to see if there is another solution
                    bv._ne(&BV::from_binary_str(solver.clone(), val.as_01x_str())).assert()?;
                }
                solver.set_opt(BtorOption::ModelGen(ModelGen::Disabled));
                solver.pop(1);
                if solutions.len() > n {
                    Ok(PossibleSolutions::MoreThanNPossibleSolutions(n))
                } else {
                    Ok(PossibleSolutions::PossibleSolutions(solutions))
                }
            },
        }
    }
}

/// Get the maximum possible solution for the `BV`: that is, the highest value
/// for which the current set of constraints is still satisfiable.
/// "Maximum" will be interpreted in an unsigned fashion.
///
/// Returns `Ok(None)` if there is no solution for the `BV`, that is, if the
/// current set of constraints is unsatisfiable. Only returns `Err` if a solver
/// query itself fails.
pub fn max_possible_solution_for_bv<V: BV>(solver: V::SolverRef, bv: &V) -> Result<Option<u64>> {
    let width = bv.get_width();
    if width > 64 {
        unimplemented!("max_possible_solution_for_bv on a BV with width > 64");
    }
    if !sat(&solver)? {
        return Ok(None);
    }
    // Shortcut: check all-ones first, and if it's a valid solution, just return that
    if sat_with_extra_constraints(&solver, &[bv._eq(&V::ones(solver.clone(), width))])? {
        if width == 64 {
            return Ok(Some(std::u64::MAX));
        } else {
            return Ok(Some((1 << width) - 1));
        }
    }
    // min is inclusive, max is exclusive (we know all-ones doesn't work)
    let mut min: u64 = 0;
    let mut max: u64 = if width == 64 { std::u64::MAX } else { (1 << width) - 1 };
    let mut pushes = 0;
    while (max - min) > 1 {
        let mid = (min / 2) + (max / 2) + (min % 2 + max % 2) / 2; // (min + max) / 2 would be easier, but fails if (min + max) overflows
        solver.push(1);
        pushes += 1;
        bv.ugte(&V::from_u64(solver.clone(), mid, width)).assert()?;
        if sat(&solver)? {
            min = mid;
        } else {
            max = mid;
            solver.pop(1);
            pushes -= 1;
        }
    }
    solver.pop(pushes);
    assert_eq!(max - min, 1);
    // Recall that min is inclusive, max is exclusive. So `min` is actually the
    // max possible solution here.
    Ok(Some(min))
}

/// Get the minimum possible solution for the `BV`: that is, the lowest value
/// for which the current set of constraints is still satisfiable.
/// "Maximum" will be interpreted in an unsigned fashion.
///
/// Returns `Ok(None)` if there is no solution for the `BV`, that is, if the
/// current set of constraints is unsatisfiable. Only returns `Err` if a solver
/// query itself fails.
pub fn min_possible_solution_for_bv<V: BV>(solver: V::SolverRef, bv: &V) -> Result<Option<u64>> {
    let width = bv.get_width();
    if width > 64 {
        unimplemented!("min_possible_solution_for_bv on a BV with width > 64");
    }
    if !sat(&solver)? {
        return Ok(None);
    }
    // Shortcut: check `0` first, and if it's a valid solution, just return that
    if sat_with_extra_constraints(&solver, &[bv._eq(&V::zero(solver.clone(), width))])? {
        return Ok(Some(0));
    }
    // min is exclusive (we know `0` doesn't work), max is inclusive
    let mut min: u64 = 0;
    let mut max: u64 = if width == 64 { std::u64::MAX } else { (1 << width) - 1 };
    let mut pushes = 0;
    while (max - min) > 1 {
        let mid = (min / 2) + (max / 2) + (min % 2 + max % 2) / 2; // (min + max) / 2 would be easier, but fails if (min + max) overflows
        solver.push(1);
        pushes += 1;
        bv.ulte(&V::from_u64(solver.clone(), mid, width)).assert()?;
        if sat(&solver)? {
            max = mid;
        } else {
            min = mid;
            solver.pop(1);
            pushes -= 1;
        }
    }
    solver.pop(pushes);
    assert_eq!(max - min, 1);
    // Recall that min is exclusive, max is inclusive. So `max` is actually the
    // min possible solution here.
    Ok(Some(max))
}

#[cfg(test)]
mod tests {
    use crate::backend::BtorRef;
    use super::*;

    type BV = <BtorRef as crate::backend::SolverRef>::BV;

    #[test]
    fn basic_sat() {
        let btor = BtorRef::default();

        // fresh btor should be sat
        assert_eq!(sat(&btor), Ok(true));

        // adding True constraint should still be sat
        BV::from_bool(btor.clone().into(), true).assert();
        assert_eq!(sat(&btor), Ok(true));

        // adding x > 0 constraint should still be sat
        let x: BV = BV::new(btor.clone().into(), 64, Some("x"));
        x.sgt(&BV::zero(btor.clone().into(), 64)).assert();
        assert_eq!(sat(&btor), Ok(true));
    }

    #[test]
    fn basic_unsat() {
        let btor = BtorRef::default();

        // adding False constraint should be unsat
        BV::from_bool(btor.clone().into(), false).assert();
        assert_eq!(sat(&btor), Ok(false));
    }

    #[test]
    fn extra_constraints() {
        let btor = BtorRef::default();

        // adding x > 3 constraint should still be sat
        let x: BV = BV::new(btor.clone().into(), 64, Some("x"));
        x.ugt(&BV::from_u64(btor.clone().into(), 3, 64)).assert();
        assert_eq!(sat(&btor), Ok(true));

        // adding x < 3 constraint should make us unsat
        let bad_constraint = x.ult(&BV::from_u64(btor.clone().into(), 3, 64));
        assert_eq!(sat_with_extra_constraints(&btor, std::iter::once(&bad_constraint)), Ok(false));

        // the solver itself should still be sat, extra constraints weren't permanently added
        assert_eq!(sat(&btor), Ok(true));
    }

    #[test]
    fn possible_solutions() {
        let btor = BtorRef::default();

        // add x > 3 constraint
        let x: BV = BV::new(btor.clone().into(), 64, Some("x"));
        x.ugt(&BV::from_u64(btor.clone().into(), 3, 64)).assert();

        // check that there are more than 2 solutions
        let solutions = get_possible_solutions_for_bv(btor.clone().into(), &x, 2).unwrap().as_u64_solutions();
        assert_eq!(solutions, Some(PossibleSolutions::MoreThanNPossibleSolutions(2)));

        // add x < 6 constraint
        x.ult(&BV::from_u64(btor.clone().into(), 6, 64)).assert();

        // check that there are now exactly two solutions
        let solutions = get_possible_solutions_for_bv(btor.clone().into(), &x, 2).unwrap().as_u64_solutions();
        assert_eq!(solutions, Some(PossibleSolutions::PossibleSolutions([4,5].into_iter().copied().collect())));

        // add x < 5 constraint
        x.ult(&BV::from_u64(btor.clone().into(), 5, 64)).assert();

        // check that there is now exactly one solution
        let solutions = get_possible_solutions_for_bv(btor.clone().into(), &x, 2).unwrap().as_u64_solutions();
        assert_eq!(solutions, Some(PossibleSolutions::PossibleSolutions(std::iter::once(4).collect())));

        // add x < 3 constraint
        x.ult(&BV::from_u64(btor.clone().into(), 3, 64)).assert();

        // check that there are now no solutions
        let solutions = get_possible_solutions_for_bv(btor.clone().into(), &x, 2).unwrap().as_u64_solutions();
        assert_eq!(solutions, Some(PossibleSolutions::PossibleSolutions(HashSet::new())));
    }

    #[test]
    fn min_possible_solution() {
        let btor = BtorRef::default();

        // add x > 3 constraint
        let x: BV = BV::new(btor.clone().into(), 64, Some("x"));
        x.ugt(&BV::from_u64(btor.clone().into(), 3, 64)).assert();

        // min possible solution should be 4
        assert_eq!(min_possible_solution_for_bv(btor.clone().into(), &x), Ok(Some(4)));

        // add x < 6 constraint
        x.ult(&BV::from_u64(btor.clone().into(), 6, 64)).assert();

        // min possible solution should still be 4
        assert_eq!(min_possible_solution_for_bv(btor.clone().into(), &x), Ok(Some(4)));

        // add x < 3 constraint
        x.ult(&BV::from_u64(btor.clone().into(), 3, 64)).assert();

        // min_possible_solution_for_bv should now return None
        assert_eq!(min_possible_solution_for_bv(btor.clone().into(), &x), Ok(None));
    }

    #[test]
    fn max_possible_solution() {
        let btor = BtorRef::default();

        // add x < 7 constraint
        let x: BV = BV::new(btor.clone().into(), 64, Some("x"));
        x.ult(&BV::from_u64(btor.clone().into(), 7, 64)).assert();

        // max possible solution should be 6
        assert_eq!(max_possible_solution_for_bv(btor.clone().into(), &x), Ok(Some(6)));

        // but min possible solution should be 0
        assert_eq!(min_possible_solution_for_bv(btor.clone().into(), &x), Ok(Some(0)));

        // add x > 3 constraint
        x.ugt(&BV::from_u64(btor.clone().into(), 3, 64)).assert();

        // max possible solution should still be 6
        assert_eq!(max_possible_solution_for_bv(btor.clone().into(), &x), Ok(Some(6)));

        // and min possible solution should now be 4
        assert_eq!(min_possible_solution_for_bv(btor.clone().into(), &x), Ok(Some(4)));

        // add x > 7 constraint
        x.ugt(&BV::from_u64(btor.clone().into(), 7, 64)).assert();

        // max_possible_solution_for_bv should now return None
        assert_eq!(max_possible_solution_for_bv(btor.clone().into(), &x), Ok(None));
    }

    #[test]
    fn min_possible_solution_overflow() {
        let btor = BtorRef::default();

        // Constrain x so that -2 and -1 are the only possible solutions. This
        // means that the min possible _unsigned_ solution will be 0b1111...1110
        // (that is, -2 if we interpreted it as signed).
        let x: BV = BV::new(btor.clone().into(), 64, Some("x"));
        let zero = BV::zero(btor.clone().into(), 64);
        x.slt(&zero).assert();
        let minustwo = zero.sub(&BV::from_u64(btor.clone().into(), 2, 64));
        x.sgte(&minustwo).assert();

        // The min possible (unsigned) solution should be -2
        assert_eq!(min_possible_solution_for_bv(btor.clone().into(), &x), Ok(Some((-2_i64) as u64)));
    }

    #[test]
    fn max_possible_solution_overflow() {
        let btor = BtorRef::default();

        // Constrain x so that -2 is a solution but -1 is not. This means that the max possible
        // _unsigned_ solution will be 0b1111...1110 (that is, -2 if we interpreted it as signed).
        let x: BV = BV::new(btor.clone().into(), 64, Some("x"));
        let minustwo = BV::zero(btor.clone().into(), 64).sub(&BV::from_u64(btor.clone().into(), 2, 64));
        x.slte(&minustwo).assert();

        // The max possible (unsigned) solution should be -2
        assert_eq!(max_possible_solution_for_bv(btor.clone().into(), &x), Ok(Some((-2_i64) as u64)));
    }
}
