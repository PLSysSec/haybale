use log::debug;
use std::fmt;
use z3::ast::{Ast, BV, Bool};

pub struct Solver<'ctx> {
    z3_solver: z3::Solver<'ctx>,

    // Invariants:
    // if `check_status` is `Some`, then it is a cached value of the last `check()`, which is still valid
    // if `model` is `Some`, then it is a model for the current solver constraints
    // if `model` is `Some`, then `check_status` must be as well (but not necessarily vice versa)
    check_status: Option<bool>,
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
    /// This function caches its result and will only call to Z3 if constraints have changed
    /// since the last call to `check()`.
    pub fn check(&mut self) -> bool {
        match self.check_status {
            Some(status) => status,
            None => {
                debug!("Solving with constraints:\n{}", self.z3_solver);
                self.check_status = Some(self.z3_solver.check());
                self.check_status.unwrap()
            }
        }
    }

    /// Returns `true` if the current constraints plus the additional constraints `conds`
    /// are together satisfiable, or `false` if not.
    /// Does not permanently add the constraints in `conds` to the solver.
    pub fn check_with_extra_constraints(&mut self, conds: &[&Bool<'ctx>]) -> bool {
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

        if retval {
            debug!("Would be sat with extra constraints {:?}", conds);
        } else {
            debug!("Would be unsat with extra constraints {:?}", conds);
        }
        retval
    }

    pub fn push(&mut self) {
        self.z3_solver.push()
    }

    pub fn pop(&mut self, n: u32) {
        self.check_status = None;
        self.model = None;
        self.z3_solver.pop(n)
    }

    /// Get one possible concrete value for the `BV`.
    /// Returns `None` if no possible solution.
    pub fn get_a_solution_for_bv(&mut self, bv: &BV<'ctx>) -> Option<u64> {
        self.refresh_model();
        if self.check() {
            Some(self.model.as_ref().expect("check_status was true but we don't have a model")
                .eval(bv).expect("Have model but failed to eval bv")
                .as_u64().expect("Failed to get u64 value of eval'd bv"))
        } else {
            None
        }
    }

    /// Get one possible concrete value for specified bits (`high`, `low`) of the `BV`, inclusive on both ends.
    /// Returns `None` if no possible solution.
    pub fn get_a_solution_for_specified_bits_of_bv(&mut self, bv: &BV<'ctx>, high: u32, low: u32) -> Option<u64> {
        assert!(high - low <= 63);  // this way the result will fit in a `u64`
        self.refresh_model();
        if self.check() {
            Some(self.model.as_ref().expect("check_status was true but we don't have a model")
                .eval(bv).expect("Have model but failed to eval bv")
                .extract(high, low)
                .simplify()  // apparently necessary so that we get back to a constant rather than an extract expression
                .as_u64().expect("Failed to get u64 value of extracted bits"))
        } else {
            None
        }
    }

    /// Get one possible concrete value for the `Bool`.
    /// Returns `None` if no possible solution.
    pub fn get_a_solution_for_bool(&mut self, b: &Bool<'ctx>) -> Option<bool> {
        self.refresh_model();
        if self.check_status.unwrap() {
            Some(self.model.as_ref().expect("check_status was true but we don't have a model")
                .eval(b).expect("Have model but failed to eval bool")
                .as_bool().expect("Failed to get value of eval'd bool"))
        } else {
            None
        }
    }

    /// Private function which ensures that the check status and model are up to date with the current constraints
    fn refresh_model(&mut self) {
        if self.model.is_some() { return; }  // nothing to do
        if self.check() {
            // check() was successful, i.e. we are sat. Generate the model.
            self.model = Some(self.z3_solver.get_model());
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
        assert!(solver.check());

        // adding True constraint should still be sat
        solver.assert(&Bool::from_bool(&ctx, true));
        assert!(solver.check());

        // adding x > 0 constraint should still be sat
        let x = BV::new_const(&ctx, "x", 64);
        solver.assert(&x.bvsgt(&BV::from_i64(&ctx, 0, 64)));
        assert!(solver.check());
    }

    #[test]
    fn unsat() {
        let ctx = z3::Context::new(&z3::Config::new());
        let mut solver = Solver::new(&ctx);

        // adding False constraint should be unsat
        solver.assert(&Bool::from_bool(&ctx, false));
        assert!(!solver.check());
    }

    #[test]
    fn unsat_with_extra_constraints() {
        let ctx = z3::Context::new(&z3::Config::new());
        let mut solver = Solver::new(&ctx);

        // adding x > 3 constraint should still be sat
        let x = BV::new_const(&ctx, "x", 64);
        solver.assert(&x.bvugt(&BV::from_u64(&ctx, 3, 64)));
        assert!(solver.check());

        // adding x < 3 constraint should make us unsat
        let bad_constraint = x.bvult(&BV::from_u64(&ctx, 3, 64));
        assert!(!solver.check_with_extra_constraints(&[&bad_constraint]));

        // the solver itself should still be sat, extra constraints weren't permanently added
        assert!(solver.check());
    }

    #[test]
    fn get_model() {
        let ctx = z3::Context::new(&z3::Config::new());
        let mut solver = Solver::new(&ctx);

        // add x > 3 constraint
        let x = BV::new_const(&ctx, "x", 64);
        solver.assert(&x.bvugt(&BV::from_u64(&ctx, 3, 64)));

        // check that the computed value of x is > 3
        let x_value = solver.get_a_solution_for_bv(&x).expect("Expected a solution for x");
        assert!(x_value > 3);
    }
}

impl<'ctx> fmt::Display for Solver<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.z3_solver.fmt(f)
    }
}
