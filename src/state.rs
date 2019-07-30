use llvm_ir::*;
use log::debug;
use std::fmt;
use z3::ast::{Ast, BV, Bool};

use crate::memory::Memory;
use crate::solver::Solver;
use crate::alloc::Alloc;
use crate::varmap::{VarMap, RestoreInfo};
use crate::size::size;

pub struct State<'ctx, 'm> {
    /// Reference to the Z3 context being used
    pub ctx: &'ctx z3::Context,
    /// Indicates the `BasicBlock` which is currently being executed
    pub cur_loc: Location<'m>,
    /// `Name` of the `BasicBlock` which was executed before this one;
    /// or `None` if this is the first `BasicBlock` being executed
    /// or the first `BasicBlock` of a function
    pub prev_bb_name: Option<Name>,
    /// Log of the basic blocks which have been executed to get to this point
    pub path: Vec<QualifiedBB>,

    // Private members
    varmap: VarMap<'ctx>,
    mem: Memory<'ctx>,
    alloc: Alloc,
    solver: Solver<'ctx>,
    /// This tracks the call stack of the symbolic execution.
    /// The first entry is the top-level caller, while the last entry is the
    /// caller of the current function.
    ///
    /// We won't have a `StackFrame` for the current function here, only each of
    /// its callers. For instance, while we are executing the top-level function,
    /// this stack will be empty.
    stack: Vec<StackFrame<'ctx, 'm>>,
    /// These backtrack points are places where execution can be resumed later
    /// (efficiently, thanks to the incremental solving capabilities of Z3).
    backtrack_points: Vec<BacktrackPoint<'ctx, 'm>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct QualifiedBB {
    pub funcname: String,
    pub bbname: Name,
}

impl fmt::Debug for QualifiedBB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pretty_name = match self.bbname {
            Name::Name(ref s) => format!("{:?}", s),
            Name::Number(n) => format!("%{}", n),
        };
        write!(f, "{{{} {}}}", self.funcname, pretty_name)
    }
}

#[derive(Clone)]
pub struct Location<'m> {
    pub module: &'m Module,
    pub func: &'m Function,
    pub bbname: Name,
}

/// Implementation of `PartialEq` assumes that module and function names are unique
impl<'m> PartialEq for Location<'m> {
    fn eq(&self, other: &Self) -> bool {
        self.module.name == other.module.name
            && self.func.name == other.func.name
            && self.bbname == other.bbname
    }
}

/// Our implementation of `PartialEq` satisfies the requirements of `Eq`
impl<'m> Eq for Location<'m> {}

impl<'m> fmt::Debug for Location<'m> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Location: module {:?}, func {:?}, bb {:?}", self.module.name, self.func.name, self.bbname)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Callsite<'m> {
    /// `Module`, `Function`, and `BasicBlock` of the callsite
    pub loc: Location<'m>,
    /// Index of the `Call` instruction within the `BasicBlock`
    pub inst: usize,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct StackFrame<'ctx, 'm> {
    /// Indicates the call instruction which was responsible for the call
    callsite: Callsite<'m>,
    /// Caller's local variables, so they can be restored when we return to the caller.
    /// This is necessary in the case of (direct or indirect) recursion.
    /// See notes on `VarMap.get_restore_info_for_fn()`.
    restore_info: RestoreInfo<'ctx>,
}

struct BacktrackPoint<'ctx, 'm> {
    /// Where to resume execution
    loc: Location<'m>,
    /// `Name` of the `BasicBlock` executed just prior to the `BacktrackPoint`.
    /// Assumed to be in the same `Module` and `Function` as `loc` (which is
    /// always true for how we currently use `BacktrackPoint`s as of this writing)
    prev_bb: Name,
    /// Call stack at the `BacktrackPoint`.
    /// This is a vector of `StackFrame`s where the first entry is the top-level
    /// caller, and the last entry is the caller of the `BacktrackPoint`'s function.
    stack: Vec<StackFrame<'ctx, 'm>>,
    /// Constraint to add before restarting execution at `next_bb`.
    /// (Intended use of this is to constrain the branch in that direction.)
    constraint: Bool<'ctx>,
    /// `VarMap` representing the state of things at the `BacktrackPoint`.
    /// For now, we require making a full copy of the `VarMap` in order to revert
    /// later.
    varmap: VarMap<'ctx>,
    /// `Memory` representing the state of things at the `BacktrackPoint`.
    /// Copies of a `Memory` should be cheap (just a Z3 object pointer), so it's
    /// not a huge concern that we need a full copy here in order to revert later.
    mem: Memory<'ctx>,
    /// The length of `path` at the `BacktrackPoint`.
    /// If we ever revert to this `BacktrackPoint`, we will truncate the `path` to
    /// its first `path_len` entries.
    path_len: usize,
}

