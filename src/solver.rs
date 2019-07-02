use log::debug;
use std::fmt;
use z3::ast::{BV, Bool};

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
    pub fn new(ctx: &'ctx z3::Context) -> Self {
        Self {
            z3_solver: z3::Solver::new(ctx),
            check_status: None,
            model: None,
        }
    }

    pub fn assert(&mut self, cond: &Bool<'ctx>) {
        debug!("asserting {}", cond);
        // A new assertion invalidates the cached check status and model
        self.check_status = None;
        self.model = None;
        self.z3_solver.assert(cond);
    }

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

    // Get one possible concrete value for the BV.
    // Returns None if no possible solution.
    pub fn get_a_solution_for_bv(&mut self, bv: &BV<'ctx>) -> Option<u64> {
        self.refresh_model();
        if self.check_status.unwrap() {
            Some(self.model.as_ref().unwrap().eval(bv).unwrap().as_u64().unwrap())
        } else {
            None
        }
    }

    // Get one possible concrete value for the Bool.
    // Returns None if no possible solution.
    pub fn get_a_solution_for_bool(&mut self, b: &Bool<'ctx>) -> Option<bool> {
        self.refresh_model();
        if self.check_status.unwrap() {
            Some(self.model.as_ref().unwrap().eval(b).unwrap().as_bool().unwrap())
        } else {
            None
        }
    }

    // Private function which ensures that the check status and model are up to date with the current constraints
    fn refresh_model(&mut self) {
        if self.model.is_some() { return; }  // nothing to do
        self.check();
        if self.check_status.unwrap() {
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
