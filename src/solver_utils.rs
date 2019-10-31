//! Simple utilities for interacting with the solver

use boolector::{Btor, BVSolution, SolverResult};
use boolector::option::{BtorOption, ModelGen};
use crate::backend::BV;
use crate::error::*;
use log::warn;
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

/// Returns `true` if under the current constraints, `a` and `b` must have the
/// same value. Returns `false` if `a` and `b` may have different values. (If the
/// current constraints are themselves unsatisfiable, that will result in
/// `true`.)
///
/// A common use case for this function is to test whether some `BV` must be
/// equal to a given concrete value. You can do this with something like
/// `bvs_must_be_equal(btor, bv, BV::from_u64(...))`.
///
/// This function and `bvs_can_be_equal()` are both more efficient than
/// `get_a_solution()` or `get_possible_solutions()`-type functions, as they do
/// not require full model generation. You should prefer this function or
/// `bvs_can_be_equal()` if they are sufficient for your needs.
pub fn bvs_must_be_equal<V: BV>(btor: &Btor, a: &V, b: &V) -> Result<bool> {
    if sat_with_extra_constraints(btor, &[a._ne(&b)])? {
        Ok(false)
    } else {
        Ok(true)
    }
}

/// Returns `true` if under the current constraints, `a` and `b` can have the
/// same value. Returns `false` if `a` and `b` cannot have the same value. (If
/// the current constraints are themselves unsatisfiable, that will also result
/// in `false`.)
///
/// A common use case for this function is to test whether some `BV` can be
/// equal to a given concrete value. You can do this with something like
/// `bvs_can_be_equal(btor, bv, BV::from_u64(...))`.
///
/// This function and `bvs_must_be_equal()` are both more efficient than
/// `get_a_solution()` or `get_possible_solutions()`-type functions, as they do
/// not require full model generation. You should prefer this function or
/// `bvs_must_be_equal()` if they are sufficient for your needs.
pub fn bvs_can_be_equal<V: BV>(btor: &Btor, a: &V, b: &V) -> Result<bool> {
    if sat_with_extra_constraints(btor, &[a._eq(&b)])? {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PossibleSolutions<V: Eq + Hash> {
    /// This is exactly the set of possible solutions; there are no others.
    /// Note that an empty set here indicates there are no possible solutions.
    Exactly(HashSet<V>),
    /// All of the solutions in this set are possible solutions, but there
    /// may be others.  That is, there are at least this many solutions.
    AtLeast(HashSet<V>),
}

impl PossibleSolutions<BVSolution> {
    /// Convert a `PossibleSolutions` over `BVSolution` into a
    /// `PossibleSolutions` over `u64`, by applying `as_u64()` to each
    /// `BVSolution`.
    /// If `as_u64()` fails for any individual solution, this returns `None`.
    pub fn as_u64_solutions(&self) -> Option<PossibleSolutions<u64>> {
        match self {
            PossibleSolutions::Exactly(v) => {
                let opt = v.iter().map(|bvs| bvs.as_u64()).collect::<Option<HashSet<u64>>>();
                opt.map(PossibleSolutions::Exactly)
            },
            PossibleSolutions::AtLeast(v) => {
                let opt = v.iter().map(|bvs| bvs.as_u64()).collect::<Option<HashSet<u64>>>();
                opt.map(PossibleSolutions::AtLeast)
            },
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum SolutionCount {
    /// There are exactly this many solutions
    Exactly(usize),
    /// There are at least this many solutions
    AtLeast(usize),
}

impl<V: Eq + Hash> PossibleSolutions<V> {
    /// Get a count of how many possible solutions there are.
    pub fn count(&self) -> SolutionCount {
        match self {
            PossibleSolutions::Exactly(v) => SolutionCount::Exactly(v.len()),
            PossibleSolutions::AtLeast(v) => SolutionCount::AtLeast(v.len()),
        }
    }
}

/// Get a description of the possible solutions for the `BV`.
///
/// `n`: Maximum number of distinct solutions to check for.
/// If there are more than `n` possible solutions, this returns a
/// `PossibleSolutions::AtLeast` containing `n+1` solutions.
///
/// These solutions will be disambiguated - see docs on `boolector::BVSolution`.
///
/// If there are no possible solutions, this returns `Ok` with an empty
/// `PossibleSolutions`, rather than returning an `Err` with `Error::Unsat`.
//
// Also, this function assumes that initially ModelGen is disabled; and it will always disable ModelGen before returning.
pub fn get_possible_solutions_for_bv<V: BV>(solver: V::SolverRef, bv: &V, n: usize) -> Result<PossibleSolutions<BVSolution>> {
    let ps = if n == 0 {
        warn!("A call to get_possible_solutions_for_bv() is resulting in a call to sat() with model generation enabled. Experimentally, these types of calls can be very slow.");
        solver.set_opt(BtorOption::ModelGen(ModelGen::All));
        if sat(&solver)? {
            PossibleSolutions::AtLeast(std::iter::once(
                bv.get_a_solution()?.disambiguate()  // a possible solution
            ).collect())
        } else {
            PossibleSolutions::Exactly(HashSet::new())  // no solutions
        }
    } else {
        match bv.as_binary_str() {
            Some(bstr) => PossibleSolutions::Exactly(
                std::iter::once(BVSolution::from_01x_str(bstr)).collect()
            ),
            None => {
                let mut solutions = HashSet::new();
                check_for_common_solutions(solver.clone(), bv, n, &mut solutions)?;
                if solutions.len() > n {
                    PossibleSolutions::AtLeast(solutions)
                } else {
                    solver.push(1);
                    for solution in solutions.iter() {
                        // Temporarily constrain that the solution can't be `solution` - we want to see if other solutions exist
                        bv._ne(&BV::from_binary_str(solver.clone(), solution.as_01x_str())).assert()?;
                    }
                    warn!("A call to get_possible_solutions_for_bv() is resulting in a call to sat() with model generation enabled. Experimentally, these types of calls can be very slow.");
                    solver.set_opt(BtorOption::ModelGen(ModelGen::All));
                    while solutions.len() <= n && sat(&solver)? {
                        let val = bv.get_a_solution()?.disambiguate();
                        solutions.insert(val.clone());
                        // Temporarily constrain that the solution can't be `val`, to see if there is another solution
                        bv._ne(&BV::from_binary_str(solver.clone(), val.as_01x_str())).assert()?;
                    }
                    solver.pop(1);
                    if solutions.len() > n {
                        PossibleSolutions::AtLeast(solutions)
                    } else {
                        PossibleSolutions::Exactly(solutions)
                    }
                }
            },
        }
    };
    solver.set_opt(BtorOption::ModelGen(ModelGen::Disabled));
    Ok(ps)
}

/// Check whether some common values are solutions, and if so, add them.
///
/// Experimental data shows that calls to `sat()` with ModelGen enabled are _so slow_
/// that it's worth doing this first to try to avoid them.
fn check_for_common_solutions<V: BV>(solver: V::SolverRef, bv: &V, n: usize, solutions: &mut HashSet<BVSolution>) -> Result<()> {
    let width = bv.get_width();
    if solutions.len() <= n && bvs_can_be_equal(&solver, bv, &BV::zero(solver.clone(), width))? {
        solutions.insert(BVSolution::from_01x_str("0".repeat(width as usize)));
    }
    if solutions.len() <= n && bvs_can_be_equal(&solver, bv, &BV::one(solver.clone(), width))? {
        solutions.insert(BVSolution::from_01x_str(format!("{:0width$b}", 1, width=width as usize)));
    }
    if solutions.len() <= n && width > 1 && bvs_can_be_equal(&solver, bv, &BV::ones(solver.clone(), width))? {
        solutions.insert(BVSolution::from_01x_str("1".repeat(width as usize)));
    }
    if solutions.len() <= n && width > 1 && bvs_can_be_equal(&solver, bv, &BV::from_u32(solver.clone(), 2, width))? {
        solutions.insert(BVSolution::from_01x_str(format!("{:0width$b}", 2, width=width as usize)));
    }
    if solutions.len() <= n && width > 2 && bvs_can_be_equal(&solver, bv, &BV::from_u32(solver.clone(), 4, width))? {
        solutions.insert(BVSolution::from_01x_str(format!("{:0width$b}", 4, width=width as usize)));
    }
    if solutions.len() <= n && width > 3 && bvs_can_be_equal(&solver, bv, &BV::from_u32(solver.clone(), 8, width))? {
        solutions.insert(BVSolution::from_01x_str(format!("{:0width$b}", 8, width=width as usize)));
    }
    if solutions.len() <= n && width > 4 && bvs_can_be_equal(&solver, bv, &BV::from_u32(solver.clone(), 16, width))? {
        solutions.insert(BVSolution::from_01x_str(format!("{:0width$b}", 16, width=width as usize)));
    }
    if solutions.len() <= n && width > 5 && bvs_can_be_equal(&solver, bv, &BV::from_u32(solver.clone(), 32, width))? {
        solutions.insert(BVSolution::from_01x_str(format!("{:0width$b}", 32, width=width as usize)));
    }
    if solutions.len() <= n && width > 6 && bvs_can_be_equal(&solver, bv, &BV::from_u32(solver.clone(), 64, width))? {
        solutions.insert(BVSolution::from_01x_str(format!("{:0width$b}", 64, width=width as usize)));
    }
    if solutions.len() <= n && width > 7 && bvs_can_be_equal(&solver, bv, &BV::from_u32(solver.clone(), 128, width))? {
        solutions.insert(BVSolution::from_01x_str(format!("{:0width$b}", 128, width=width as usize)));
    }
    if solutions.len() <= n && width > 8 && bvs_can_be_equal(&solver, bv, &BV::from_u32(solver.clone(), 256, width))? {
        solutions.insert(BVSolution::from_01x_str(format!("{:0width$b}", 256, width=width as usize)));
    }
    if solutions.len() <= n && width > 9 && bvs_can_be_equal(&solver, bv, &BV::from_u32(solver.clone(), 512, width))? {
        solutions.insert(BVSolution::from_01x_str(format!("{:0width$b}", 512, width=width as usize)));
    }
    if solutions.len() <= n && width > 10 && bvs_can_be_equal(&solver, bv, &BV::from_u32(solver.clone(), 1024, width))? {
        solutions.insert(BVSolution::from_01x_str(format!("{:0width$b}", 1024, width=width as usize)));
    }
    if solutions.len() <= n && width > 11 && bvs_can_be_equal(&solver, bv, &BV::from_u32(solver.clone(), 2048, width))? {
        solutions.insert(BVSolution::from_01x_str(format!("{:0width$b}", 2048, width=width as usize)));
    }
    if solutions.len() <= n && width > 12 && bvs_can_be_equal(&solver, bv, &BV::from_u32(solver.clone(), 4096, width))? {
        solutions.insert(BVSolution::from_01x_str(format!("{:0width$b}", 4096, width=width as usize)));
    }
    Ok(())
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
    // Shortcut: if the BV is constant, just return its constant value
    if let Some(u) = bv.as_u64() {
        return Ok(Some(u));
    }
    // Shortcut: check all-ones first, and if it's a valid solution, just return that
    if bvs_can_be_equal(&solver, bv, &V::ones(solver.clone(), width))? {
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
        let mid = if mid / 2 > min { mid / 2 } else { mid };  // as another small optimization, rather than checking the midpoint (pure binary search) we bias towards the small end (checking effectively the 25th percentile if min is 0) as we assume small positive numbers are more common, this gets us towards 0 with half the number of solves
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
    // Shortcut: if the BV is constant, just return its constant value
    if let Some(u) = bv.as_u64() {
        return Ok(Some(u));
    }
    // Shortcut: check `0` first, and if it's a valid solution, just return that
    if bvs_can_be_equal(&solver, bv, &V::zero(solver.clone(), width))? {
        return Ok(Some(0));
    }
    // min is exclusive (we know `0` doesn't work), max is inclusive
    let mut min: u64 = 0;
    let mut max: u64 = if width == 64 { std::u64::MAX } else { (1 << width) - 1 };
    let mut pushes = 0;
    while (max - min) > 1 {
        let mid = (min / 2) + (max / 2) + (min % 2 + max % 2) / 2; // (min + max) / 2 would be easier, but fails if (min + max) overflows
        let mid = if mid / 2 > min { mid / 2 } else { mid };  // as another small optimization, rather than checking the midpoint (pure binary search) we bias towards the small end (checking effectively the 25th percentile if min is 0) as we assume small positive numbers are more common, this gets us towards 0 with half the number of solves
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
    fn can_or_must_be_equal() {
        let btor = BtorRef::default();

        // create constants 2, 3, 4, 5, and 7, which we'll need
        let two = BV::from_u64(btor.clone().into(), 2, 64);
        let three = BV::from_u64(btor.clone().into(), 3, 64);
        let four = BV::from_u64(btor.clone().into(), 4, 64);
        let five = BV::from_u64(btor.clone().into(), 5, 64);
        let seven = BV::from_u64(btor.clone().into(), 7, 64);

        // add an x > 3 constraint
        let x: BV = BV::new(btor.clone().into(), 64, Some("x"));
        x.ugt(&three).assert();

        // we should have that x _can be_ 7 but not _must be_ 7
        assert_eq!(bvs_can_be_equal(&btor, &x, &seven), Ok(true));
        assert_eq!(bvs_must_be_equal(&btor, &x, &seven), Ok(false));

        // we should have that x neither _can be_ nor _must be_ 2
        assert_eq!(bvs_can_be_equal(&btor, &x, &two), Ok(false));
        assert_eq!(bvs_must_be_equal(&btor, &x, &two), Ok(false));

        // add an x < 5 constraint
        x.ult(&five).assert();

        // we should now have that x both _can be_ and _must be_ 4
        assert_eq!(bvs_can_be_equal(&btor, &x, &four), Ok(true));
        assert_eq!(bvs_must_be_equal(&btor, &x, &four), Ok(true));
    }

    #[test]
    fn possible_solutions() {
        let btor = BtorRef::default();

        // add x > 3 constraint
        let x: BV = BV::new(btor.clone().into(), 64, Some("x"));
        x.ugt(&BV::from_u64(btor.clone().into(), 3, 64)).assert();

        // check that there are more than 2 solutions
        let num_solutions = get_possible_solutions_for_bv(btor.clone().into(), &x, 2).unwrap().count();
        assert_eq!(num_solutions, SolutionCount::AtLeast(3));

        // add x < 6 constraint
        x.ult(&BV::from_u64(btor.clone().into(), 6, 64)).assert();

        // check that there are now exactly two solutions
        let solutions = get_possible_solutions_for_bv(btor.clone().into(), &x, 2).unwrap().as_u64_solutions();
        assert_eq!(solutions, Some(PossibleSolutions::Exactly([4,5].into_iter().copied().collect())));

        // add x < 5 constraint
        x.ult(&BV::from_u64(btor.clone().into(), 5, 64)).assert();

        // check that there is now exactly one solution
        let solutions = get_possible_solutions_for_bv(btor.clone().into(), &x, 2).unwrap().as_u64_solutions();
        assert_eq!(solutions, Some(PossibleSolutions::Exactly(std::iter::once(4).collect())));

        // add x < 3 constraint
        x.ult(&BV::from_u64(btor.clone().into(), 3, 64)).assert();

        // check that there are now no solutions
        let solutions = get_possible_solutions_for_bv(btor.clone().into(), &x, 2).unwrap().as_u64_solutions();
        assert_eq!(solutions, Some(PossibleSolutions::Exactly(HashSet::new())));
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
