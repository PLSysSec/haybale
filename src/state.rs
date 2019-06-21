use inkwell::basic_block::BasicBlock;
use z3;

pub struct State<'ctx> {
    solver: z3::Solver<'ctx>,
    backtrack_points: Vec<BacktrackPoint<'ctx>>,
}

struct BacktrackPoint<'ctx> {
  // BasicBlock to resume execution at
  // We use owned BasicBlocks because copy should be cheap (I'm not sure why it's not a Copy type in inkwell)
  next_bb: BasicBlock,
  // BasicBlock executed just prior to the BacktrackPoint
  prev_bb: BasicBlock,
  // Constraint to add before restarting execution at next_bb
  // (intended use of this is to constrain the branch in that direction)
  // We use owned Asts because:
  //   a) it seems necessary to not use refs, and
  //   b) it seems reasonable for callers to give us ownership of these Asts.
  //       If/when that becomes not reasonable, we should probably use boxed
  //       Asts here rather than making callers copy.
  constraint: z3::Ast<'ctx>,
}

impl<'ctx> BacktrackPoint<'ctx> {
    fn new(next_bb: BasicBlock, prev_bb: BasicBlock, constraint: z3::Ast<'ctx>) -> Self {
        BacktrackPoint{
            next_bb,
            prev_bb,
            constraint,
        }
    }
}

impl<'ctx> State<'ctx> {
    pub fn new(ctx: &'ctx z3::Context) -> Self {
        State {
            solver: z3::Solver::new(ctx),
            backtrack_points: Vec::new(),
        }
    }

    pub fn assert(&self, ast: &z3::Ast<'ctx>) {
        self.solver.assert(ast);
    }

    pub fn check(&self) -> bool {
        self.solver.check()
    }

    pub fn check_with_extra_constraints(&self, asts: &[&z3::Ast<'ctx>]) -> bool {
        self.solver.push();
        for ast in asts {
          self.solver.assert(ast);
        }
        let retval = self.solver.check();
        self.solver.pop(1);
        retval
    }

    pub fn get_model(&self) -> z3::Model<'ctx> {
      self.solver.get_model()
    }

    // again, we require owned BasicBlocks because copy should be cheap.  Caller can clone if necessary.
    // The constraint will be added only if we end up backtracking to this point, and only then
    pub fn save_backtracking_point(&mut self, next_bb: BasicBlock, prev_bb: BasicBlock, constraint: z3::Ast<'ctx>) {
        self.solver.push();
        self.backtrack_points.push(BacktrackPoint::new(next_bb, prev_bb, constraint));
    }

    // returns the BasicBlock where execution should continue and the BasicBlock executed before that
    // or None if there are no saved backtracking points left
    pub fn revert_to_backtracking_point(&mut self) -> Option<(BasicBlock, BasicBlock)> {
        self.solver.pop(1);
        if let Some(bp) = self.backtrack_points.pop() {
            self.assert(&bp.constraint);
            Some((bp.next_bb, bp.prev_bb))
            // thanks to SSA, we don't need to roll back the VarMap; we'll just overwrite existing entries as needed.
            // Code on the backtracking path will never reference variables which we assigned on the original path.
            // This will become not true when we get to loops, but we don't support loops yet anyway
        } else {
            None
        }
    }

    // in lieu of an actual Display or Debug for State (for now)
    pub fn prettyprint_constraints(&self) {
        println!("{}", self.solver);
    }
}
