use llvm_ir::*;
use log::debug;
use reduce::Reduce;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use crate::alloc::Alloc;
use crate::varmap::{VarMap, RestoreInfo};
use crate::size::size;
use crate::backend::*;
use crate::config::Config;
use crate::extend::*;

pub struct State<'ctx, 'm, B> where B: Backend<'ctx> {
    /// Reference to the Z3 context being used
    pub ctx: &'ctx z3::Context,
    /// Indicates the `BasicBlock` which is currently being executed
    pub cur_loc: Location<'m>,
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
    /// Map from `Name`s of global variables, to addresses at which they are allocated
    allocated_globals: HashMap<Name, B::BV>,
    /// This tracks the call stack of the symbolic execution.
    /// The first entry is the top-level caller, while the last entry is the
    /// caller of the current function.
    ///
    /// We won't have a `StackFrame` for the current function here, only each of
    /// its callers. For instance, while we are executing the top-level function,
    /// this stack will be empty.
    stack: Vec<StackFrame<'ctx, 'm, B::BV, B::Bool>>,
    /// These backtrack points are places where execution can be resumed later
    /// (efficiently, thanks to the incremental solving capabilities of Z3).
    backtrack_points: Vec<BacktrackPoint<'ctx, 'm, B>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct PathEntry {
    pub funcname: String,
    pub bbname: Name,
}

impl fmt::Debug for PathEntry {
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
struct StackFrame<'ctx, 'm, V, B> where V: BV<'ctx, AssociatedBool = B>, B: Bool<'ctx, AssociatedBV = V> {
    /// Indicates the call instruction which was responsible for the call
    callsite: Callsite<'m>,
    /// Caller's local variables, so they can be restored when we return to the caller.
    /// This is necessary in the case of (direct or indirect) recursion.
    /// See notes on `VarMap.get_restore_info_for_fn()`.
    restore_info: RestoreInfo<'ctx, V, B>,
}

struct BacktrackPoint<'ctx, 'm, B> where B: Backend<'ctx> {
    /// Where to resume execution
    loc: Location<'m>,
    /// `Name` of the `BasicBlock` executed just prior to the `BacktrackPoint`.
    /// Assumed to be in the same `Module` and `Function` as `loc` (which is
    /// always true for how we currently use `BacktrackPoint`s as of this writing)
    prev_bb: Name,
    /// Call stack at the `BacktrackPoint`.
    /// This is a vector of `StackFrame`s where the first entry is the top-level
    /// caller, and the last entry is the caller of the `BacktrackPoint`'s function.
    stack: Vec<StackFrame<'ctx, 'm, B::BV, B::Bool>>,
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

impl<'ctx, 'm, B> fmt::Display for BacktrackPoint<'ctx, 'm, B> where B: Backend<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<BacktrackPoint to execute bb {:?} with constraint {:?} and {} frames on the callstack>", self.loc.bbname, self.constraint, self.stack.len())
    }
}

impl<'ctx, 'm, B> State<'ctx, 'm, B> where B: Backend<'ctx> {
    /// `start_loc`: the `Location` where the `State` should begin executing.
    ///   As of this writing, this should be the entry point of a function, or you
    ///   will have problems.
    pub fn new(ctx: &'ctx z3::Context, start_loc: Location<'m>, config: &Config<B>) -> Self {
        let backend_state = Rc::new(RefCell::new(B::State::default()));
        let mut state = Self {
            ctx,
            cur_loc: start_loc,
            prev_bb_name: None,
            path: Vec::new(),
            varmap: VarMap::new(ctx, config.loop_bound),
            mem: Memory::new_uninitialized(ctx, backend_state.clone()),
            alloc: Alloc::new(),
            solver: B::Solver::new(ctx, backend_state.clone()),
            allocated_globals: HashMap::new(),
            stack: Vec::new(),
            backtrack_points: Vec::new(),

            // listed last (out-of-order) so that it can be used above but moved in now
            backend_state,
        };
        // Here we do initialization of the global variables in the Module
        debug!("Initializing global variables");
        for var in &state.cur_loc.module.global_vars {
            if let Type::PointerType { pointee_type, .. } = &var.ty {
                let addr = state.allocate(size(&*pointee_type) as u64);
                if let Some(ref c) = var.initializer {
                    state.write(&addr, state.const_to_bv(c));
                }
                state.allocated_globals.insert(var.name.clone(), addr);
            } else {
                panic!("Global variable has non-pointer type {:?}", &var.ty);
            }
        }
        debug!("Done initializing global variables");
        state
    }

    /// Add `cond` as a constraint, i.e., assert that `cond` must be true
    pub fn assert(&mut self, cond: &B::Bool) {
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
    pub fn check_with_extra_constraints<'a>(&'a mut self, conds: impl Iterator<Item = &'a B::Bool>) -> bool {
        self.solver.check_with_extra_constraints(conds)
    }

    /// Get one possible concrete value for the `BV`.
    /// Returns `None` if no possible solution.
    pub fn get_a_solution_for_bv(&mut self, bv: &B::BV) -> Option<u64> {
        self.solver.get_a_solution_for_bv(bv)
    }

    /// Get one possible concrete value for the `Bool`.
    /// Returns `None` if no possible solution.
    pub fn get_a_solution_for_bool(&mut self, b: &B::Bool) -> Option<bool> {
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
                if let Some(addr) = self.allocated_globals.get(name) {
                    addr.clone()
                } else if let Some(alias) = self.cur_loc.module.global_aliases.iter().find(|a| &a.name == name) {
                    self.const_to_bv(&alias.aliasee)
                } else {
                    panic!("const_to_bv on a GlobalReference but couldn't find {:?} in the module's globals", name)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    // we don't include tests here for Solver, Memory, Alloc, or VarMap; those are tested in their own modules.
    // Instead, here we just test the nontrivial functionality that State has itself.

    /// utility to initialize a `State` out of a `z3::Context`, a `Module`, and a `Function`
    fn blank_state<'ctx, 'm>(ctx: &'ctx z3::Context, module: &'m Module, func: &'m Function) -> State<'ctx, 'm, Z3Backend<'ctx>> {
        let start_loc = Location {
            module,
            func,
            bbname: "test_bb".to_owned().into(),
        };
        State::new(ctx, start_loc, &Config::default())
    }

    /// utility that creates a technically valid (but functionally useless) `Module` for testing
    fn blank_module(name: impl Into<String>) -> Module {
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
        let x = z3::ast::BV::new_const(&ctx, "x", 64);
        state.assert(&x.bvsgt(&BV::from_i64(&ctx, 11, 64)));

        // create a backtrack point with constraint y > 5
        let y = z3::ast::BV::new_const(&ctx, "y", 64);
        let constraint = y.bvsgt(&BV::from_i64(&ctx, 5, 64));
        let bb = BasicBlock::new(Name::Name("bb_target".to_owned()));
        state.save_backtracking_point(bb.name.clone(), constraint);

        // check that the constraint y > 5 wasn't added: adding y < 4 should keep us sat
        assert!(state.check_with_extra_constraints(std::iter::once(&y.bvslt(&BV::from_i64(&ctx, 4, 64)))));

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