impl<'ctx, 'm> fmt::Display for BacktrackPoint<'ctx, 'm> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<BacktrackPoint to execute bb {:?} with constraint {} and {} frames on the callstack>", self.loc.bbname, self.constraint, self.stack.len())
    }
}

impl<'ctx, 'm> State<'ctx, 'm> {
    /// `start_loc`: the `Location` where the `State` should begin executing.
    ///   As of this writing, this should be the entry point of a function, or you
    ///   will have problems.
    /// `max_versions_of_name`: the maximum number of versions allowed of a given `Name`,
    ///   that is, the maximum number of Z3 objects created for a given LLVM SSA value.
    ///   Used to bound both loop iterations and recursion depth.
    pub fn new(ctx: &'ctx z3::Context, start_loc: Location<'m>, max_versions_of_name: usize) -> Self {
        Self {
            ctx,
            cur_loc: start_loc,
            prev_bb_name: None,
            path: Vec::new(),
            varmap: VarMap::new(ctx, max_versions_of_name),
            mem: Memory::new(ctx),
            alloc: Alloc::new(),
            solver: Solver::new(ctx),
            stack: Vec::new(),
            backtrack_points: Vec::new(),
        }
    }

    /// Add `cond` as a constraint, i.e., assert that `cond` must be true
    pub fn assert(&mut self, cond: &Bool<'ctx>) {
        self.solver.assert(cond)
    }

    /// Returns `true` if current constraints are satisfiable, `false` if not.
    /// This function caches its result and will only call to Z3 if constraints have changed
    /// since the last call to `check()`.
    pub fn check(&mut self) -> bool {
        self.solver.check()
    }

