use llvm_ir::*;
use log::debug;
use reduce::Reduce;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use crate::alloc::Alloc;
use crate::backend::*;
use crate::config::Config;
use crate::extend::*;
use crate::global_allocations::*;
use crate::possible_solutions::*;
use crate::project::Project;
use crate::size::size;
use crate::varmap::{VarMap, RestoreInfo};

pub struct State<'ctx, 'p, B> where B: Backend<'ctx> {
    /// Reference to the Z3 context being used
    pub ctx: &'ctx z3::Context,
    /// Indicates the `BasicBlock` which is currently being executed
    pub cur_loc: Location<'p>,
    /// `Name` of the `BasicBlock` which was executed before this one;
    /// or `None` if this is the first `BasicBlock` being executed
    /// or the first `BasicBlock` of a function
    pub prev_bb_name: Option<Name>,
    /// Log of the basic blocks which have been executed to get to this point
    pub path: Vec<PathEntry>,
    /// A place where `Backend`s can put any additional state they need for
    /// themselves
    pub backend_state: Rc<RefCell<B::State>>,

    // Private members
    varmap: VarMap<'ctx, B::BV, B::Bool>,
    mem: B::Memory,
    alloc: Alloc,
    solver: B::Solver,
    global_allocations: GlobalAllocations<'ctx, 'p, B::BV>,
    /// This tracks the call stack of the symbolic execution.
    /// The first entry is the top-level caller, while the last entry is the
    /// caller of the current function.
    ///
    /// We won't have a `StackFrame` for the current function here, only each of
    /// its callers. For instance, while we are executing the top-level function,
    /// this stack will be empty.
    stack: Vec<StackFrame<'ctx, 'p, B::BV, B::Bool>>,
    /// These backtrack points are places where execution can be resumed later
    /// (efficiently, thanks to the incremental solving capabilities of Z3).
    backtrack_points: Vec<BacktrackPoint<'ctx, 'p, B>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct PathEntry {
    pub modname: String,
    pub funcname: String,
    pub bbname: Name,
}

impl fmt::Debug for PathEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pretty_name = match self.bbname {
            Name::Name(ref s) => format!("{:?}", s),
            Name::Number(n) => format!("%{}", n),
        };
        write!(f, "{{{}: {} {}}}", self.modname, self.funcname, pretty_name)
    }
}

#[derive(Clone)]
pub struct Location<'p> {
    pub module: &'p Module,
    pub func: &'p Function,
    pub bbname: Name,
}

/// Implementation of `PartialEq` assumes that module and function names are unique
impl<'p> PartialEq for Location<'p> {
    fn eq(&self, other: &Self) -> bool {
        self.module.name == other.module.name
            && self.func.name == other.func.name
            && self.bbname == other.bbname
    }
}

/// Our implementation of `PartialEq` satisfies the requirements of `Eq`
impl<'p> Eq for Location<'p> {}

impl<'p> fmt::Debug for Location<'p> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Location: module {:?}, func {:?}, bb {:?}", self.module.name, self.func.name, self.bbname)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Callsite<'p> {
    /// `Module`, `Function`, and `BasicBlock` of the callsite
    pub loc: Location<'p>,
    /// Index of the `Call` instruction within the `BasicBlock`
    pub inst: usize,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct StackFrame<'ctx, 'p, V, B> where V: BV<'ctx, AssociatedBool = B>, B: Bool<'ctx, AssociatedBV = V> {
    /// Indicates the call instruction which was responsible for the call
    callsite: Callsite<'p>,
    /// Caller's local variables, so they can be restored when we return to the caller.
    /// This is necessary in the case of (direct or indirect) recursion.
    /// See notes on `VarMap.get_restore_info_for_fn()`.
    restore_info: RestoreInfo<'ctx, V, B>,
}

struct BacktrackPoint<'ctx, 'p, B> where B: Backend<'ctx> {
    /// Where to resume execution
    loc: Location<'p>,
    /// `Name` of the `BasicBlock` executed just prior to the `BacktrackPoint`.
    /// Assumed to be in the same `Module` and `Function` as `loc` (which is
    /// always true for how we currently use `BacktrackPoint`s as of this writing)
    prev_bb: Name,
    /// Call stack at the `BacktrackPoint`.
    /// This is a vector of `StackFrame`s where the first entry is the top-level
    /// caller, and the last entry is the caller of the `BacktrackPoint`'s function.
    stack: Vec<StackFrame<'ctx, 'p, B::BV, B::Bool>>,
    /// Constraint to add before restarting execution at `next_bb`.
    /// (Intended use of this is to constrain the branch in that direction.)
    constraint: B::Bool,
    /// `VarMap` representing the state of things at the `BacktrackPoint`.
    /// For now, we require making a full copy of the `VarMap` in order to revert
    /// later.
    varmap: VarMap<'ctx, B::BV, B::Bool>,
    /// `Memory` representing the state of things at the `BacktrackPoint`.
    /// Copies of a `Memory` should be cheap (just a Z3 object pointer), so it's
    /// not a huge concern that we need a full copy here in order to revert later.
    mem: B::Memory,
    /// The length of `path` at the `BacktrackPoint`.
    /// If we ever revert to this `BacktrackPoint`, we will truncate the `path` to
    /// its first `path_len` entries.
    path_len: usize,
    /// The backend state at the `BacktrackPoint`.
    backend_state: B::State,
}

