use log::debug;
use std::convert::TryInto;
use std::fmt;
use z3::ast::{Ast, BV, Bool};

pub struct Solver<'ctx> {
    z3_solver: z3::Solver<'ctx>,

    // Invariants:
    // if `check_status` is `Some`, then it is a cached value of the last `check()`, which is still valid
    // if `model` is `Some`, then it is a model for the current solver constraints
    // if `model` is `Some`, then `check_status` must be as well (but not necessarily vice versa)
    check_status: Option<z3::SatResult>,
    model: Option<z3::Model<'ctx>>,
}

impl<'ctx> Solver<'ctx> {
    /// A new `Solver` with no constraints
    pub fn new(ctx: &'ctx z3::Context) -> Self {
        Self {
            z3_solver: z3::Solver::new(ctx),
            check_status: None,
            model: None,
        }
    }

    /// Add `cond` as a constraint, i.e., assert that `cond` must be true
    pub fn assert(&mut self, cond: &Bool<'ctx>) {
        let cond = cond.simplify();
        // A new assertion invalidates the cached check status and model
        self.check_status = None;
        self.model = None;
        self.z3_solver.assert(&cond);
    }

    /// Returns `true` if current constraints are satisfiable, `false` if not.
    ///
    /// Returns `Err` if the query failed (e.g., was interrupted or timed out).
    ///
    /// This function caches its result and will only call to Z3 if constraints have changed
    /// since the last call to `check()`.
    pub fn check(&mut self) -> Result<bool, &'static str> {
        match self.check_status {
            Some(status) => status.try_into(),
            None => {
                debug!("Solving with constraints:\n{}", self.z3_solver);
                self.check_status = Some(self.z3_solver.check());
                self.check_status.unwrap().try_into()
            }
        }
    }

    /// Returns `true` if the current constraints plus the additional constraints `conds`
    /// are together satisfiable, or `false` if not.
    ///
    /// Returns `Err` if the query failed (e.g., was interrupted or timed out).
    ///
    /// Does not permanently add the constraints in `conds` to the solver.
    pub fn check_with_extra_constraints<'b>(&'b mut self, conds: impl Iterator<Item = &'b Bool<'ctx>>) -> Result<bool, &'static str> {
        // although the check status by itself would not be invalidated by this,
        // we do need to run check() again before getting the model,
        // so we indicate that by invalidating the check status if we don't have a model
        if self.model.is_none() {
            self.check_status = None;
        }

        self.z3_solver.push();
        for cond in conds {
            self.z3_solver.assert(cond);
        }
        let retval = self.z3_solver.check();
        self.z3_solver.pop(1);
        retval.try_into()
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
    /// Returns `Ok(None)` if no possible solution, or `Err` if solver query failed.
    pub fn get_a_solution_for_bv(&mut self, bv: &BV<'ctx>) -> Result<Option<u64>, &'static str> {
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
    /// Returns `Ok(None)` if no possible solution, or `Err` if solver query failed.
    pub fn get_a_solution_for_specified_bits_of_bv(&mut self, bv: &BV<'ctx>, high: u32, low: u32) -> Result<Option<u64>, &'static str> {
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
    /// Returns `Ok(None)` if no possible solution, or `Err` if solver query failed.
    pub fn get_a_solution_for_bool(&mut self, b: &Bool<'ctx>) -> Result<Option<bool>, &'static str> {
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
    fn get_model() {
        let ctx = z3::Context::new(&z3::Config::new());
        let mut solver = Solver::new(&ctx);

        // add x > 3 constraint
        let x = BV::new_const(&ctx, "x", 64);
        solver.assert(&x.bvugt(&BV::from_u64(&ctx, 3, 64)));

        // check that the computed value of x is > 3
        let x_value = solver.get_a_solution_for_bv(&x).unwrap().expect("Expected a solution for x");
        assert!(x_value > 3);
    }
}

impl<'ctx> fmt::Display for Solver<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.z3_solver.fmt(f)
    }
}