    /// Returns `true` if the current constraints plus the additional constraints `conds`
    /// are together satisfiable, or `false` if not.
    /// Does not permanently add the constraints in `conds` to the solver.
    pub fn check_with_extra_constraints(&mut self, conds: &[&Bool<'ctx>]) -> bool {
        self.solver.check_with_extra_constraints(conds)
    }

    /// Get one possible concrete value for the `BV`.
    /// Returns `None` if no possible solution.
    pub fn get_a_solution_for_bv(&mut self, bv: &BV<'ctx>) -> Option<u64> {
        self.solver.get_a_solution_for_bv(bv)
    }

    /// Get one possible concrete value for the `Bool`.
    /// Returns `None` if no possible solution.
    pub fn get_a_solution_for_bool(&mut self, b: &Bool<'ctx>) -> Option<bool> {
        self.solver.get_a_solution_for_bool(b)
    }

    /// Get one possible concrete value for the given IR `Name` (from the given `Function` name), which represents a bitvector.
    /// Returns `None` if no possible solution.
    pub fn get_a_solution_for_bv_by_irname(&mut self, funcname: &String, name: &Name) -> Option<u64> {
        let bv = self.varmap.lookup_bv_var(funcname, name).clone();  // clone() so that the borrow of self is released
        self.get_a_solution_for_bv(&bv)
    }

    /// Get one possible concrete value for the given IR `Name` (from the given `Function` name), which represents a bool.
    /// Returns `None` if no possible solution.
    pub fn get_a_solution_for_bool_by_irname(&mut self, funcname: &String, name: &Name) -> Option<bool> {
        let b = self.varmap.lookup_bool_var(funcname, name).clone();  // clone() so that the borrow of self is released
        self.get_a_solution_for_bool(&b)
    }

    /// Create a new `BV` for the given `Name` (in the current function).
    /// This function performs uniquing, so if you call it twice
    /// with the same `Name`-`Function` pair, you will get two different `BV`s.
    /// Returns the new `BV`, or `Err` if it can't be created.
    /// (As of this writing, the only reason an `Err` might be returned is that
    /// creating the new `BV` would exceed `max_versions_of_name` -- see
    /// [`State::new()`](struct.State.html#method.new).)
    /// Also, we assume that no two `Function`s share the same name.
    pub fn new_bv_with_name(&mut self, name: Name, bits: u32) -> Result<BV<'ctx>, &'static str> {
        self.varmap.new_bv_with_name(self.cur_loc.func.name.clone(), name, bits)
    }

    /// Create a new `Bool` for the given `Name` (in the current function).
    /// This function performs uniquing, so if you call it twice
    /// with the same `Name`-`Function` pair, you will get two different `Bool`s.
    /// Returns the new `Bool`, or `Err` if it can't be created.
    /// (As of this writing, the only reason an `Err` might be returned is that
    /// creating the new `Bool` would exceed `max_versions_of_name` -- see
    /// [`State::new()`](struct.State.html#method.new).)
    /// Also, we assume that no two `Function`s share the same name.
    pub fn new_bool_with_name(&mut self, name: Name) -> Result<Bool<'ctx>, &'static str> {
        self.varmap.new_bool_with_name(self.cur_loc.func.name.clone(), name)
    }

    /// Record the result of `thing` to be `resultval`.
    /// Assumes `thing` is in the current function.
    /// Will fail if that would exceed `max_versions_of_name` (see [`State::new`](struct.State.html#method.new)).
    pub fn record_bv_result(&mut self, thing: &impl instruction::HasResult, resultval: BV<'ctx>) -> Result<(), &'static str> {
        let bits = size(&thing.get_type());
        let result = self.new_bv_with_name(thing.get_result().clone(), bits as u32)?;
        self.assert(&result._eq(&resultval));
        Ok(())
    }

    /// Record the result of `thing` to be `resultval`.
    /// Assumes `thing` is in the current function.
    /// Will fail if that would exceed `max_versions_of_name` (see [`State::new`](struct.State.html#method.new)).
    pub fn record_bool_result(&mut self, thing: &impl instruction::HasResult, resultval: Bool<'ctx>) -> Result<(), &'static str> {
        assert_eq!(thing.get_type(), Type::bool());
        let result = self.new_bool_with_name(thing.get_result().clone())?;
        self.assert(&result._eq(&resultval));
        Ok(())
    }

    /// Convert an `Operand` to the appropriate `BV`.
    /// Assumes the `Operand` is in the current function.
    /// (All `Operand`s should be either a constant or a variable we previously added to the state.)
    pub fn operand_to_bv(&self, op: &Operand) -> BV<'ctx> {
        match op {
            Operand::ConstantOperand(Constant::Int { bits, value }) => BV::from_u64(self.ctx, *value, *bits),
            Operand::ConstantOperand(Constant::Null(ty))
            | Operand::ConstantOperand(Constant::AggregateZero(ty))
            | Operand::ConstantOperand(Constant::Undef(ty))
                => BV::from_u64(self.ctx, 0, size(ty) as u32),
            Operand::LocalOperand { name, .. } => self.varmap.lookup_bv_var(&self.cur_loc.func.name, name).clone(),
            Operand::MetadataOperand => panic!("Can't convert {:?} to BV", op),
            _ => unimplemented!("operand_to_bv() for {:?}", op)
        }
    }

    /// Convert an `Operand` to the appropriate `Bool`.
    /// Assumes the `Operand` is in the current function.
    /// (All `Operand`s should be either a constant or a variable we previously added to the state.)
    /// This will panic if the `Operand` doesn't have type `Type::bool()`
    pub fn operand_to_bool(&self, op: &Operand) -> Bool<'ctx> {
        match op {
            Operand::ConstantOperand(Constant::Int { bits, value }) => {
                assert_eq!(*bits, 1);
                Bool::from_bool(self.ctx, *value != 0)
            },
            Operand::LocalOperand { name, .. } => self.varmap.lookup_bool_var(&self.cur_loc.func.name, name).clone(),
            op => panic!("Can't convert {:?} to Bool", op),
        }
    }

    /// Read a value `bits` bits long from memory at `addr`.
    /// Caller is responsible for ensuring that the read does not cross cell boundaries
    /// (see notes in memory.rs)
    pub fn read(&self, addr: &BV<'ctx>, bits: u32) -> BV<'ctx> {
        self.mem.read(addr, bits)
    }

    /// Write a value into memory at `addr`.
    /// Caller is responsible for ensuring that the write does not cross cell boundaries
    /// (see notes in memory.rs)
    pub fn write(&mut self, addr: &BV<'ctx>, val: BV<'ctx>) {
        self.mem.write(addr, val)
    }

    /// Allocate a value of size `bits`; return a pointer to the newly allocated object
    pub fn allocate(&mut self, bits: impl Into<u64>) -> BV<'ctx> {
        let raw_ptr = self.alloc.alloc(bits);
        BV::from_u64(self.ctx, raw_ptr, 64)
    }

    /// Record having visited the given `QualifiedBB` on the current path.
    pub fn record_in_path(&mut self, bb: QualifiedBB) {
        debug!("Recording a path entry {:?}", bb);
        self.path.push(bb);
    }

    /// Record entering a call at the given `inst` in the current location's `BasicBlock`
    pub fn push_callsite(&mut self, inst: usize) {
        self.stack.push(StackFrame {
            callsite: Callsite {
                loc: self.cur_loc.clone(),
                inst,
            },
            // TODO: taking this `restore_info` every time a callsite is pushed
            // may be expensive, and is only necessary if the call we're going
            // to make will eventually (directly or indirectly) recurse. In the
            // future we could check the LLVM 'norecurse' attribute to know when
            // this is not necessary.
            restore_info: self.varmap.get_restore_info_for_fn(self.cur_loc.func.name.clone()),
        })
    }

    /// Record leaving the current function. Returns the `Callsite` at which the
    /// current function was called, or `None` if the current function was the
    /// top-level function.
    ///
    /// Also restores the caller's local variables.
    pub fn pop_callsite(&mut self) -> Option<Callsite<'m>> {
        if let Some(StackFrame { callsite, restore_info }) = self.stack.pop() {
            self.varmap.restore_fn_vars(restore_info);
            Some(callsite)
        } else {
            None
        }

    }