impl<'ctx, 'p, B> fmt::Display for BacktrackPoint<'ctx, 'p, B> where B: Backend<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<BacktrackPoint to execute bb {:?} with constraint {:?} and {} frames on the callstack>", self.loc.bbname, self.constraint, self.stack.len())
    }
}

impl<'ctx, 'p, B> State<'ctx, 'p, B> where B: Backend<'ctx> {
    /// `start_loc`: the `Location` where the `State` should begin executing.
    /// As of this writing, `start_loc` should be the entry point of a
    /// function, or you will have problems.
    pub fn new(
        ctx: &'ctx z3::Context,
        project: &'p Project,
        start_loc: Location<'p>,
        config: &Config<'ctx, B>,
    ) -> Self {
        let backend_state = Rc::new(RefCell::new(B::State::default()));
        let mut state = Self {
            ctx,
            cur_loc: start_loc.clone(),
            prev_bb_name: None,
            path: Vec::new(),
            varmap: VarMap::new(ctx, config.loop_bound),
            mem: Memory::new_uninitialized(ctx, backend_state.clone()),
            alloc: Alloc::new(),
            solver: B::Solver::new(ctx, backend_state.clone()),
            global_allocations: GlobalAllocations::new(),
            stack: Vec::new(),
            backtrack_points: Vec::new(),

            // listed last (out-of-order) so that it can be used above but moved in now
            backend_state,
        };
        // Here we do allocation and initialization of the global variables in the Project.
        // We need to do these in two separate passes - first allocating them
        // all, then initializing them all - because initializers can refer to
        // the addresses of other global variables, potentially even circularly.
        //
        // Note that `project.all_global_vars()` gives us both global variable
        // *definitions* and *declarations*; we can distinguish these because
        // (direct quote from the LLVM docs) "Definitions have initializers,
        // declarations don't." This implies that even globals without an
        // initializer in C have one in LLVM, which seems weird to me, but it's
        // what the docs say, and also matches what I've seen empirically.
        debug!("Allocating global variables");
        for (var, module) in project.all_global_vars().filter(|(var,_)| var.initializer.is_some()) {
            // Allocate the global variable.
            //
            // In the allocation pass, we want to process each global variable
            // exactly once, and the order doesn't matter, so we simply process
            // definitions, since each global variable must have exactly one
            // definition. Hence the `filter()` above.
            if let Type::PointerType { pointee_type, .. } = &var.ty {
                let addr = state.allocate(size(&*pointee_type) as u64);
                debug!("Allocated {:?} at {:?}", var.name, addr);
                state.global_allocations.allocate_global_var(var, module, addr);
            } else {
                panic!("Global variable has non-pointer type {:?}", &var.ty);
            }
        }
        // We also have to allocate (at least a tiny bit of) memory for each
        // `Function`, just so that we can have pointers to those `Function`s.
        // The `state.addr_to_function` map will help in interpreting these
        // function pointers.
        debug!("Allocating functions");
        for (func, module) in project.all_functions() {
            let addr: u64 = state.alloc.alloc(64 as u64);  // we just allocate 64 bits for each function. No reason to allocate more.
            let addr_bv = BV::from_u64(state.ctx, addr, 64);
            debug!("Allocated {:?} at {:?}", func.name, addr_bv);
            state.global_allocations.allocate_function(func, module, addr, addr_bv);
       }
        // Now we do initialization of global variables.
        debug!("Initializing global variables");
        for (var, module) in project.all_global_vars() {
            // Like with the allocation pass, in the initialization pass we
            // again only need to process definitions. Conveniently, definitions
            // are where we find the initializer anyway; so we initialize the
            // variable if and only if there is an initializer. (See notes on
            // definitions vs. declarations above.) Also, processing only
            // definitions ensures that we only initialize each variable once.
            //
            // We assume that global-variable initializers can only refer to the
            // *addresses* of other globals, and not the *values* of other
            // global constants, so that it's fine that any referred-to globals
            // may have been allocated but not initialized at this point (since
            // their definition may not have been processed yet).
            // This assumption seems to hold empirically: in my tests,
            // (1) clang performs constant-folding, even at -O0, on global
            //     variable initializers so that these initializers do not refer to
            //     the values of other global constants at the LLVM level. For
            //     instance, the C code
            //       `const int a = 1; const int b = a + 3;`
            //     is translated into the LLVM equivalent of
            //       `const int a = 1; const int b = 4;`
            // (2) clang rejects programs where global variable initializers refer
            //     to the value of externally-defined global constants, in which
            //     case the constant-folding described above would be impossible.
            //     Note, however, that clang does allow referring to the *addresses*
            //     of externally-defined global variables.
            if let Some(ref initial_val) = var.initializer {
                debug!("Initializing {:?} with initializer {:?}", var.name, initial_val);
                let addr = state.global_allocations
                    .get_global_address(&var.name, module)
                    .unwrap_or_else(|| panic!("Trying to initialize global variable {:?} in module {:?} but failed to find its allocated address", var.name, &module.name));
                state.cur_loc.module = module;  // have to do this prior to call to state.const_to_bv(), to ensure the correct module is used for resolution of references to other globals
                state.mem.write(&addr, state.const_to_bv(initial_val));
            }
        }
        debug!("Done allocating and initializing global variables");
        state.cur_loc = start_loc;  // reset any changes we made above
        state
    }

    /// Add `cond` as a constraint, i.e., assert that `cond` must be true
    pub fn assert(&mut self, cond: &B::Bool) {
        self.solver.assert(cond)
    }

    /// Returns `true` if current constraints are satisfiable, `false` if not.
    ///
    /// Returns `None` if the query failed (e.g., was interrupted or timed out).
    ///
    /// With the default `Z3Backend`, this function caches its result and will
    /// only call to Z3 if constraints have changed since the last call to
    /// `check()`.
    pub fn check(&mut self) -> Result<bool, &'static str> {
        self.solver.check()
    }

    /// Returns `true` if the current constraints plus the additional constraints `conds`
    /// are together satisfiable, or `false` if not.
    ///
    /// Returns `None` if the query failed (e.g., was interrupted or timed out).
    ///
    /// Does not permanently add the constraints in `conds` to the solver.
    pub fn check_with_extra_constraints<'a>(&'a mut self, conds: impl Iterator<Item = &'a B::Bool>) -> Result<bool, &'static str> {
        self.solver.check_with_extra_constraints(conds)
    }

    /// Get one possible concrete value for the `BV`.
    /// Returns `Ok(None)` if no possible solution, or `Err` if solver query failed.
    pub fn get_a_solution_for_bv(&mut self, bv: &B::BV) -> Result<Option<u64>, &'static str> {
        self.solver.get_a_solution_for_bv(bv)
    }

    /// Get one possible concrete value for the `Bool`.
    /// Returns `Ok(None)` if no possible solution, or `Err` if solver query failed.
    pub fn get_a_solution_for_bool(&mut self, b: &B::Bool) -> Result<Option<bool>, &'static str> {
        self.solver.get_a_solution_for_bool(b)
    }

    /// Get one possible concrete value for the given IR `Name` (from the given `Function` name), which represents a bitvector.
    /// Returns `Ok(None)` if no possible solution, or `Err` if solver query failed.
    pub fn get_a_solution_for_bv_by_irname(&mut self, funcname: &String, name: &Name) -> Result<Option<u64>, &'static str> {
        let bv = self.varmap.lookup_bv_var(funcname, name).clone();  // clone() so that the borrow of self is released
        self.get_a_solution_for_bv(&bv)
    }

    /// Get one possible concrete value for the given IR `Name` (from the given `Function` name), which represents a bool.
    /// Returns `Ok(None)` if no possible solution, or `Err` if solver query failed.
    pub fn get_a_solution_for_bool_by_irname(&mut self, funcname: &String, name: &Name) -> Result<Option<bool>, &'static str> {
        let b = self.varmap.lookup_bool_var(funcname, name).clone();  // clone() so that the borrow of self is released
        self.get_a_solution_for_bool(&b)
    }

    /// Get a description of the possible solutions for the `BV`.
    ///
    /// `n`: Maximum number of distinct solutions to return.
    /// If there are more than `n` possible solutions, this simply
    /// returns `PossibleSolutions::MoreThanNPossibleSolutions(n)`.
    pub fn get_possible_solutions_for_bv(&mut self, bv: &B::BV, n: usize) -> Result<PossibleSolutions<u64>, &'static str> {
        self.solver.get_possible_solutions_for_bv(bv, n)
    }

    /// Get a description of the possible solutions for the `Bool`.
    pub fn get_possible_solutions_for_bool(&mut self, b: &B::Bool) -> Result<PossibleSolutions<bool>, &'static str> {
        self.solver.get_possible_solutions_for_bool(b)
    }

    /// Get a description of the possible solutions for the given IR `Name` (from the given `Function` name), which represents a bitvector.
    ///
    /// `n`: Maximum number of distinct solutions to return.
    /// If there are more than `n` possible solutions, this simply
    /// returns `PossibleSolutions::MoreThanNPossibleSolutions(n)`.
    pub fn get_possible_solutions_for_bv_by_irname(&mut self, funcname: &String, name: &Name, n: usize) -> Result<PossibleSolutions<u64>, &'static str> {
        let bv = self.varmap.lookup_bv_var(funcname, name).clone();  // clone() so that the borrow of self is released
        self.get_possible_solutions_for_bv(&bv, n)
    }

    /// Get a description of the possible solutions for the given IR `Name` (from the given `Function` name), which represents a bool.
    pub fn get_possible_solutions_for_bool_by_irname(&mut self, funcname: &String, name: &Name) -> Result<PossibleSolutions<bool>, &'static str> {
        let b = self.varmap.lookup_bool_var(funcname, name).clone();  // clone() so that the borrow of self is released
        self.get_possible_solutions_for_bool(&b)
    }

    /// Create a new (unconstrained) `BV` for the given `Name` (in the current function).
    ///
    /// This function performs uniquing, so if you call it twice
    /// with the same `Name`-`Function` pair, you will get two different `BV`s.
    ///
    /// Returns the new `BV`, or `Err` if it can't be created.
    /// (As of this writing, the only reason an `Err` might be returned is that
    /// creating the new `BV` would exceed `max_versions_of_name` -- see
    /// [`State::new()`](struct.State.html#method.new).)
    ///
    /// Also, we assume that no two `Function`s share the same name.
    pub fn new_bv_with_name(&mut self, name: Name, bits: u32) -> Result<B::BV, &'static str> {
        self.varmap.new_bv_with_name(self.cur_loc.func.name.clone(), name, bits)
    }

    /// Create a new (unconstrained) `Bool` for the given `Name` (in the current function).
    ///
    /// This function performs uniquing, so if you call it twice
    /// with the same `Name`-`Function` pair, you will get two different `Bool`s.
    ///
    /// Returns the new `Bool`, or `Err` if it can't be created.
    /// (As of this writing, the only reason an `Err` might be returned is that
    /// creating the new `Bool` would exceed `max_versions_of_name` -- see
    /// [`State::new()`](struct.State.html#method.new).)
    ///
    /// Also, we assume that no two `Function`s share the same name.
    pub fn new_bool_with_name(&mut self, name: Name) -> Result<B::Bool, &'static str> {
        self.varmap.new_bool_with_name(self.cur_loc.func.name.clone(), name)
    }

    /// Assign the given `BV` to the given `Name` (in the current function).
    ///
    /// This function performs uniquing, so it creates a new version of the
    /// variable represented by the `(String, Name)` pair rather than overwriting
    /// the current version.
    ///
    /// Returns `Err` if the assignment can't be performed.
    /// (As of this writing, the only reason an `Err` might be returned is that
    /// creating the new version of the `BV` would exceed `max_versions_of_name`
    /// -- see [`State::new()`](struct.State.html#method.new).)
    pub fn assign_bv_to_name(&mut self, name: Name, bv: B::BV) -> Result<(), &'static str> {
        self.varmap.assign_bv_to_name(self.cur_loc.func.name.clone(), name, bv)
    }

    /// Assign the given `Bool` to the given `Name` (in the current function).
    ///
    /// This function performs uniquing, so it creates a new version of the
    /// variable represented by the `(String, Name)` pair rather than overwriting
    /// the current version.
    ///
    /// Returns `Err` if the assignment can't be performed.
    /// (As of this writing, the only reason an `Err` might be returned is that
    /// creating the new version of the `Bool` would exceed `max_versions_of_name`
    /// -- see [`State::new()`](struct.State.html#method.new).)
    pub fn assign_bool_to_name(&mut self, name: Name, b: B::Bool) -> Result<(), &'static str> {
        self.varmap.assign_bool_to_name(self.cur_loc.func.name.clone(), name, b)
    }

    /// Record the result of `thing` to be `resultval`.
    /// Assumes `thing` is in the current function.
    /// Will fail if that would exceed `max_versions_of_name` (see [`State::new`](struct.State.html#method.new)).
    pub fn record_bv_result(&mut self, thing: &impl instruction::HasResult, resultval: B::BV) -> Result<(), &'static str> {
        self.assign_bv_to_name(thing.get_result().clone(), resultval)
    }

    /// Record the result of `thing` to be `resultval`.
    /// Assumes `thing` is in the current function.
    /// Will fail if that would exceed `max_versions_of_name` (see [`State::new`](struct.State.html#method.new)).
    pub fn record_bool_result(&mut self, thing: &impl instruction::HasResult, resultval: B::Bool) -> Result<(), &'static str> {
        self.assign_bool_to_name(thing.get_result().clone(), resultval)
    }

    /// Overwrite the latest version of the given `Name` to instead be `bv`.
    /// Assumes `Name` is in the current function.
    pub fn overwrite_latest_version_of_bv(&mut self, name: &Name, bv: B::BV) {
        self.varmap.overwrite_latest_version_of_bv(&self.cur_loc.func.name, name, bv)
    }

    /// Overwrite the latest version of the given `Name` to instead be `b`.
    /// Assumes `Name` is in the current function.
    pub fn overwrite_latest_version_of_bool(&mut self, name: &Name, b: B::Bool) {
        self.varmap.overwrite_latest_version_of_bool(&self.cur_loc.func.name, name, b)
    }

    /// Convert an `Operand` to the appropriate `BV`.
    /// Assumes the `Operand` is in the current function.
    /// (All `Operand`s should be either a constant or a variable we previously added to the state.)
    pub fn operand_to_bv(&self, op: &Operand) -> B::BV {
        match op {
            Operand::ConstantOperand(c) => self.const_to_bv(c),
            Operand::LocalOperand { name, .. } => self.varmap.lookup_bv_var(&self.cur_loc.func.name, name).clone(),
            Operand::MetadataOperand => panic!("Can't convert {:?} to BV", op),
        }
    }

    /// Convert a `Constant` to the appropriate `BV`.
    fn const_to_bv(&self, c: &Constant) -> B::BV {
        match c {
            Constant::Int { bits, value } => BV::from_u64(self.ctx, *value, *bits),
            Constant::Null(ty) | Constant::AggregateZero(ty) | Constant::Undef(ty)
                => BV::from_u64(self.ctx, 0, size(ty) as u32),
            Constant::Struct { values, .. } => values.iter().map(|c| self.const_to_bv(c)).reduce(|a,b| b.concat(&a)).unwrap(),
            Constant::Array { elements, .. } => elements.iter().map(|c| self.const_to_bv(c)).reduce(|a,b| b.concat(&a)).unwrap(),
            Constant::Vector(elements) => elements.iter().map(|c| self.const_to_bv(c)).reduce(|a,b| b.concat(&a)).unwrap(),
            Constant::GlobalReference { name, .. } => {
                if let Some(addr) = self.global_allocations.get_global_address(name, self.cur_loc.module) {
                    addr.clone()
                } else if let Some(alias) = self.cur_loc.module.global_aliases.iter().find(|a| &a.name == name) {
                    self.const_to_bv(&alias.aliasee)
                } else {
                    panic!("const_to_bv: GlobalReference to {:?} which was not found (current module is {:?})", name, &self.cur_loc.module.name)
                }
            },
            Constant::Add(a) => self.const_to_bv(&a.operand0).add(&self.const_to_bv(&a.operand1)),
            Constant::Sub(s) => self.const_to_bv(&s.operand0).sub(&self.const_to_bv(&s.operand1)),
            Constant::Mul(m) => self.const_to_bv(&m.operand0).mul(&self.const_to_bv(&m.operand1)),
            Constant::UDiv(u) => self.const_to_bv(&u.operand0).udiv(&self.const_to_bv(&u.operand1)),
            Constant::SDiv(s) => self.const_to_bv(&s.operand0).sdiv(&self.const_to_bv(&s.operand1)),
            Constant::URem(u) => self.const_to_bv(&u.operand0).urem(&self.const_to_bv(&u.operand1)),
            Constant::SRem(s) => self.const_to_bv(&s.operand0).srem(&self.const_to_bv(&s.operand1)),
            Constant::And(a) => self.const_to_bv(&a.operand0).and(&self.const_to_bv(&a.operand1)),
            Constant::Or(o) => self.const_to_bv(&o.operand0).or(&self.const_to_bv(&o.operand1)),
            Constant::Xor(x) => self.const_to_bv(&x.operand0).xor(&self.const_to_bv(&x.operand1)),
            Constant::Shl(s) => self.const_to_bv(&s.operand0).shl(&self.const_to_bv(&s.operand1)),
            Constant::LShr(s) => self.const_to_bv(&s.operand0).lshr(&self.const_to_bv(&s.operand1)),
            Constant::AShr(s) => self.const_to_bv(&s.operand0).ashr(&self.const_to_bv(&s.operand1)),
            Constant::ExtractElement(ee) => match &ee.index {
                Constant::Int { value: index, .. } => match &ee.vector {
                    Constant::Vector(els) => self.const_to_bv(&els.get(*index as usize).expect("Constant::ExtractElement index out of range")),
                    c => panic!("Expected ExtractElement.vector to be a Constant::Vector, got {:?}", c),
                },
                index => unimplemented!("ExtractElement.index is not a Constant::Int, instead it is {:?}", index),
            },
            Constant::InsertElement(ie) => match &ie.index {
                Constant::Int { value: index, .. } => match &ie.vector {
                    Constant::Vector(els) => {
                        let mut els = els.clone();
                        *els.get_mut(*index as usize).expect("Constant::InsertElement index out of range") = ie.element.clone();
                        self.const_to_bv(&Constant::Vector(els))
                    },
                    c => panic!("Expected InsertElement.vector to be a Constant::Vector, got {:?}", c),
                },
                index => unimplemented!("InsertElement.index is not a Constant::Int, instead it is {:?}", index),
            }
            Constant::ExtractValue(ev) => self.const_to_bv(Self::simplify_const_ev(&ev.aggregate, ev.indices.iter().copied())),
            Constant::InsertValue(iv) => self.const_to_bv(&Self::simplify_const_iv(iv.aggregate.clone(), iv.element.clone(), iv.indices.iter().copied())),
            Constant::GetElementPtr(gep) => {
                // heavily inspired by `ExecutionManager::symex_gep()` in symex.rs. TODO could try to share more code
                let z3base = self.const_to_bv(&gep.address);
                let offset = self.get_offset(gep.indices.iter(), &gep.address.get_type(), z3base.get_size());
                z3base.add(&offset).simplify()
            },
            Constant::Trunc(t) => self.const_to_bv(&t.operand).extract(size(&t.to_type) as u32 - 1, 0),
            Constant::ZExt(z) => zero_extend_to_bits(self.const_to_bv(&z.operand), size(&z.to_type) as u32),
            Constant::SExt(s) => sign_extend_to_bits(self.const_to_bv(&s.operand), size(&s.to_type) as u32),
            Constant::PtrToInt(pti) => {
                let bv = self.const_to_bv(&pti.operand);
                assert_eq!(bv.get_size(), size(&pti.to_type) as u32);
                bv  // just a cast, it's the same bits underneath
            },
            Constant::IntToPtr(itp) => {
                let bv = self.const_to_bv(&itp.operand);
                assert_eq!(bv.get_size(), size(&itp.to_type) as u32);
                bv  // just a cast, it's the same bits underneath
            },
            Constant::BitCast(bc) => {
                let bv = self.const_to_bv(&bc.operand);
                assert_eq!(bv.get_size(), size(&bc.to_type) as u32);
                bv  // just a cast, it's the same bits underneath
            },
            Constant::AddrSpaceCast(ac) => {
                let bv = self.const_to_bv(&ac.operand);
                assert_eq!(bv.get_size(), size(&ac.to_type) as u32);
                bv  // just a cast, it's the same bits underneath
            },
            Constant::Select(s) => {
                let b = self.const_to_bool(&s.condition).simplify().as_bool().expect("Constant::Select: Expected a constant condition");
                if b {
                    self.const_to_bv(&s.true_value)
                } else {
                    self.const_to_bv(&s.false_value)
                }
            },
            _ => unimplemented!("const_to_bv for {:?}", c),
        }
    }

    /// Given a `Constant::Struct` and a series of `ExtractValue` indices, get the
    /// final `Constant` referred to
    fn simplify_const_ev(s: &Constant, mut indices: impl Iterator<Item = u32>) -> &Constant {
        match indices.next() {
            None => s,
            Some(index) => {
                if let Constant::Struct { values, .. } = s {
                    let val = values.get(index as usize).expect("Constant::ExtractValue index out of range");
                    Self::simplify_const_ev(val, indices)
                } else {
                    panic!("simplify_const_ev: not a Constant::Struct: {:?}", s)
                }
            }
        }

    }

    /// Given a `Constant::Struct`, a value to insert, and a series of
    /// `InsertValue` indices, get the final `Constant` referred to
    fn simplify_const_iv(s: Constant, val: Constant, mut indices: impl Iterator<Item = u32>) -> Constant {
        match indices.next() {
            None => val,
            Some(index) => {
                if let Constant::Struct { name, mut values, is_packed } = s {
                    let to_replace = values.get(index as usize).expect("Constant::InsertValue index out of range").clone();
                    values[index as usize] = Self::simplify_const_iv(to_replace, val, indices);
                    Constant::Struct { name, values, is_packed }
                } else {
                    panic!("simplify_const_iv: not a Constant::Struct: {:?}", s)
                }
            }
        }
    }

    /// Get the offset of the element (in bytes, as a `BV` of `result_bits` bits)
    //
    // Heavily inspired by `ExecutionManager::get_offset()` in symex.rs. TODO could try to share more code
    fn get_offset(&self, mut indices: impl Iterator<Item = &'p Constant>, base_type: &Type, result_bits: u32) -> B::BV {
        match indices.next() {
            None => BV::from_u64(self.ctx, 0, result_bits),
            Some(index) => match base_type {
                Type::PointerType { pointee_type, .. }
                | Type::ArrayType { element_type: pointee_type, .. }
                | Type::VectorType { element_type: pointee_type, .. }
                => {
                    let el_size_bits = size(pointee_type) as u64;
                    if el_size_bits % 8 != 0 {
                        unimplemented!("Type with size {} bits", el_size_bits);
                    }
                    let el_size_bytes = el_size_bits / 8;
                    zero_extend_to_bits(self.const_to_bv(index), result_bits)
                        .mul(&B::BV::from_u64(self.ctx, el_size_bytes, result_bits))
                        .add(&self.get_offset(indices, pointee_type, result_bits))
                },
                Type::StructType { element_types, .. } => match index {
                    Constant::Int { value: index, .. } => {
                        let mut offset_bits = 0;
                        for ty in element_types.iter().take(*index as usize) {
                            offset_bits += size(ty) as u64;
                        }
                        if offset_bits % 8 != 0 {
                            unimplemented!("Struct offset of {} bits", offset_bits);
                        }
                        let offset_bytes = offset_bits / 8;
                        B::BV::from_u64(self.ctx, offset_bytes, result_bits)
                            .add(&self.get_offset(indices, &element_types[*index as usize], result_bits))
                    },
                    _ => panic!("Can't get_offset from struct type with index {:?}", index),
                },
                Type::NamedStructType { ty, .. } => {
                    let rc: Rc<RefCell<Type>> = ty.as_ref()
                        .expect("get_offset on an opaque struct type")
                        .upgrade()
                        .expect("Failed to upgrade weak reference");
                    let actual_ty: &Type = &rc.borrow();
                    if let Type::StructType { ref element_types, .. } = actual_ty {
                        // this code copied from the StructType case, unfortunately
                        match index {
                            Constant::Int { value: index, .. } => {
                                let mut offset_bits = 0;
                                for ty in element_types.iter().take(*index as usize) {
                                    offset_bits += size(ty) as u64;
                                }
                                if offset_bits % 8 != 0 {
                                    unimplemented!("Struct offset of {} bits", offset_bits);
                                }
                                let offset_bytes = offset_bits / 8;
                                B::BV::from_u64(self.ctx, offset_bytes, result_bits)
                                    .add(&self.get_offset(indices, &element_types[*index as usize], result_bits))
                            },
                            _ => panic!("Can't get_offset from struct type with index {:?}", index),
                        }
                    } else {
                        panic!("Expected NamedStructType inner type to be a StructType, but got {:?}", actual_ty)
                    }
                }
                _ => panic!("get_offset with base type {:?}", base_type),
            }
        }
    }

    /// Convert an `Operand` to the appropriate `Bool`.
    /// Assumes the `Operand` is in the current function.
    /// (All `Operand`s should be either a constant or a variable we previously added to the state.)
    /// This will panic if the `Operand` doesn't have type `Type::bool()`
    pub fn operand_to_bool(&self, op: &Operand) -> B::Bool {
        match op {
            Operand::ConstantOperand(c) => self.const_to_bool(c),
            Operand::LocalOperand { name, .. } => self.varmap.lookup_bool_var(&self.cur_loc.func.name, name).clone(),
            op => panic!("Can't convert {:?} to Bool", op),
        }
    }

    /// Convert a `Constant` to the appropriate `Bool`.
    fn const_to_bool(&self, c: &Constant) -> B::Bool {
        match c {
            Constant::Int { bits, value } => {
                assert_eq!(*bits, 1);
                B::Bool::from_bool(self.ctx, *value != 0)
            },
            Constant::And(a) => self.const_to_bool(&a.operand0).and(&[&self.const_to_bool(&a.operand1)]),
            Constant::Or(o) => self.const_to_bool(&o.operand0).or(&[&self.const_to_bool(&o.operand1)]),
            Constant::Xor(x) => self.const_to_bool(&x.operand0).xor(&self.const_to_bool(&x.operand1)),
            Constant::ICmp(icmp) => {
                let bv0 = self.const_to_bv(&icmp.operand0);
                let bv1 = self.const_to_bv(&icmp.operand1);
                match icmp.predicate {
                    IntPredicate::EQ => bv0._eq(&bv1),
                    IntPredicate::NE => bv0._eq(&bv1).not(),
                    IntPredicate::UGT => bv0.ugt(&bv1),
                    IntPredicate::UGE => bv0.uge(&bv1),
                    IntPredicate::ULT => bv0.ult(&bv1),
                    IntPredicate::ULE => bv0.ule(&bv1),
                    IntPredicate::SGT => bv0.sgt(&bv1),
                    IntPredicate::SGE => bv0.sge(&bv1),
                    IntPredicate::SLT => bv0.slt(&bv1),
                    IntPredicate::SLE => bv0.sle(&bv1),
                }
            },
            Constant::Select(s) => {
                let b = self.const_to_bool(&s.condition).simplify().as_bool().expect("Constant::Select: Expected a constant condition");
                if b {
                    self.const_to_bool(&s.true_value)
                } else {
                    self.const_to_bool(&s.false_value)
                }
            },
            _ => unimplemented!("const_to_bool for {:?}", c),
        }
    }

    /// Given a `BV`, interpret it as a function pointer, and return a
    /// description of the possible `Function`s which it would point to.
    ///
    /// `n`: Maximum number of distinct `Function`s to return.
    /// If there are more than `n` possible `Function`s, this simply returns
    /// `PossibleSolutions::MoreThanNPossibleSolutions(n)`.
    ///
    /// Returns `Err` if the solver query fails or if it finds that it is
    /// possible that the `BV` points to something that's not a `Function` in
    /// the `Project`.
    pub fn interpret_as_function_ptr(&mut self, bv: B::BV, n: usize) -> Result<PossibleSolutions<&'p Function>, String> {
        if n == 0 {
            unimplemented!("n == 0 in interpret_as_function_ptr")
        }
        // First try to interpret without a full solve (i.e., with `as_u64()`)
        match bv.as_u64().and_then(|addr| self.global_allocations.get_func_for_address(addr, self.cur_loc.module)) {
            Some(f) => Ok(PossibleSolutions::PossibleSolutions(vec![f])),  // there is only one possible solution, and it's this `f`
            None => {
                match self.get_possible_solutions_for_bv(&bv, n)? {
                    PossibleSolutions::MoreThanNPossibleSolutions(n) => Ok(PossibleSolutions::MoreThanNPossibleSolutions(n)),
                    PossibleSolutions::PossibleSolutions(v) => {
                        v.into_iter()
                            .map(|addr| self.global_allocations.get_func_for_address(addr, self.cur_loc.module)
                                .ok_or_else(|| format!("This BV can't be interpreted as a function pointer: it has a possible solution 0x{:x} which points to something that's not a function.\n  The BV was: {:?}", addr, bv))
                            )
                            .collect::<Vec<Result<_,_>>>()
                            .into_iter()
                            .collect::<Result<Vec<_>,_>>()
                            .map(PossibleSolutions::PossibleSolutions)
                    }
                }
            }
        }
    }

    /// Read a value `bits` bits long from memory at `addr`.
    /// Caller is responsible for ensuring that the read does not cross cell boundaries
    /// (see notes in memory.rs)
    pub fn read(&self, addr: &B::BV, bits: u32) -> B::BV {
        self.mem.read(addr, bits)
    }

    /// Write a value into memory at `addr`.
    /// Caller is responsible for ensuring that the write does not cross cell boundaries
    /// (see notes in memory.rs)
    pub fn write(&mut self, addr: &B::BV, val: B::BV) {
        self.mem.write(addr, val)
    }

    /// Allocate a value of size `bits`; return a pointer to the newly allocated object
    pub fn allocate(&mut self, bits: impl Into<u64>) -> B::BV {
        let raw_ptr = self.alloc.alloc(bits);
        BV::from_u64(self.ctx, raw_ptr, 64)
    }

    /// Record a `PathEntry` in the current path.
    pub fn record_in_path(&mut self, entry: PathEntry) {
        debug!("Recording a path entry {:?}", entry);
        self.path.push(entry);
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
    pub fn pop_callsite(&mut self) -> Option<Callsite<'p>> {
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
    pub fn save_backtracking_point(&mut self, bb_to_enter: Name, constraint: B::Bool) {
        debug!("Saving a backtracking point, which would enter bb {:?} with constraint {:?}", bb_to_enter, constraint);
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
            backend_state: self.backend_state.borrow().clone(),
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
            *self.backend_state.borrow_mut() = bp.backend_state;
            self.cur_loc = bp.loc;
            self.prev_bb_name = Some(bp.prev_bb);
            true
        } else {
            false
        }
    }

    /// returns the number of saved backtracking points
    pub fn count_backtracking_points(&self) -> usize {
        self.backtrack_points.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // we don't include tests here for Solver, Memory, Alloc, or VarMap; those are tested in their own modules.
    // Instead, here we just test the nontrivial functionality that State has itself.

    /// utility to initialize a `State` out of a `z3::Context`, a `Project`, and a function name
    fn blank_state<'ctx, 'p>(ctx: &'ctx z3::Context, project: &'p Project, funcname: &str) -> State<'ctx, 'p, Z3Backend<'ctx>> {
        let (func, module) = project.get_func_by_name(funcname).expect("Failed to find function");
        let start_loc = Location {
            module,
            func,
            bbname: "test_bb".to_owned().into(),
        };
        State::new(ctx, project, start_loc, &Config::default())
    }

    /// Utility that creates a simple `Project` for testing.
    /// The `Project` will contain a single `Module` (with the given name) which contains
    /// a single function (given).
    fn blank_project(modname: impl Into<String>, func: Function) -> Project {
        Project::from_module(Module {
            name: modname.into(),
            source_file_name: String::new(),
            data_layout: String::new(),
            target_triple: None,
            functions: vec![func],
            global_vars: vec![],
            global_aliases: vec![],
            named_struct_types: HashMap::new(),
            inline_assembly: String::new(),
        })
    }

    /// utility that creates a technically valid (but functionally useless) `Function` for testing
    fn blank_function(name: impl Into<String>) -> Function {
        Function::new(name)
    }

    #[test]
    fn lookup_vars_via_operand() {
        let ctx = z3::Context::new(&z3::Config::new());
        let func = blank_function("test_func");
        let project = blank_project("test_mod", func);
        let mut state = blank_state(&ctx, &project, "test_func");

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
        let func = blank_function("test_func");
        let project = blank_project("test_mod", func);
        let mut state = blank_state(&ctx, &project, "test_func");

        // create an llvm-ir value which is constant 3
        let constint = Constant::Int { bits: 64, value: 3 };

        // this should create a corresponding Z3 value which is also constant 3
        let bv = state.operand_to_bv(&Operand::ConstantOperand(constint));

        // check that the Z3 value was evaluated to 3
        assert_eq!(state.get_a_solution_for_bv(&bv), Ok(Some(3)));
    }

    #[test]
    fn const_bool() {
        let ctx = z3::Context::new(&z3::Config::new());
        let func = blank_function("test_func");
        let project = blank_project("test_mod", func);
        let mut state = blank_state(&ctx, &project, "test_func");

        // create llvm-ir constants true and false
        let consttrue = Constant::Int { bits: 1, value: 1 };
        let constfalse = Constant::Int { bits: 1, value: 0 };

        // this should create Z3 values true and false
        let bvtrue = state.operand_to_bool(&Operand::ConstantOperand(consttrue));
        let bvfalse = state.operand_to_bool(&Operand::ConstantOperand(constfalse));

        // check that the Z3 values are evaluated to true and false respectively
        assert_eq!(state.get_a_solution_for_bool(&bvtrue), Ok(Some(true)));
        assert_eq!(state.get_a_solution_for_bool(&bvfalse), Ok(Some(false)));

        // assert the first one, which should be true, so we should still be sat
        state.assert(&bvtrue);
        assert_eq!(state.check(), Ok(true));

        // assert the second one, which should be false, so we should be unsat
        state.assert(&bvfalse);
        assert_eq!(state.check(), Ok(false));
    }

    #[test]
    fn backtracking() {
        let ctx = z3::Context::new(&z3::Config::new());
        let func = blank_function("test_func");
        let project = blank_project("test_mod", func);
        let mut state = blank_state(&ctx, &project, "test_func");

        // assert x > 11
        let x = z3::ast::BV::new_const(&ctx, "x", 64);
        state.assert(&x.bvsgt(&BV::from_i64(&ctx, 11, 64)));

        // create a backtrack point with constraint y > 5
        let y = z3::ast::BV::new_const(&ctx, "y", 64);
        let constraint = y.bvsgt(&BV::from_i64(&ctx, 5, 64));
        let bb = BasicBlock::new(Name::Name("bb_target".to_owned()));
        state.save_backtracking_point(bb.name.clone(), constraint);

        // check that the constraint y > 5 wasn't added: adding y < 4 should keep us sat
        assert_eq!(state.check_with_extra_constraints(std::iter::once(&y.bvslt(&BV::from_i64(&ctx, 4, 64)))), Ok(true));

        // assert x < 8 to make us unsat
        state.assert(&x.bvslt(&BV::from_i64(&ctx, 8, 64)));
        assert_eq!(state.check(), Ok(false));

        // note the pre-rollback location
        let pre_rollback = state.cur_loc.clone();

        // roll back to backtrack point; check that we ended up at the right loc
        // and with the right prev_bb
        assert!(state.revert_to_backtracking_point());
        assert_eq!(state.cur_loc.func, pre_rollback.func);
        assert_eq!(state.cur_loc.bbname, bb.name);
        assert_eq!(state.prev_bb_name, Some("test_bb".to_owned().into()));  // the `blank_state` comes with this as the current bb name

        // check that the constraint x < 8 was removed: we're sat again
        assert_eq!(state.check(), Ok(true));

        // check that the constraint y > 5 was added: y evaluates to something > 5
        assert!(state.get_a_solution_for_bv(&y).unwrap().unwrap() > 5);

        // check that the first constraint remained in place: x > 11
        assert!(state.get_a_solution_for_bv(&x).unwrap().unwrap() > 11);

        // check that trying to backtrack again fails
        assert!(!state.revert_to_backtracking_point());
    }
}
