//! A `Solver` based on the Z3 solver, but with an extra layer for caching
//! generated models, and some additional higher-level methods.

use crate::error::*;
use crate::possible_solutions::*;
use log::debug;
use std::fmt;
use z3::ast::{Ast, BV, Bool};

pub struct Solver<'ctx> {
    ctx: &'ctx z3::Context,
    z3_solver: z3::Solver<'ctx>,

    // Invariants:
    // if `check_status` is `Some`, then it is a cached value of the last `check()`, which is still valid
    // if `model` is `Some`, then it is a model for the current solver constraints
    // if `model` is `Some`, then `check_status` must be as well (but not necessarily vice versa)
    check_status: Option<z3::SatResult>,
    model: Option<z3::Model<'ctx>>,
}

impl<'ctx> fmt::Display for Solver<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        self.z3_solver.fmt(f)
    }
}

impl<'ctx> Solver<'ctx> {
    /// A new `Solver` with no constraints
    pub fn new(ctx: &'ctx z3::Context) -> Self {
        Self {
            ctx,
            z3_solver: z3::Solver::new(ctx),
            check_status: None,
            model: None,
        }
    }

    /// Get the `Context` this `Solver` was created with
    pub fn get_context(&self) -> &'ctx z3::Context {
        self.ctx
    }

    /// Add `constraint` as a constraint, i.e., assert that `cond` must be true
    pub fn assert(&mut self, constraint: &Bool<'ctx>) {
        let constraint = constraint.simplify();
        // A new assertion invalidates the cached check status and model
        self.check_status = None;
        self.model = None;
        self.z3_solver.assert(&constraint);
    }

    /// Returns `true` if current constraints are satisfiable, `false` if not.
    ///
    /// Returns `Error::SolverError` if the query failed (e.g., was interrupted or timed out).
    ///
    /// This function caches its result and will only call to Z3 if constraints have changed
    /// since the last call to `check()`.
    pub fn check(&mut self) -> Result<bool> {
        if let None = self.check_status {
            debug!("Solving with constraints:\n{}", self.z3_solver);
            self.check_status = Some(self.z3_solver.check());
        }
        match self.check_status.unwrap() {
            z3::SatResult::Sat => Ok(true),
            z3::SatResult::Unsat => Ok(false),
            z3::SatResult::Unknown => Err(Error::SolverError("The query was interrupted, timed out, or otherwise failed".to_owned())),
        }
    }

    /// Returns `true` if the current constraints plus the additional constraints `conds`
    /// are together satisfiable, or `false` if not.
    ///
    /// Returns `Error::SolverError` if the query failed (e.g., was interrupted or timed out).
    ///
    /// Does not permanently add the constraints in `conds` to the solver.
    pub fn check_with_extra_constraints<'b>(&'b mut self, conds: impl Iterator<Item = &'b Bool<'ctx>>) -> Result<bool> {
        // This implementation is slightly more efficient than the default
        // implementation provided by the `crate::backend::Solver` trait.
        // Although the default implementation would be correct, it would
        // needlessly invalidate the generated model, whereas for this solver,
        // we know that the cached model will still be valid after this
        // operation due to the `pop()`.
        //
        // That said, if we currently have a check status but not a cached
        // model, we will need to run `check()` again before getting the model, so
        // in that case we do intentionally invalidate the check status.
        if self.model.is_none() {
            self.check_status = None;
        }

        self.z3_solver.push();
        for cond in conds {
            self.z3_solver.assert(cond);
        }
        let retval = self.z3_solver.check();
        self.z3_solver.pop(1);
        match retval {
            z3::SatResult::Sat => Ok(true),
            z3::SatResult::Unsat => Ok(false),
            z3::SatResult::Unknown => Err(Error::SolverError("The query was interrupted, timed out, or otherwise failed".to_owned())),
        }
    }

    pub fn push(&mut self) {
        self.z3_solver.push()
    }

    pub fn pop(&mut self, n: usize) {
        self.check_status = None;
        self.model = None;
        self.z3_solver.pop(n as u32)
    }

    /// Get one possible concrete value for the `BV`.
    /// Returns `Ok(None)` if no possible solution, or `Error::SolverError` if the solver query failed.
    pub fn get_a_solution_for_bv(&mut self, bv: &BV<'ctx>) -> Result<Option<u64>> {
        self.refresh_model();
        if self.check()? {
            Ok(Some(self.model.as_ref().expect("check_status was true but we don't have a model")
                .eval(bv).expect("Have model but failed to eval bv")
                .as_u64().expect("Failed to get u64 value of eval'd bv")
            ))
        } else {
            Ok(None)
        }
    }

    /// Get one possible concrete value for specified bits (`high`, `low`) of the `BV`, inclusive on both ends.
    /// Returns `Ok(None)` if no possible solution, or `Error::SolverError` if the solver query failed.
    pub fn get_a_solution_for_specified_bits_of_bv(&mut self, bv: &BV<'ctx>, high: u32, low: u32) -> Result<Option<u64>> {
        assert!(high - low <= 63);  // this way the result will fit in a `u64`
        self.refresh_model();
        if self.check()? {
            Ok(Some(self.model.as_ref().expect("check_status was true but we don't have a model")
                .eval(bv).expect("Have model but failed to eval bv")
                .extract(high, low)
                .simplify()  // apparently necessary so that we get back to a constant rather than an extract expression
                .as_u64().expect("Failed to get u64 value of extracted bits")
            ))
        } else {
            Ok(None)
        }
    }

    /// Get one possible concrete value for the `Bool`.
    /// Returns `Ok(None)` if no possible solution, or `Error::SolverError` if the solver query failed.
    pub fn get_a_solution_for_bool(&mut self, b: &Bool<'ctx>) -> Result<Option<bool>> {
        self.refresh_model();
        if self.check()? {
            Ok(Some(self.model.as_ref().expect("check_status was true but we don't have a model")
                .eval(b).expect("Have model but failed to eval bool")
                .as_bool().expect("Failed to get value of eval'd bool")
            ))
        } else {
            Ok(None)
        }
    }

    /// Get a description of the possible solutions for the `BV`.
    ///
    /// `n`: Maximum number of distinct solutions to return.
    /// If there are more than `n` possible solutions, this simply
    /// returns `PossibleSolutions::MoreThanNPossibleSolutions(n)`.
    ///
    /// Only returns `Err` if the solver query itself fails.
    pub fn get_possible_solutions_for_bv(&mut self, bv: &BV<'ctx>, n: usize) -> Result<PossibleSolutions<u64>> {
        // This is the same as the default implementation provided by the
        // `crate::backend::Solver` trait, but is included here in case anyone
        // wants to use this `Solver` without the backend trait.
        // Also, it makes it more visible in the docs.
        let mut solutions = vec![];
        self.push();
        while solutions.len() <= n {
            match self.get_a_solution_for_bv(bv)? {
                None => break,  // no more possible solutions, we're done
                Some(val) => {
                    solutions.push(val);
                    // Temporarily constrain that the solution can't be `val`, to see if there is another solution
                    self.assert(&bv._eq(&BV::from_u64(self.get_context(), val, bv.get_size())).not());
                }
            }
        }
        self.pop(1);
        if solutions.len() > n {
            Ok(PossibleSolutions::MoreThanNPossibleSolutions(n))
        } else {
            Ok(PossibleSolutions::PossibleSolutions(solutions))
        }
    }

    /// Get a description of the possible solutions for the `Bool`.
    ///
    /// Since there cannot be more than two solutions (`true` and `false`),
    /// this should never return the `PossibleSolutions::MoreThanNPossibleSolutions` variant.
    /// Instead, it should only return one of these four things:
    ///   - `PossibleSolutions::PossibleSolutions(vec![])` indicating no possible solution,
    ///   - `PossibleSolutions::PossibleSolutions(vec![true])`,
    ///   - `PossibleSolutions::PossibleSolutions(vec![false])`,
    ///   - `PossibleSolutions::PossibleSolutions(vec![true, false])`
    ///
    /// Only returns `Err` if the solver query itself fails.
    pub fn get_possible_solutions_for_bool(&mut self, b: &Bool<'ctx>) -> Result<PossibleSolutions<bool>> {
        // This is the same as the default implementation provided by the
        // `crate::backend::Solver` trait, but is included here in case anyone
        // wants to use this `Solver` without the backend trait.
        // Also, it makes it more visible in the docs.
        self.push();
        let retval = match self.get_a_solution_for_bool(b)? {
            None => PossibleSolutions::PossibleSolutions(vec![]),
            Some(val) => {
                // Temporarily constrain that the solution can't be `val`, to see if there is another solution
                self.assert(&b._eq(&Bool::from_bool(self.get_context(), val)).not());
                match self.get_a_solution_for_bool(b)? {
                    None => PossibleSolutions::PossibleSolutions(vec![val]),
                    Some(_) => PossibleSolutions::PossibleSolutions(vec![true, false]),
                }
            }
        };
        self.pop(1);
        Ok(retval)
    }

    /// Private function which ensures that the check status and model are up to date with the current constraints
    fn refresh_model(&mut self) {
        if self.model.is_some() { return; }  // nothing to do
        if self.check() == Ok(true) {
            // check() was successful, i.e. we are sat. Generate the model.
            self.model = Some(self.z3_solver.get_model());
            debug!("Generated model:\n{}\n", self.current_model_to_pretty_string());
        }
    }

    pub fn current_model_to_pretty_string(&self) -> String {
        if let Some(model) = &self.model {
            let displayed = model.to_string();
            let sorted = itertools::sorted(displayed.lines());
            sorted.fold(String::new(), |s, line| s + "\n" + line)
        } else {
            "<no model generated>".to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sat() {
        let ctx = z3::Context::new(&z3::Config::new());
        let mut solver = Solver::new(&ctx);

        // empty solver should be sat
        assert_eq!(solver.check(), Ok(true));

        // adding True constraint should still be sat
        solver.assert(&Bool::from_bool(&ctx, true));
        assert_eq!(solver.check(), Ok(true));

        // adding x > 0 constraint should still be sat
        let x = BV::new_const(&ctx, "x", 64);
        solver.assert(&x.bvsgt(&BV::from_i64(&ctx, 0, 64)));
        assert_eq!(solver.check(), Ok(true));
    }

    #[test]
    fn unsat() {
        let ctx = z3::Context::new(&z3::Config::new());
        let mut solver = Solver::new(&ctx);

        // adding False constraint should be unsat
        solver.assert(&Bool::from_bool(&ctx, false));
        assert_eq!(solver.check(), Ok(false));
    }

    #[test]
    fn unsat_with_extra_constraints() {
        let ctx = z3::Context::new(&z3::Config::new());
        let mut solver = Solver::new(&ctx);

        // adding x > 3 constraint should still be sat
        let x = BV::new_const(&ctx, "x", 64);
        solver.assert(&x.bvugt(&BV::from_u64(&ctx, 3, 64)));
        assert_eq!(solver.check(), Ok(true));

        // adding x < 3 constraint should make us unsat
        let bad_constraint = x.bvult(&BV::from_u64(&ctx, 3, 64));
        assert_eq!(solver.check_with_extra_constraints(std::iter::once(&bad_constraint)), Ok(false));

        // the solver itself should still be sat, extra constraints weren't permanently added
        assert_eq!(solver.check(), Ok(true));
    }

    #[test]
    fn get_a_solution() {
        let ctx = z3::Context::new(&z3::Config::new());
        let mut solver = Solver::new(&ctx);

        // add x > 3 constraint
        let x = BV::new_const(&ctx, "x", 64);
        solver.assert(&x.bvugt(&BV::from_u64(&ctx, 3, 64)));

        // check that the computed value of x is > 3
        let x_value = solver.get_a_solution_for_bv(&x).unwrap().expect("Expected a solution for x");
        assert!(x_value > 3);
    }

    #[test]
    fn possible_solutions() {
        let ctx = z3::Context::new(&z3::Config::new());
        let mut solver = Solver::new(&ctx);

        // add x > 3 constraint
        let x = BV::new_const(&ctx, "x", 64);
        solver.assert(&x.bvugt(&BV::from_u64(&ctx, 3, 64)));

        // check that there are more than 2 solutions
        assert_eq!(solver.get_possible_solutions_for_bv(&x, 2), Ok(PossibleSolutions::MoreThanNPossibleSolutions(2)));

        // add x < 6 constraint
        solver.assert(&x.bvult(&BV::from_u64(&ctx, 6, 64)));

        // check that there are now exactly two solutions
        assert_eq!(solver.get_possible_solutions_for_bv(&x, 2), Ok(PossibleSolutions::PossibleSolutions(vec![4, 5])));

        // add x < 5 constraint
        solver.assert(&x.bvult(&BV::from_u64(&ctx, 5, 64)));

        // check that there is now exactly one solution
        assert_eq!(solver.get_possible_solutions_for_bv(&x, 2), Ok(PossibleSolutions::PossibleSolutions(vec![4])));

        // add x < 3 constraint
        solver.assert(&x.bvult(&BV::from_u64(&ctx, 3, 64)));

        // check that there are now no solutions
        assert_eq!(solver.get_possible_solutions_for_bv(&x, 2), Ok(PossibleSolutions::PossibleSolutions(vec![])));
    }
}