    /// Save the current state, about to enter the `BasicBlock` with the given `Name` (which must be
    /// in the same `Module` and `Function` as `state.cur_loc`), as a backtracking point.
    /// The constraint will be added only if we end up backtracking to this point, and only then.
    pub fn save_backtracking_point(&mut self, bb_to_enter: Name, constraint: Bool<'ctx>) {
        debug!("Saving a backtracking point, which would enter bb {:?} with constraint {}", bb_to_enter, constraint);
        self.solver.push();
        let backtrack_loc = Location {
            module: self.cur_loc.module,
            func: self.cur_loc.func,
            bbname: bb_to_enter,
        };
        self.backtrack_points.push(BacktrackPoint {
            loc: backtrack_loc,
            prev_bb: self.cur_loc.bbname.clone(),
            stack: self.stack.clone(),
            constraint,
            varmap: self.varmap.clone(),
            mem: self.mem.clone(),
            path_len: self.path.len(),
        });
    }

    /// returns `true` if the operation was successful, or `false` if there are
    /// no saved backtracking points
    pub fn revert_to_backtracking_point(&mut self) -> bool {
        if let Some(bp) = self.backtrack_points.pop() {
            debug!("Reverting to backtracking point {}", bp);
            self.solver.pop(1);
            self.assert(&bp.constraint);
            self.varmap = bp.varmap;
            self.mem = bp.mem;
            self.stack = bp.stack;
            self.path.truncate(bp.path_len);
            self.cur_loc = bp.loc;
            self.prev_bb_name = Some(bp.prev_bb);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // we don't include tests here for Solver, Memory, Alloc, or VarMap; those are tested in their own modules.
    // Instead, here we just test the nontrivial functionality that State has itself.

    /// utility to initialize a `State` out of a `z3::Context`, a `Module`, and a `Function`
    fn blank_state<'ctx, 'm>(ctx: &'ctx z3::Context, module: &'m Module, func: &'m Function) -> State<'ctx, 'm> {
        let start_loc = Location {
            module,
            func,
            bbname: "test_bb".to_owned().into(),
        };
        State::new(ctx, start_loc, 20)
    }

    /// utility that creates a technically valid (but functionally useless) `Module` for testing
    fn blank_module(name: impl Into<String>) -> Module {
        use std::collections::HashMap;
        Module {
            name: name.into(),
            source_file_name: String::new(),
            data_layout: String::new(),
            target_triple: None,
            functions: vec![],
            global_vars: vec![],
            global_aliases: vec![],
            named_struct_types: HashMap::new(),
            inline_assembly: String::new(),
        }
    }

    /// utility that creates a technically valid (but functionally useless) `Function` for testing
    fn blank_function(name: impl Into<String>) -> Function {
        Function::new(name)
    }

    #[test]
    fn lookup_vars_via_operand() {
        let ctx = z3::Context::new(&z3::Config::new());
        let module = blank_module("test_mod");
        let func = blank_function("test_func");
        let mut state = blank_state(&ctx, &module, &func);

        // create llvm-ir names
        let valname = Name::Name("val".to_owned());
        let boolname = Name::Number(2);

        // create corresponding Z3 values
        let valvar = state.new_bv_with_name(valname.clone(), 64).unwrap();
        let boolvar = state.new_bool_with_name(boolname.clone()).unwrap();  // these clone()s wouldn't normally be necessary but we want to reuse the names to create `Operand`s later

        // check that we can look up the correct Z3 values via LocalOperands
        let valop = Operand::LocalOperand { name: valname, ty: Type::i32() };
        let boolop = Operand::LocalOperand { name: boolname, ty: Type::bool() };
        assert_eq!(state.operand_to_bv(&valop), valvar);
        assert_eq!(state.operand_to_bool(&boolop), boolvar);
    }

    #[test]
    fn const_bv() {
        let ctx = z3::Context::new(&z3::Config::new());
        let module = blank_module("test_mod");
        let func = blank_function("test_func");
        let mut state = blank_state(&ctx, &module, &func);

        // create an llvm-ir value which is constant 3
        let constint = Constant::Int { bits: 64, value: 3 };

        // this should create a corresponding Z3 value which is also constant 3
        let bv = state.operand_to_bv(&Operand::ConstantOperand(constint));

        // check that the Z3 value was evaluated to 3
        assert_eq!(state.get_a_solution_for_bv(&bv), Some(3));
    }

    #[test]
    fn const_bool() {
        let ctx = z3::Context::new(&z3::Config::new());
        let module = blank_module("test_mod");
        let func = blank_function("test_func");
        let mut state = blank_state(&ctx, &module, &func);

        // create llvm-ir constants true and false
        let consttrue = Constant::Int { bits: 1, value: 1 };
        let constfalse = Constant::Int { bits: 1, value: 0 };

        // this should create Z3 values true and false
        let bvtrue = state.operand_to_bool(&Operand::ConstantOperand(consttrue));
        let bvfalse = state.operand_to_bool(&Operand::ConstantOperand(constfalse));

        // check that the Z3 values are evaluated to true and false respectively
        assert_eq!(state.get_a_solution_for_bool(&bvtrue), Some(true));
        assert_eq!(state.get_a_solution_for_bool(&bvfalse), Some(false));

        // assert the first one, which should be true, so we should still be sat
        state.assert(&bvtrue);
        assert!(state.check());

        // assert the second one, which should be false, so we should be unsat
        state.assert(&bvfalse);
        assert!(!state.check());
    }

    #[test]
    fn backtracking() {
        let ctx = z3::Context::new(&z3::Config::new());
        let module = blank_module("test_mod");
        let func = blank_function("test_func");
        let mut state = blank_state(&ctx, &module, &func);

        // assert x > 11
        let x = BV::new_const(&ctx, "x", 64);
        state.assert(&x.bvsgt(&BV::from_i64(&ctx, 11, 64)));

        // create a backtrack point with constraint y > 5
        let y = BV::new_const(&ctx, "y", 64);
        let constraint = y.bvsgt(&BV::from_i64(&ctx, 5, 64));
        let bb = BasicBlock::new(Name::Name("bb_target".to_owned()));
        state.save_backtracking_point(bb.name.clone(), constraint);

        // check that the constraint y > 5 wasn't added: adding y < 4 should keep us sat
        assert!(state.check_with_extra_constraints(&[&y.bvslt(&BV::from_i64(&ctx, 4, 64))]));

        // assert x < 8 to make us unsat
        state.assert(&x.bvslt(&BV::from_i64(&ctx, 8, 64)));
        assert!(!state.check());

        // note the pre-rollback location
        let pre_rollback = state.cur_loc.clone();

        // roll back to backtrack point; check that we ended up at the right loc
        // and with the right prev_bb
        assert!(state.revert_to_backtracking_point());
        assert_eq!(state.cur_loc.func, pre_rollback.func);
        assert_eq!(state.cur_loc.bbname, bb.name);
        assert_eq!(state.prev_bb_name, Some("test_bb".to_owned().into()));  // the `blank_state` comes with this as the current bb name

        // check that the constraint x < 8 was removed: we're sat again
        assert!(state.check());

        // check that the constraint y > 5 was added: y evaluates to something > 5
        assert!(state.get_a_solution_for_bv(&y).unwrap() > 5);

        // check that the first constraint remained in place: x > 11
        assert!(state.get_a_solution_for_bv(&x).unwrap() > 11);

        // check that trying to backtrack again fails
        assert!(!state.revert_to_backtracking_point());
    }
}
