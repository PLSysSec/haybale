use inkwell::basic_block::BasicBlock;
use inkwell::values::*;
use log::debug;
use std::collections::HashMap;
use std::fmt;
use z3::ast::{Ast, BV, Bool};

use crate::utils::*;

type VarMap<'ctx> = HashMap<AnyValueEnum, BVorBool<'ctx>>;

// Our VarMap stores both BVs and Bools
#[derive(Clone, PartialEq, Eq)]
enum BVorBool<'ctx> {
    BV(BV<'ctx>),
    Bool(Bool<'ctx>),
}

impl<'ctx> fmt::Debug for BVorBool<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BVorBool::BV(bv) => write!(f, "BV( {} )", bv),
            BVorBool::Bool(b) => write!(f, "Bool( {} )", b),
        }
    }
}

impl<'ctx> From<BV<'ctx>> for BVorBool<'ctx> {
    fn from(bv: BV<'ctx>) -> BVorBool<'ctx> {
        BVorBool::BV(bv)
    }
}

impl<'ctx> From<Bool<'ctx>> for BVorBool<'ctx> {
    fn from(b: Bool<'ctx>) -> BVorBool<'ctx> {
        BVorBool::Bool(b)
    }
}

impl<'ctx> From<BVorBool<'ctx>> for BV<'ctx> {
    fn from(b: BVorBool<'ctx>) -> BV<'ctx> {
        match b {
            BVorBool::BV(bv) => bv,
            _ => panic!("Can't convert {:?} to BV", b),
        }
    }
}

impl<'ctx> From<BVorBool<'ctx>> for Bool<'ctx> {
    fn from(b: BVorBool<'ctx>) -> Bool<'ctx> {
        match b {
            BVorBool::Bool(b) => b,
            _ => panic!("Can't convert {:?} to Bool", b),
        }
    }
}

// these are basically From impls, but for converting ref to ref
impl<'ctx> BVorBool<'ctx> {
    fn as_bv(&self) -> &BV<'ctx> {
        match self {
            BVorBool::BV(bv) => &bv,
            _ => panic!("Can't convert {:?} to BV", self),
        }
    }

    fn as_bool(&self) -> &Bool<'ctx> {
        match self {
            BVorBool::Bool(b) => &b,
            _ => panic!("Can't convert {:?} to Bool", self),
        }
    }
}

pub struct State<'ctx> {
    pub ctx: &'ctx z3::Context,
    solver: z3::Solver<'ctx>,
    vars: VarMap<'ctx>,
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
  // We use owned Bools because:
  //   a) it seems necessary to not use refs, and
  //   b) it seems reasonable for callers to give us ownership of these Bools.
  //       If/when that becomes not reasonable, we should probably use boxed
  //       Bools here rather than making callers copy.
  constraint: Bool<'ctx>,
}

impl<'ctx> BacktrackPoint<'ctx> {
    fn new(next_bb: BasicBlock, prev_bb: BasicBlock, constraint: Bool<'ctx>) -> Self {
        BacktrackPoint{
            next_bb,
            prev_bb,
            constraint,
        }
    }
}

impl<'ctx> fmt::Display for BacktrackPoint<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<BacktrackPoint to execute bb {} with constraint {}>",
            get_bb_name(self.next_bb), self.constraint)
    }
}

impl<'ctx> State<'ctx> {
    pub fn new(ctx: &'ctx z3::Context) -> Self {
        State {
            ctx,
            solver: z3::Solver::new(ctx),
            vars: HashMap::new(),
            backtrack_points: Vec::new(),
        }
    }

    pub fn assert(&self, cond: &Bool<'ctx>) {
        debug!("asserting {}", cond);
        self.solver.assert(cond);
    }

    pub fn check(&self) -> bool {
        debug!("Solving with constraints:\n{}", self.solver);
        self.solver.check()
    }

    pub fn check_with_extra_constraints(&self, conds: &[&Bool<'ctx>]) -> bool {
        self.solver.push();
        for cond in conds {
          self.solver.assert(cond);
        }
        let retval = self.solver.check();
        self.solver.pop(1);
        retval
    }

    pub fn get_model(&self) -> z3::Model<'ctx> {
        let model = self.solver.get_model();
        debug!("Returned model:\n{}", model);
        model
    }

    // Associate the given value with the given BV
    pub fn add_bv_var(&mut self, v: impl AnyValue + Copy, bv: BV<'ctx>) {
        debug!("Adding var {} = {}", get_value_name(v), bv);
        self.vars.insert(v.as_any_value_enum(), bv.into());
    }

    // Associate the given value with the given Bool
    pub fn add_bool_var(&mut self, v: impl AnyValue + Copy, b: Bool<'ctx>) {
        debug!("Adding var {} = {}", get_value_name(v), b);
        self.vars.insert(v.as_any_value_enum(), b.into());
    }

    // Look up the BV previously created for the given value
    pub fn lookup_bv_var(&self, v: impl AnyValue + Copy) -> &BV<'ctx> {
        debug!("Looking up var {}", get_value_name(v));
        self.vars.get(&v.as_any_value_enum()).unwrap_or_else(|| {
            let keys: Vec<&AnyValueEnum> = self.vars.keys().collect();
            panic!("Failed to find value {:?} in map with keys {:?}", v, keys);
        }).as_bv()
    }

    // Look up the Bool previously created for the given value
    pub fn lookup_bool_var(&self, v: impl AnyValue + Copy) -> &Bool<'ctx> {
        debug!("Looking up var {}", get_value_name(v));
        self.vars.get(&v.as_any_value_enum()).unwrap_or_else(|| {
            let keys: Vec<&AnyValueEnum> = self.vars.keys().collect();
            panic!("Failed to find value {:?} in map with keys {:?}", v, keys);
        }).as_bool()
    }

    // Convert a Value to the appropriate BV
    // Should be an operand, that is, an RHS value
    // (that way, we know it's either a constant or a variable we previously added to the state)
    pub fn operand_to_bv(&self, v: impl BasicValue + Copy) -> BV<'ctx> {
        match v.as_basic_value_enum() {
            BasicValueEnum::IntValue(iv) => {
                if iv.is_const() {
                    BV::from_u64(self.ctx, iv.get_zero_extended_constant().unwrap(), iv.get_type().get_bit_width())
                } else {
                    self.lookup_bv_var(v).clone()
                }
            },
            v => unimplemented!("operand_to_bv() for {:?}", v)
        }
    }

    // Convert an IntValue to the appropriate Bool
    // Should be an operand, that is, an RHS value
    // (that way, we know it's either a constant or a variable we previously added to the state)
    // This will panic if the Value isn't an LLVM i1 type
    pub fn operand_to_bool(&self, v: IntValue) -> Bool<'ctx> {
        assert_eq!(v.get_type().get_bit_width(), 1);
        if v.is_const() {
            Bool::from_bool(self.ctx, v.get_zero_extended_constant().unwrap() != 0)
        } else {
            self.lookup_bool_var(v).clone()
        }
    }

    // again, we require owned BasicBlocks because copy should be cheap.  Caller can clone if necessary.
    // The constraint will be added only if we end up backtracking to this point, and only then
    pub fn save_backtracking_point(&mut self, next_bb: BasicBlock, prev_bb: BasicBlock, constraint: Bool<'ctx>) {
        debug!("Saving a backtracking point, which would enter bb {:?} with constraint {}", get_bb_name(next_bb), constraint);
        self.solver.push();
        self.backtrack_points.push(BacktrackPoint::new(next_bb, prev_bb, constraint));
    }

    // returns the BasicBlock where execution should continue and the BasicBlock executed before that
    // or None if there are no saved backtracking points left
    pub fn revert_to_backtracking_point(&mut self) -> Option<(BasicBlock, BasicBlock)> {
        if let Some(bp) = self.backtrack_points.pop() {
            debug!("Reverting to backtracking point {}", bp);
            self.solver.pop(1);
            debug!("Constraints are now:\n{}", self.solver);
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
