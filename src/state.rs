use boolector::option::{BtorOption, ModelGen};
use boolector::BVSolution;
use either::Either;
use itertools::Itertools;
use llvm_ir::types::{FPType, NamedStructDef, Typed};
use llvm_ir::*;
use log::{debug, info, warn};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

// Rust 1.51.0 introduced its own `.reduce()` on the main `Iterator` trait.
// So, starting with 1.51.0, we don't need `reduce::Reduce`, and in fact it
// causes a conflict.
#[rustversion::before(1.51)]
use reduce::Reduce;

use crate::alloc::Alloc;
use crate::backend::*;
use crate::config::{Config, NullPointerChecking};
use crate::demangling::Demangling;
use crate::error::*;
use crate::function_hooks::{self, FunctionHooks};
use crate::global_allocations::*;
use crate::hooks;
use crate::project::Project;
use crate::solver_utils::{self, PossibleSolutions};
use crate::varmap::{RestoreInfo, VarMap};
use crate::watchpoints::{Watchpoint, Watchpoints};

/// A `State` describes the full program state at a given moment during symbolic
/// execution.
#[derive(Clone)]
pub struct State<'p, B: Backend> {
    /// Reference to the solver instance being used
    pub solver: B::SolverRef,
    /// The configuration being used
    pub config: Config<'p, B>,
    /// Indicates the instruction which is currently being executed
    pub cur_loc: Location<'p>,

    // Private members
    proj: &'p Project,
    varmap: VarMap<B::BV>,
    mem: RefCell<B::Memory>,
    alloc: Alloc,
    global_allocations: GlobalAllocations<'p, B>,
    /// Pointer size in bits.
    /// E.g., this will be `64` if we're analyzing code which was compiled for a
    /// 64-bit platform.
    pointer_size_bits: u32,
    /// Separate from the user-defined hooks in the `config`, these are built-in
    /// hooks for LLVM intrinsics. They can be overridden by hooks in the
    /// `config`; see notes on function resolution in function_hooks.rs.
    pub(crate) intrinsic_hooks: FunctionHooks<'p, B>,
    /// This tracks the call stack of the symbolic execution.
    /// The first entry is the top-level caller, while the last entry is the
    /// caller of the current function.
    ///
    /// We won't have a `StackFrame` for the current function here, only each of
    /// its callers. For instance, while we are executing the top-level function,
    /// this stack will be empty.
    stack: Vec<StackFrame<'p, B::BV>>,
    /// These backtrack points are places where execution can be resumed later
    /// (efficiently, thanks to the incremental solving capabilities of Boolector).
    backtrack_points: RefCell<Vec<BacktrackPoint<'p, B>>>,
    /// Log of the basic blocks which have been executed to get to this point
    path: Vec<PathEntry<'p>>,
    /// Memory watchpoints (segments of memory to log reads/writes of).
    ///
    /// These will persist across backtracking - i.e., backtracking will not
    /// restore watchpoints to what they were at the backtrack point;
    /// backtracking will not touch the set of mem_watchpoints or their
    /// enabled statuses.
    mem_watchpoints: Watchpoints,
    /// Empirically, solving with model-gen enabled can be very slow.
    /// In particular, given a `BV` representing a function pointer, solving for
    /// the concrete function pointer it represents can be slow.
    /// However, if we have a guess for the concrete value, checking whether that
    /// guess is correct may be much faster than blindly solving for the value.
    ///
    /// This cache keeps track of the most recent concrete function pointer value
    /// we resolved at each `Location` where we call a function pointer.
    /// Hopefully, this means we can do the model-gen solve the first time, and
    /// then subsequent times just check that the same solution still holds.
    ///
    /// This cache persists across backtracking - there's no reason to reset it,
    /// as its contents are still treated as "guesses" and checked each time
    /// anyway, and function pointers _probably_ resolve to the same value on
    /// multiple paths.
    function_ptr_cache: HashMap<Location<'p>, u64>,
}

/// Describes a location in LLVM IR in a format more suitable for printing - for
/// instance, uses function names rather than references to `Function` objects.
/// For a richer representation of a code location, see
/// [`Location`](struct.Location.html).
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct LocationDescription<'p> {
    pub modname: String,
    pub funcname: String,
    pub bbname: Name,
    pub instr: BBInstrIndex,
    pub source_loc: Option<&'p DebugLoc>,
}

/// Denotes either a particular instruction in a basic block, or its terminator.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum BBInstrIndex {
    /// Index of the instruction within the basic block. 0-indexed, so 0 means the first instruction of the basic block.
    Instr(usize),
    /// Indicates the basic block terminator (not one of its instructions)
    Terminator,
}

impl fmt::Display for BBInstrIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BBInstrIndex::Instr(i) => write!(f, "instr {}", i),
            BBInstrIndex::Terminator => write!(f, "terminator"),
        }
    }
}

fn pretty_source_loc(source_loc: &DebugLoc) -> String {
    source_loc.to_string()
}

impl<'p> fmt::Debug for LocationDescription<'p> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string_with_module()) // default to with-module, especially for a Debug representation
    }
}

impl<'p> LocationDescription<'p> {
    pub(crate) fn to_string_with_module(&self) -> String {
        format!(
            "{{{}: {}, bb {}, {}}}",
            self.modname, self.funcname, self.bbname, self.instr
        )
    }

    pub(crate) fn to_string_no_module(&self) -> String {
        format!("{{{}, bb {}, {}}}", self.funcname, self.bbname, self.instr)
    }
}

/// Describes one segment of a path through the LLVM IR. The "segment" will be
/// one or more consecutive instructions in a single basic block.
///
/// For now, it's just a wrapper around a `Location` describing where the path
/// segment started.
/// E.g., instr 0 within some basic block means we started at the beginning of
/// that basic block.
/// Since the segment stays within a single basic block, the end of the segment
/// must be somewhere within that basic block.
#[derive(PartialEq, Eq, Clone)]
pub struct PathEntry<'p>(pub Location<'p>);

impl<'p> fmt::Debug for PathEntry<'p> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string_with_module()) // default to with-module, especially for a Debug representation
    }
}

impl<'p> PathEntry<'p> {
    pub(crate) fn to_string_with_module(&self) -> String {
        format!(
            "{{{}: {}, bb {}, starting at {}}}",
            self.0.module.name, self.0.func.name, self.0.bb.name, self.0.instr
        )
    }

    pub(crate) fn to_string_no_module(&self) -> String {
        format!(
            "{{{}, bb {}, starting at {}}}",
            self.0.func.name, self.0.bb.name, self.0.instr
        )
    }

    /// Get all the source locations touched on this path segment.
    /// Consecutive LLVM instructions with the same source location will be
    /// collapsed, so no two consecutive items of the returned iterator will be
    /// equal.
    /// The returned iterator may also be empty, for instance if no debuginfo is
    /// present.
    pub(crate) fn get_all_source_locs(&self) -> impl Iterator<Item = &'p DebugLoc> {
        self.0
            .bb
            .instrs
            .iter()
            .filter_map(|instr| instr.get_debug_loc().as_ref())
            .dedup()
    }
}

/// Fully describes a code location within the LLVM IR.
#[derive(Clone)]
pub struct Location<'p> {
    pub module: &'p Module,
    pub func: &'p Function,
    pub bb: &'p BasicBlock,
    pub instr: BBInstrIndex,
    /// Source location which this IR location corresponds to, if available.
    pub source_loc: Option<&'p DebugLoc>,
}

/// Implementation of `PartialEq` assumes that module names are unique;
/// that function names are unique within a module;
/// and that bb names are unique within a function
impl<'p> PartialEq for Location<'p> {
    fn eq(&self, other: &Self) -> bool {
        self.module.name == other.module.name
            && self.func.name == other.func.name
            && self.bb.name == other.bb.name
            && self.instr == other.instr
    }
}

/// Our implementation of `PartialEq` satisfies the requirements of `Eq`
impl<'p> Eq for Location<'p> {}

impl<'p> Hash for Location<'p> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.module.name.hash(state);
        self.func.name.hash(state);
        self.bb.name.hash(state);
        self.instr.hash(state);
    }
}

impl<'p> fmt::Debug for Location<'p> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}}}", self.to_string_with_module()) // default to with-module, especially for a Debug representation
    }
}

impl<'p> Location<'p> {
    /// Format this `Location` as a string, including the full module name
    pub fn to_string_with_module(&self) -> String {
        format!(
            "{}: {}, bb {}, {}",
            self.module.name, self.func.name, self.bb.name, self.instr
        )
    }

    /// Format this `Location` as a string, omitting the module name
    pub fn to_string_no_module(&self) -> String {
        format!("{}, bb {}, {}", self.func.name, self.bb.name, self.instr)
    }

    /// Format this `Location` as a string, including the short module name. The
    /// short module name is the part of the module name after the last `/`, if
    /// any; or the full module name, if the module name does not contain a `/`.
    pub fn to_string_short_module(&self) -> String {
        let short_module_name = self
            .module
            .name
            .rsplit('/')
            .next()
            .unwrap_or(&self.module.name);
        format!(
            "{}: {}, bb {}, {}",
            short_module_name, self.func.name, self.bb.name, self.instr
        )
    }
}

impl<'p> From<Location<'p>> for LocationDescription<'p> {
    fn from(loc: Location<'p>) -> LocationDescription {
        LocationDescription {
            modname: loc.module.name.clone(),
            funcname: loc.func.name.clone(),
            bbname: loc.bb.name.clone(),
            instr: loc.instr,
            source_loc: loc.source_loc,
        }
    }
}

impl<'p> Location<'p> {
    /// Move to the start of the given basic block, in the same function
    pub(crate) fn move_to_start_of_bb(&mut self, bb: &'p BasicBlock) {
        self.bb = bb;
        self.instr = BBInstrIndex::Instr(0);
    }

    /// Move to the start of the basic block with the given name, in the same function
    pub(crate) fn move_to_start_of_bb_by_name(&mut self, bbname: &Name) {
        self.move_to_start_of_bb(self.func.get_bb_by_name(bbname).unwrap_or_else(|| {
            panic!(
                "Failed to find bb named {} in function {:?}",
                bbname, self.func.name
            )
        }))
    }

    /// Increment the instruction index in the `Location`.
    /// Caller is responsible for ensuring that the `Location` did not point to a
    /// terminator, or this function will panic.
    pub(crate) fn inc(&mut self) {
        match self.instr {
            BBInstrIndex::Instr(i) => {
                if i + 1 >= self.bb.instrs.len() {
                    self.instr = BBInstrIndex::Terminator;
                } else {
                    self.instr = BBInstrIndex::Instr(i + 1);
                }
            },
            BBInstrIndex::Terminator => {
                panic!("called inc() on a Location pointing to a terminator")
            },
        }
    }
}

/// Fully describes the code location of a `Call` or `Invoke` instruction within
/// the LLVM IR, and also includes a reference to the `Call` or `Invoke` instruction
/// itself.
#[derive(PartialEq, Clone, Debug)]
pub struct Callsite<'p> {
    /// Indicates the call or invoke instruction which was responsible for the call
    pub loc: Location<'p>,
    /// Reference to the actual instruction (either a `Call` or `Invoke`) which was
    /// responsible for the call
    pub instr: Either<&'p instruction::Call, &'p terminator::Invoke>,
}

#[derive(PartialEq, Clone, Debug)]
struct StackFrame<'p, V: BV> {
    /// Indicates the call or invoke instruction which was responsible for the call
    callsite: Callsite<'p>,
    /// Caller's local variables, so they can be restored when we return to the caller.
    /// This is necessary in the case of (direct or indirect) recursion.
    /// See notes on `VarMap.get_restore_info_for_fn()`.
    restore_info: RestoreInfo<V>,
}

#[derive(Clone)]
struct BacktrackPoint<'p, B: Backend> {
    /// Where to resume execution
    loc: Location<'p>,
    /// Call stack at the `BacktrackPoint`.
    /// This is a vector of `StackFrame`s where the first entry is the top-level
    /// caller, and the last entry is the caller of the `BacktrackPoint`'s function.
    stack: Vec<StackFrame<'p, B::BV>>,
    /// Constraint to add before restarting execution at `next_bb`.
    /// (Intended use of this is to constrain the branch in that direction.)
    constraint: B::BV,
    /// `VarMap` representing the state of things at the `BacktrackPoint`.
    /// For now, we require making a full copy of the `VarMap` in order to revert
    /// later.
    varmap: VarMap<B::BV>,
    /// `Memory` representing the state of things at the `BacktrackPoint`.
    /// Copies of a `Memory` should be cheap (just a Boolector refcounted
    /// pointer), so it's not a huge concern that we need a full copy here in
    /// order to revert later.
    mem: B::Memory,
    /// The length of `path` at the `BacktrackPoint`.
    /// If we ever revert to this `BacktrackPoint`, we will truncate the `path` to
    /// its first `path_len` entries.
    path_len: usize,
}

impl<'p, B: Backend> fmt::Display for BacktrackPoint<'p, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<BacktrackPoint to execute bb {} with constraint {:?} and {} frames on the callstack>",
            self.loc.bb.name,
            self.constraint,
            self.stack.len()
        )
    }
}

impl<'p, B: Backend> State<'p, B>
where
    B: 'p,
{
    /// `start_loc`: the `Location` where the `State` should begin executing.
    /// As of this writing, `start_loc` should be the entry point of a
    /// function, or you will have problems.
    pub fn new(project: &'p Project, start_loc: Location<'p>, mut config: Config<'p, B>) -> Self {
        let solver = B::SolverRef::new();
        solver.set_opt(BtorOption::SolverTimeout(config.solver_query_timeout));
        if config.demangling.is_none() {
            config.demangling = Some(Demangling::autodetect(project));
        }
        let mut state = Self {
            cur_loc: start_loc.clone(),
            pointer_size_bits: project.pointer_size_bits(),
            proj: project,
            varmap: VarMap::new(solver.clone(), config.loop_bound),
            mem: RefCell::new(Memory::new_uninitialized(
                solver.clone(),
                match config.null_pointer_checking {
                    NullPointerChecking::Simple => true,
                    NullPointerChecking::SplitPath => true,
                    NullPointerChecking::None => false,
                },
                None,
                project.pointer_size_bits(),
            )),
            alloc: Alloc::new(),
            global_allocations: GlobalAllocations::new(),
            intrinsic_hooks: {
                let mut intrinsic_hooks = FunctionHooks::new();
                // we use "function names" that are clearly illegal, as an additional precaution to avoid collisions with actual function names
                intrinsic_hooks.add("intrinsic: llvm.memset", &hooks::intrinsics::symex_memset);
                intrinsic_hooks.add(
                    "intrinsic: llvm.memcpy/memmove",
                    &hooks::intrinsics::symex_memcpy,
                );
                intrinsic_hooks.add("intrinsic: llvm.bswap", &hooks::intrinsics::symex_bswap);
                intrinsic_hooks.add("intrinsic: llvm.ctlz", &hooks::intrinsics::symex_ctlz);
                intrinsic_hooks.add("intrinsic: llvm.cttz", &hooks::intrinsics::symex_cttz);
                intrinsic_hooks.add(
                    "intrinsic: llvm.objectsize",
                    &hooks::intrinsics::symex_objectsize,
                );
                intrinsic_hooks.add("intrinsic: llvm.assume", &hooks::intrinsics::symex_assume);
                intrinsic_hooks.add(
                    "intrinsic: llvm.uadd.with.overflow",
                    &hooks::intrinsics::symex_uadd_with_overflow,
                );
                intrinsic_hooks.add(
                    "intrinsic: llvm.sadd.with.overflow",
                    &hooks::intrinsics::symex_sadd_with_overflow,
                );
                intrinsic_hooks.add(
                    "intrinsic: llvm.usub.with.overflow",
                    &hooks::intrinsics::symex_usub_with_overflow,
                );
                intrinsic_hooks.add(
                    "intrinsic: llvm.ssub.with.overflow",
                    &hooks::intrinsics::symex_ssub_with_overflow,
                );
                intrinsic_hooks.add(
                    "intrinsic: llvm.umul.with.overflow",
                    &hooks::intrinsics::symex_umul_with_overflow,
                );
                intrinsic_hooks.add(
                    "intrinsic: llvm.smul.with.overflow",
                    &hooks::intrinsics::symex_smul_with_overflow,
                );
                intrinsic_hooks.add(
                    "intrinsic: llvm.uadd.sat",
                    &hooks::intrinsics::symex_uadd_sat,
                );
                intrinsic_hooks.add(
                    "intrinsic: llvm.sadd.sat",
                    &hooks::intrinsics::symex_sadd_sat,
                );
                intrinsic_hooks.add(
                    "intrinsic: llvm.usub.sat",
                    &hooks::intrinsics::symex_usub_sat,
                );
                intrinsic_hooks.add(
                    "intrinsic: llvm.ssub.sat",
                    &hooks::intrinsics::symex_ssub_sat,
                );
                intrinsic_hooks.add(
                    "intrinsic: generic_stub_hook",
                    &function_hooks::generic_stub_hook,
                );
                intrinsic_hooks.add("intrinsic: abort_hook", &function_hooks::abort_hook);
                intrinsic_hooks
            },
            stack: Vec::new(),
            backtrack_points: RefCell::new(Vec::new()),
            path: Vec::new(),
            mem_watchpoints: config.initial_mem_watchpoints.clone().into_iter().collect(),
            function_ptr_cache: HashMap::new(),

            // listed last (out-of-order) so that they can be used above but moved in now
            solver,
            config,
        };
        // Here we do allocation of the global variables in the Project.
        // We can do _initialization_ lazily (on first reference to the global
        // variable), but we need to do all the _allocation_ up front,
        // because initializers can refer to the addresses of other global
        // variables, potentially even circularly.
        //
        // Note that `project.all_global_vars()` gives us both global variable
        // *definitions* and *declarations*; we can distinguish these because
        // (direct quote from the LLVM docs) "Definitions have initializers,
        // declarations don't." This implies that even globals without an
        // initializer in C have one in LLVM, which seems weird to me, but it's
        // what the docs say, and also matches what I've seen empirically.
        //
        // We'll save each initializer as we allocate the global variable, but
        // only actually process each initializer as the global variable is
        // referenced for the first time.  This saves us from doing all the
        // memory reads/writes right away, which improves performance, especially
        // if the `Project` includes a lot of globals we'll never use (e.g., if
        // we parsed in way more modules than we actually need).
        info!("Allocating global variables and functions");
        debug!("Allocating global variables");
        for (var, module) in project
            .all_global_vars()
            .filter(|(var, _)| var.initializer.is_some())
        {
            // Allocate the global variable.
            //
            // In the allocation pass, we want to process each global variable
            // exactly once, and the order doesn't matter, so we simply process
            // definitions, since each global variable must have exactly one
            // definition. Hence the `filter()` above.
            if let Type::PointerType { pointee_type, .. } = var.ty.as_ref() {
                let size_bits = state.size_in_bits(&pointee_type).expect(
                    "Global variable has a struct type which is opaque in the entire Project",
                );
                let size_bits = if size_bits == 0 {
                    debug!(
                        "Global {:?} has size 0 bits; allocating 8 bits for it anyway",
                        var.name
                    );
                    8
                } else {
                    size_bits
                };
                let addr = state.allocate(size_bits as u64);
                debug!("Allocated {:?} at {:?}", var.name, addr);
                state
                    .global_allocations
                    .allocate_global_var(var, module, addr);
            } else {
                panic!("Global variable has non-pointer type {:?}", &var.ty);
            }
        }
        // We also have to allocate (at least a tiny bit of) memory for each
        // `Function`, just so that we can have pointers to those `Function`s.
        // We can use `global_allocations.get_func_for_address()` to interpret
        // these function pointers.
        // Similarly, we allocate tiny bits of memory for each function hook,
        // so that we can have pointers to those hooks.
        debug!("Allocating functions");
        for (func, module) in project.all_functions() {
            let addr: u64 = state.alloc.alloc(64_u64); // we just allocate 64 bits for each function. No reason to allocate more.
            let addr_bv = state.bv_from_u64(addr, project.pointer_size_bits());
            debug!("Allocated {:?} at {:?}", func.name, addr_bv);
            state
                .global_allocations
                .allocate_function(func, module, addr, addr_bv);
        }
        debug!("Allocating function hooks");
        for (funcname, hook) in state.config.function_hooks.get_all_hooks() {
            let addr: u64 = state.alloc.alloc(64_u64); // we just allocate 64 bits for each function. No reason to allocate more.
            let addr_bv = state.bv_from_u64(addr, project.pointer_size_bits());
            debug!("Allocated hook for {:?} at {:?}", funcname, addr_bv);
            state
                .global_allocations
                .allocate_function_hook((*hook).clone(), addr, addr_bv);
        }
        debug!("Done allocating global variables and functions");
        state
    }

    /// Fully duplicate the `State`. Unlike with `clone()`, the `State` this
    /// function returns will have a fully separate (fully duplicated) solver
    /// instance. (With `clone()`, the states will still share references to the
    /// same solver instance.)
    pub fn fork(&self) -> Self {
        let mut cloned = self.clone();
        let new_solver = cloned.solver.duplicate();
        cloned.varmap.change_solver(new_solver.clone());
        cloned.mem.borrow_mut().change_solver(new_solver.clone());
        cloned.global_allocations.change_solver(new_solver.clone());
        cloned.solver = new_solver;
        cloned
    }

    /// Returns `true` if current constraints are satisfiable, `false` if not.
    ///
    /// Returns `Error::SolverError` if the query failed (e.g., was interrupted or timed out).
    pub fn sat(&self) -> Result<bool> {
        solver_utils::sat(&self.solver)
    }

    /// Returns `true` if the current constraints plus the given additional constraints
    /// are together satisfiable, or `false` if not.
    ///
    /// Returns `Error::SolverError` if the query failed (e.g., was interrupted or timed out).
    ///
    /// Does not permanently add the given constraints to the solver.
    pub fn sat_with_extra_constraints<'b>(
        &'b self,
        constraints: impl IntoIterator<Item = &'b B::BV>,
    ) -> Result<bool> {
        solver_utils::sat_with_extra_constraints(&self.solver, constraints)
    }

    /// Get the `BV` corresponding to the given IR `Name` (from the given
    /// `Function` name).
    ///
    /// There should already have been a `BV` created for this `Name` on this
    /// path; this won't attempt to create a `BV` if there isn't already one for
    /// this `Name`.
    #[allow(clippy::ptr_arg)] // as of this writing, clippy warns that the &String argument should be &str; but it actually needs to be &String here
    pub fn get_bv_by_irname<'s>(&'s self, funcname: &String, name: &Name) -> &'s B::BV {
        self.varmap.lookup_var(funcname, name)
    }

    /// Returns `true` if under the current constraints, `a` and `b` must have the
    /// same value. Returns `false` if `a` and `b` may have different values. (If the
    /// current constraints are themselves unsatisfiable, that will result in
    /// `true`.)
    ///
    /// A common use case for this function is to test whether some `BV` must be
    /// equal to a given concrete value. You can do this with something like
    /// `state.bvs_must_be_equal(bv, &state.bv_from_u64(...))`.
    ///
    /// This function and `bvs_can_be_equal()` are both more efficient than
    /// `get_a_solution()` or `get_possible_solutions()`-type functions, as they do
    /// not require full model generation. You should prefer this function or
    /// `bvs_can_be_equal()` if they are sufficient for your needs.
    pub fn bvs_must_be_equal(&self, a: &B::BV, b: &B::BV) -> Result<bool> {
        solver_utils::bvs_must_be_equal(&self.solver, a, b)
    }

    /// Returns `true` if under the current constraints, `a` and `b` can have the
    /// same value. Returns `false` if `a` and `b` cannot have the same value. (If
    /// the current constraints are themselves unsatisfiable, that will also result
    /// in `false`.)
    ///
    /// A common use case for this function is to test whether some `BV` can be
    /// equal to a given concrete value. You can do this with something like
    /// `state.bvs_can_be_equal(bv, &state.bv_from_u64(...))`.
    ///
    /// This function and `bvs_must_be_equal()` are both more efficient than
    /// `get_a_solution()` or `get_possible_solutions()`-type functions, as they do
    /// not require full model generation. You should prefer this function or
    /// `bvs_must_be_equal()` if they are sufficient for your needs.
    pub fn bvs_can_be_equal(&self, a: &B::BV, b: &B::BV) -> Result<bool> {
        solver_utils::bvs_can_be_equal(&self.solver, a, b)
    }

    /// Get one possible concrete value for the `BV`.
    /// Returns `Ok(None)` if no possible solution, or `Error::SolverError` if the solver query failed.
    pub fn get_a_solution_for_bv(&self, bv: &B::BV) -> Result<Option<BVSolution>> {
        // first check if the `bv` is a constant, if so, we can avoid a solve
        match bv.as_binary_str() {
            Some(bstr) => Ok(Some(BVSolution::from_01x_str(bstr))),
            None => {
                warn!("A call to get_a_solution_for_bv() is resulting in a call to sat() with model generation enabled. Experimentally, these types of calls can be very slow. The BV is {:?}", bv);
                self.solver.set_opt(BtorOption::ModelGen(ModelGen::All));
                let solution = if self.sat()? {
                    bv.get_a_solution().map(Some)
                } else {
                    Ok(None)
                };
                self.solver
                    .set_opt(BtorOption::ModelGen(ModelGen::Disabled));
                solution
            },
        }
    }

    /// Get one possible concrete value for the given IR `Name` (from the given `Function` name).
    /// Returns `Ok(None)` if no possible solution, or `Error::SolverError` if the solver query failed.
    #[allow(clippy::ptr_arg)] // as of this writing, clippy warns that the &String argument should be &str; but it actually needs to be &String here
    pub fn get_a_solution_for_irname(
        &mut self,
        funcname: &String,
        name: &Name,
    ) -> Result<Option<BVSolution>> {
        let bv = self.varmap.lookup_var(funcname, name);
        self.get_a_solution_for_bv(bv)
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
    pub fn get_possible_solutions_for_bv(
        &self,
        bv: &B::BV,
        n: usize,
    ) -> Result<PossibleSolutions<BVSolution>> {
        solver_utils::get_possible_solutions_for_bv(self.solver.clone(), bv, n)
    }

    /// Get a description of the possible solutions for the given IR `Name` (from the given `Function` name).
    ///
    /// `n`: Maximum number of distinct solutions to check for.
    /// If there are more than `n` possible solutions, this returns a
    /// `PossibleSolutions::AtLeast` containing `n+1` solutions.
    ///
    /// These solutions will be disambiguated - see docs on `boolector::BVSolution`.
    ///
    /// If there are no possible solutions, this returns `Ok` with an empty
    /// `PossibleSolutions`, rather than returning an `Err` with `Error::Unsat`.
    #[allow(clippy::ptr_arg)] // as of this writing, clippy warns that the &String argument should be &str; but it actually needs to be &String here
    pub fn get_possible_solutions_for_irname(
        &mut self,
        funcname: &String,
        name: &Name,
        n: usize,
    ) -> Result<PossibleSolutions<BVSolution>> {
        let bv = self.varmap.lookup_var(funcname, name);
        self.get_possible_solutions_for_bv(bv, n)
    }

    /// Get the maximum possible solution for the `BV`: that is, the highest value
    /// for which the current set of constraints is still satisfiable.
    /// "Maximum" will be interpreted in an unsigned fashion.
    ///
    /// Returns `Ok(None)` if there is no solution for the `BV`, that is, if the
    /// current set of constraints is unsatisfiable. Only returns `Err` if a solver
    /// query itself fails. Panics if the `BV` is wider than 64 bits.
    pub fn max_possible_solution_for_bv_as_u64(&self, bv: &B::BV) -> Result<Option<u64>> {
        solver_utils::max_possible_solution_for_bv_as_u64(self.solver.clone(), bv)
    }

    /// Get the maximum possible solution for the given IR `Name` (from the given
    /// `Function` name): that is, the highest value for which the current set of
    /// constraints is still satisfiable.
    /// "Maximum" will be interpreted in an unsigned fashion.
    ///
    /// Returns `Ok(None)` if there is no solution for the `BV`, that is, if the
    /// current set of constraints is unsatisfiable. Only returns `Err` if a solver
    /// query itself fails. Panics if the `BV` is wider than 64 bits.
    #[allow(clippy::ptr_arg)] // as of this writing, clippy warns that the &String argument should be &str; but it actually needs to be &String here
    pub fn max_possible_solution_for_irname_as_u64(
        &mut self,
        funcname: &String,
        name: &Name,
    ) -> Result<Option<u64>> {
        let bv = self.varmap.lookup_var(funcname, name);
        solver_utils::max_possible_solution_for_bv_as_u64(self.solver.clone(), bv)
    }

    /// Get the minimum possible solution for the `BV`: that is, the lowest value
    /// for which the current set of constraints is still satisfiable.
    /// "Minimum" will be interpreted in an unsigned fashion.
    ///
    /// Returns `Ok(None)` if there is no solution for the `BV`, that is, if the
    /// current set of constraints is unsatisfiable. Only returns `Err` if a solver
    /// query itself fails. Panics if the `BV` is wider than 64 bits.
    pub fn min_possible_solution_for_bv_as_u64(&self, bv: &B::BV) -> Result<Option<u64>> {
        solver_utils::min_possible_solution_for_bv_as_u64(self.solver.clone(), bv)
    }

    /// Get the minimum possible solution for the given IR `Name` (from the given
    /// `Function` name): that is, the lowest value for which the current set of
    /// constraints is still satisfiable.
    /// "Minimum" will be interpreted in an unsigned fashion.
    ///
    /// Returns `Ok(None)` if there is no solution for the `BV`, that is, if the
    /// current set of constraints is unsatisfiable. Only returns `Err` if a solver
    /// query itself fails. Panics if the `BV` is wider than 64 bits.
    #[allow(clippy::ptr_arg)] // as of this writing, clippy warns that the &String argument should be &str; but it actually needs to be &String here
    pub fn min_possible_solution_for_irname_as_u64(
        &self,
        funcname: &String,
        name: &Name,
    ) -> Result<Option<u64>> {
        let bv = self.varmap.lookup_var(funcname, name);
        solver_utils::min_possible_solution_for_bv_as_u64(self.solver.clone(), bv)
    }

    /// Create a `BV` constant representing the given `bool` (either constant
    /// `true` or constant `false`).
    /// The resulting `BV` will be either constant `0` or constant `1`, and will
    /// have bitwidth `1`.
    pub fn bv_from_bool(&self, b: bool) -> B::BV {
        B::BV::from_bool(self.solver.clone(), b)
    }

    /// Create a `BV` representing the given constant `i32` value, with the given
    /// bitwidth.
    pub fn bv_from_i32(&self, i: i32, width: u32) -> B::BV {
        B::BV::from_i32(self.solver.clone(), i, width)
    }

    /// Create a `BV` representing the given constant `u32` value, with the given
    /// bitwidth.
    pub fn bv_from_u32(&self, u: u32, width: u32) -> B::BV {
        B::BV::from_u32(self.solver.clone(), u, width)
    }

    /// Create a `BV` representing the given constant `i64` value, with the given
    /// bitwidth.
    pub fn bv_from_i64(&self, i: i64, width: u32) -> B::BV {
        B::BV::from_i64(self.solver.clone(), i, width)
    }

    /// Create a `BV` representing the given constant `u64` value, with the given
    /// bitwidth.
    pub fn bv_from_u64(&self, u: u64, width: u32) -> B::BV {
        B::BV::from_u64(self.solver.clone(), u, width)
    }

    /// Create a `BV` representing the constant `0` of the given bitwidth.
    /// This is equivalent to `self.bv_from_i32(0, width)` but may be more
    /// efficient.
    pub fn zero(&self, width: u32) -> B::BV {
        B::BV::zero(self.solver.clone(), width)
    }

    /// Create a `BV` representing the constant `1` of the given bitwidth.
    /// This is equivalent to `self.bv_from_i32(1, width)` but may be more
    /// efficient.
    pub fn one(&self, width: u32) -> B::BV {
        B::BV::one(self.solver.clone(), width)
    }

    /// Create a `BV` constant of the given width, where all bits are set to one.
    /// This is equivalent to `self.bv_from_i32(-1, width)` but may be more
    /// efficient.
    pub fn ones(&self, width: u32) -> B::BV {
        B::BV::ones(self.solver.clone(), width)
    }

    /// Create a new (unconstrained) `BV` for the given `Name` (in the current function).
    ///
    /// This function performs uniquing, so if you call it twice
    /// with the same `Name`-`Function` pair, you will get two different `BV`s.
    ///
    /// Returns the new `BV`, or `Err` if it can't be created.
    ///
    /// (As of this writing, the only `Err` that might be returned is
    /// `Error::LoopBoundExceeded`, which is returned if creating the new `BV`
    /// would exceed `max_versions_of_name` -- see
    /// [`Config`](struct.Config.html).)
    ///
    /// Also, we assume that no two `Function`s share the same name.
    pub fn new_bv_with_name(&mut self, name: Name, bits: u32) -> Result<B::BV> {
        self.varmap
            .new_bv_with_name(self.cur_loc.func.name.clone(), name, bits)
    }

    /// Assign the given `BV` to the given `Name` (in the current function).
    ///
    /// This function performs uniquing, so it creates a new version of the
    /// variable represented by the `(String, Name)` pair rather than overwriting
    /// the current version.
    ///
    /// Returns `Err` if the assignment can't be performed.
    ///
    /// (As of this writing, the only `Err` that might be returned is
    /// `Error::LoopBoundExceeded`, which is returned if creating the new version
    /// of the `BV` would exceed `max_versions_of_name` -- see
    /// [`Config`](struct.Config.html).)
    pub fn assign_bv_to_name(&mut self, name: Name, bv: B::BV) -> Result<()> {
        self.varmap
            .assign_bv_to_name(self.cur_loc.func.name.clone(), name, bv)
    }

    /// Record the result of `thing` to be `resultval`.
    /// Assumes `thing` is in the current function.
    /// Will fail with `Error::LoopBoundExceeded` if that would exceed
    /// `max_versions_of_name` (see [`Config`](struct.Config.html)).
    #[cfg(debug_assertions)]
    pub fn record_bv_result(
        &mut self,
        thing: &impl instruction::HasResult,
        resultval: B::BV,
    ) -> Result<()> {
        let thing_size_in_bits = self.size_in_bits(&self.type_of(thing)).ok_or_else(|| {
            Error::MalformedInstruction("Instruction result type is an opaque struct type".into())
        })?;
        if thing_size_in_bits != resultval.get_width() {
            Err(Error::OtherError(format!(
                "Computed result for an instruction has the wrong size: instruction {:?} with result size {}, but got result {:?} with size {}",
                thing,
                thing_size_in_bits,
                resultval,
                resultval.get_width()
            )))
        } else {
            self.assign_bv_to_name(thing.get_result().clone(), resultval)
        }
    }
    #[cfg(not(debug_assertions))]
    pub fn record_bv_result(
        &mut self,
        thing: &impl instruction::HasResult,
        resultval: B::BV,
    ) -> Result<()> {
        self.assign_bv_to_name(thing.get_result().clone(), resultval)
    }

    /// Overwrite the latest version of the given `Name` to instead be `bv`.
    /// Assumes `Name` is in the current function.
    pub fn overwrite_latest_version_of_bv(&mut self, name: &Name, bv: B::BV) {
        self.varmap
            .overwrite_latest_version_of_bv(&self.cur_loc.func.name, name, bv)
    }

    /// Convenience function to get the `Type` of anything that is `Typed`.
    pub fn type_of<T: Typed + ?Sized>(&self, t: &T) -> TypeRef {
        self.cur_loc.module.type_of(t)
    }

    /// Convert an `Operand` to the appropriate `BV`.
    /// Assumes the `Operand` is in the current function.
    /// (All `Operand`s should be either a constant or a variable we previously added to the state.)
    pub fn operand_to_bv(&self, op: &Operand) -> Result<B::BV> {
        match op {
            Operand::ConstantOperand(c) => self.const_to_bv(c),
            Operand::LocalOperand { name, .. } => Ok(self
                .varmap
                .lookup_var(&self.cur_loc.func.name, name)
                .clone()),
            Operand::MetadataOperand => panic!("Can't convert {:?} to BV", op),
        }
    }

    /// Convert a `Constant` to the appropriate `BV`.
    pub fn const_to_bv(&self, c: &Constant) -> Result<B::BV> {
        match c {
            Constant::Int { bits, value } => Ok(self.bv_from_u64(*value, *bits)),
            Constant::Null(ty) | Constant::AggregateZero(ty) | Constant::Undef(ty) => {
                let size_bits = self.size_in_bits(ty).ok_or_else(|| {
                    Error::OtherError(format!(
                        "const_to_bv on a constant with opaque struct type: {:?}",
                        c
                    ))
                })?;
                assert_ne!(size_bits, 0, "const_to_bv: can't convert constant of size 0 to a BV; use const_to_bv_maybe_zerowidth() instead");
                Ok(self.zero(size_bits))
            },
            Constant::Struct {
                values: elements, ..
            }
            | Constant::Array { elements, .. }
            | Constant::Vector(elements) => elements
                .iter()
                .map(|c| self.const_to_bv(c)) // produces an iterator over Result<B::BV>
                .reduce(|a, b| Ok(b?.concat(&a?))) // the lambda has type Fn(Result<B::BV>, Result<B::BV>) -> Result<B::BV>
                .unwrap(), // unwrap the Option<> produced by reduce(), leaving the final return type Result<B::BV>
            Constant::GlobalReference { name, .. } => {
                if let Some(ga) = self
                    .global_allocations
                    .get_global_allocation(name, self.cur_loc.module)
                {
                    match ga {
                        GlobalAllocation::Function { addr, .. } => Ok(addr.clone()),
                        GlobalAllocation::GlobalVariable {
                            addr,
                            initializer,
                            initialized,
                        } => {
                            // First, initialize the global if it hasn't been already.
                            // As mentioned in comments in `State::new()`, we lazily
                            // initialize globals upon first reference to them.
                            //
                            // We assume that global-variable initializers can only refer to the
                            // *addresses* of other globals, and not the *values* of other
                            // global constants, so that it's fine that any referred-to globals
                            // may have been allocated but not initialized at this point.
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
                            // Therefore, we can go ahead and set our `.initialized` flag early,
                            // because even if `const_to_bv` on our initializer references other
                            // globals (possibly causing their lazy initialization as well),
                            // those globals can't refer to our contents, so won't know that we
                            // are lying about being initialized.
                            // Setting the flag early prevents an infinite loop where I try to
                            // initialize, but my initializer refers to your address so you try
                            // to initialize, but your initializer refers to my address so I try
                            // to initialize, etc.
                            if !initialized.get() {
                                debug!(
                                    "Initializing {:?} with initializer {:?}",
                                    name, &initializer
                                );
                                initialized.set(true);
                                // Global variables could be zero-element arrays, or structs
                                // containing zero-element arrays, so we use
                                // `const_to_bv_maybe_zerowidth()`
                                if let Some(bv) = self.const_to_bv_maybe_zerowidth(initializer)? {
                                    // If that returned `None`, the global is a zero-element array,
                                    // in which case we don't want to initialize it (and can't, or
                                    // we'd get a panic about a 0-width BV)
                                    self.write_without_mut(addr, bv)?;
                                }
                            }
                            Ok(addr.clone())
                        },
                    }
                } else if let Some(alias) = self
                    .cur_loc
                    .module
                    .global_aliases
                    .iter()
                    .find(|a| &a.name == name)
                {
                    self.const_to_bv(&alias.aliasee)
                } else {
                    Err(Error::OtherError(format!("const_to_bv: GlobalReference to {:?} which was not found (current module is {:?})", name, &self.cur_loc.module.name)))
                }
            },
            Constant::Add(a) => Ok(self
                .const_to_bv(&a.operand0)?
                .add(&self.const_to_bv(&a.operand1)?)),
            Constant::Sub(s) => Ok(self
                .const_to_bv(&s.operand0)?
                .sub(&self.const_to_bv(&s.operand1)?)),
            Constant::Mul(m) => Ok(self
                .const_to_bv(&m.operand0)?
                .mul(&self.const_to_bv(&m.operand1)?)),
            Constant::UDiv(u) => Ok(self
                .const_to_bv(&u.operand0)?
                .udiv(&self.const_to_bv(&u.operand1)?)),
            Constant::SDiv(s) => Ok(self
                .const_to_bv(&s.operand0)?
                .sdiv(&self.const_to_bv(&s.operand1)?)),
            Constant::URem(u) => Ok(self
                .const_to_bv(&u.operand0)?
                .urem(&self.const_to_bv(&u.operand1)?)),
            Constant::SRem(s) => Ok(self
                .const_to_bv(&s.operand0)?
                .srem(&self.const_to_bv(&s.operand1)?)),
            Constant::And(a) => Ok(self
                .const_to_bv(&a.operand0)?
                .and(&self.const_to_bv(&a.operand1)?)),
            Constant::Or(o) => Ok(self
                .const_to_bv(&o.operand0)?
                .or(&self.const_to_bv(&o.operand1)?)),
            Constant::Xor(x) => Ok(self
                .const_to_bv(&x.operand0)?
                .xor(&self.const_to_bv(&x.operand1)?)),
            Constant::Shl(s) => Ok(self
                .const_to_bv(&s.operand0)?
                .sll(&self.const_to_bv(&s.operand1)?)),
            Constant::LShr(s) => Ok(self
                .const_to_bv(&s.operand0)?
                .srl(&self.const_to_bv(&s.operand1)?)),
            Constant::AShr(s) => Ok(self
                .const_to_bv(&s.operand0)?
                .sra(&self.const_to_bv(&s.operand1)?)),
            Constant::ExtractElement(ee) => match &ee.index.as_ref() {
                Constant::Int { value: index, .. } => match &ee.vector.as_ref() {
                    Constant::Vector(els) => {
                        let el = els.get(*index as usize).ok_or_else(|| {
                            Error::MalformedInstruction(
                                "Constant::ExtractElement index out of range".to_owned(),
                            )
                        })?;
                        self.const_to_bv(el)
                    },
                    c => Err(Error::MalformedInstruction(format!(
                        "Expected ExtractElement.vector to be a Constant::Vector, got {:?}",
                        c
                    ))),
                },
                index => Err(Error::MalformedInstruction(format!(
                    "Expected ExtractElement.index to be a Constant::Int, but got {:?}",
                    index
                ))),
            },
            Constant::InsertElement(ie) => match &ie.index.as_ref() {
                Constant::Int { value: index, .. } => match &ie.vector.as_ref() {
                    Constant::Vector(els) => {
                        let mut els = els.clone();
                        let el: &mut ConstantRef =
                            els.get_mut(*index as usize).ok_or_else(|| {
                                Error::MalformedInstruction(
                                    "Constant::InsertElement index out of range".to_owned(),
                                )
                            })?;
                        *el = ie.element.clone();
                        self.const_to_bv(&Constant::Vector(els))
                    },
                    c => Err(Error::MalformedInstruction(format!(
                        "Expected InsertElement.vector to be a Constant::Vector, got {:?}",
                        c
                    ))),
                },
                index => Err(Error::MalformedInstruction(format!(
                    "Expected InsertElement.index to be a Constant::Int, but got {:?}",
                    index
                ))),
            },
            Constant::ExtractValue(ev) => self.const_to_bv(Self::simplify_const_ev(
                &ev.aggregate,
                ev.indices.iter().copied(),
            )?),
            Constant::InsertValue(iv) => {
                let c = Self::simplify_const_iv(
                    &iv.aggregate,
                    (*iv.element).clone(),
                    iv.indices.iter().copied(),
                )?;
                self.const_to_bv(&c)
            },
            Constant::GetElementPtr(gep) => {
                // heavily inspired by `ExecutionManager::symex_gep()` in symex.rs. TODO could try to share more code
                let bvbase = self.const_to_bv(&gep.address)?;
                let offset = self.get_offset_recursive(
                    gep.indices.iter(),
                    &self.type_of(&gep.address),
                    bvbase.get_width(),
                )?;
                Ok(bvbase.add(&offset))
            },
            Constant::Trunc(t) => {
                let to_size_bits = self.size_in_bits(&t.to_type).ok_or_else(|| {
                    Error::OtherError(format!(
                        "const_to_bv on a constant with opaque struct type {:?}",
                        c
                    ))
                })?;
                assert_ne!(to_size_bits, 0, "const_to_bv: can't convert constant of size 0 to a BV; use const_to_bv_maybe_zerowidth() instead");
                self.const_to_bv(&t.operand)
                    .map(|bv| bv.slice(to_size_bits - 1, 0))
            },
            Constant::ZExt(z) => {
                let to_size_bits = self.size_in_bits(&z.to_type).ok_or_else(|| {
                    Error::OtherError(format!(
                        "const_to_bv on a constant with opaque struct type {:?}",
                        c
                    ))
                })?;
                assert_ne!(to_size_bits, 0, "const_to_bv: can't convert constant of size 0 to a BV; use const_to_bv_maybe_zerowidth() instead");
                self.const_to_bv(&z.operand)
                    .map(|bv| bv.zero_extend_to_bits(to_size_bits))
            },
            Constant::SExt(s) => {
                let to_size_bits = self.size_in_bits(&s.to_type).ok_or_else(|| {
                    Error::OtherError(format!(
                        "const_to_bv on a constant with opaque struct type {:?}",
                        c
                    ))
                })?;
                assert_ne!(to_size_bits, 0, "const_to_bv: can't convert constant of size 0 to a BV; use const_to_bv_maybe_zerowidth() instead");
                self.const_to_bv(&s.operand)
                    .map(|bv| bv.sign_extend_to_bits(to_size_bits))
            },
            Constant::PtrToInt(pti) => {
                let bv = self.const_to_bv(&pti.operand)?;
                assert_eq!(
                    bv.get_width(),
                    self.size_in_bits(&pti.to_type)
                        .ok_or_else(|| Error::MalformedInstruction(
                            "PtrToInt result type is opaque struct type".into()
                        ))?
                );
                Ok(bv) // just a cast, it's the same bits underneath
            },
            Constant::IntToPtr(itp) => {
                let bv = self.const_to_bv(&itp.operand)?;
                assert_eq!(
                    bv.get_width(),
                    self.size_in_bits(&itp.to_type)
                        .ok_or_else(|| Error::MalformedInstruction(
                            "IntToPtr result type is opaque struct type".into()
                        ))?
                );
                Ok(bv) // just a cast, it's the same bits underneath
            },
            Constant::BitCast(bc) => {
                let bv = self.const_to_bv(&bc.operand)?;
                assert_eq!(
                    bv.get_width(),
                    self.size_in_bits(&bc.to_type)
                        .ok_or_else(|| Error::MalformedInstruction(
                            "BitCast result type is opaque struct type".into()
                        ))?
                );
                Ok(bv) // just a cast, it's the same bits underneath
            },
            Constant::AddrSpaceCast(ac) => {
                let bv = self.const_to_bv(&ac.operand)?;
                assert_eq!(
                    bv.get_width(),
                    self.size_in_bits(&ac.to_type)
                        .ok_or_else(|| Error::MalformedInstruction(
                            "AddrSpaceCast result type is opaque struct type".into()
                        ))?
                );
                Ok(bv) // just a cast, it's the same bits underneath
            },
            Constant::ICmp(icmp) => {
                let bv0 = self.const_to_bv(&icmp.operand0)?;
                let bv1 = self.const_to_bv(&icmp.operand1)?;
                Ok(match icmp.predicate {
                    IntPredicate::EQ => bv0._eq(&bv1),
                    IntPredicate::NE => bv0._ne(&bv1),
                    IntPredicate::UGT => bv0.ugt(&bv1),
                    IntPredicate::UGE => bv0.ugte(&bv1),
                    IntPredicate::ULT => bv0.ult(&bv1),
                    IntPredicate::ULE => bv0.ulte(&bv1),
                    IntPredicate::SGT => bv0.sgt(&bv1),
                    IntPredicate::SGE => bv0.sgte(&bv1),
                    IntPredicate::SLT => bv0.slt(&bv1),
                    IntPredicate::SLE => bv0.slte(&bv1),
                })
            },
            Constant::Select(s) => {
                let b = self.const_to_bv(&s.condition)?;
                match b.as_bool() {
                    None => Err(Error::MalformedInstruction(
                        "Constant::Select: Expected a constant condition".to_owned(),
                    )),
                    Some(true) => self.const_to_bv(&s.true_value),
                    Some(false) => self.const_to_bv(&s.false_value),
                }
            },
            _ => unimplemented!("const_to_bv for {:?}", c),
        }
    }

    /// Convert a `Constant` to the appropriate `BV`, allowing for the `Constant`
    /// to possibly be zero-width (LLVM 0-element array is the only way for that
    /// to happen) or be a struct with zero-width elements (i.e., struct with one
    /// or more elements being a 0-element array).
    ///
    /// Returns `Ok(None)` if the result would be a zero-width `BV`.
    fn const_to_bv_maybe_zerowidth(&self, c: &Constant) -> Result<Option<B::BV>> {
        match c {
            Constant::Null(ty) | Constant::AggregateZero(ty) | Constant::Undef(ty) => {
                match self.size_in_bits(ty) {
                    None => Err(Error::OtherError(format!(
                        "const_to_bv on a constant with opaque struct type: {:?}",
                        c
                    ))),
                    Some(0) => Ok(None),
                    Some(bits) => Ok(Some(self.zero(bits))),
                }
            },
            Constant::Struct { values, .. } => {
                values
                    .iter()
                    .map(|val| {
                        self.size_in_bits(&self.type_of(val.as_ref()))
                            .map(|bits| (val, bits))
                            .ok_or_else(|| {
                                Error::OtherError(format!(
                                    "const_to_bv: encountered an opaque struct type: {:?}",
                                    val
                                ))
                            })
                    })
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .filter(|&(_val, bits)| bits > 0)
                    .map(|(val, _bits)| val)
                    .map(|val| self.const_to_bv_maybe_zerowidth(val).transpose().unwrap()) // since we `filter()`'d first, we should have all `Some`s here. We transpose-unwrap Result<Option<BV>> to Result<BV>
                    .reduce(|a, b| Ok(b?.concat(&a?))) // the lambda has type Fn(Result<B::BV>, Result<B::BV>) -> Result<B::BV>
                    .transpose()
            },
            Constant::Array { elements, .. } if elements.is_empty() => Ok(None),
            // note that Constant::Vector cannot have 0 elements, per LLVM LangRef
            _ => self.const_to_bv(c).map(|bv| Some(bv)),
        }
    }

    /// Given a `Constant::Struct` and a series of `ExtractValue` indices, get the
    /// final `Constant` referred to
    fn simplify_const_ev(
        s: &Constant,
        mut indices: impl Iterator<Item = u32>,
    ) -> Result<&Constant> {
        match indices.next() {
            None => Ok(s),
            Some(index) => {
                if let Constant::Struct { values, .. } = s {
                    let val = values.get(index as usize).ok_or_else(|| {
                        Error::MalformedInstruction(
                            "Constant::ExtractValue index out of range".to_owned(),
                        )
                    })?;
                    Self::simplify_const_ev(val, indices)
                } else {
                    panic!("simplify_const_ev: not a Constant::Struct: {:?}", s)
                }
            },
        }
    }

    /// Given a `Constant::Struct`, a value to insert, and a series of
    /// `InsertValue` indices, get the final `Constant` referred to
    fn simplify_const_iv(
        s: &Constant,
        val: Constant,
        mut indices: impl Iterator<Item = u32>,
    ) -> Result<ConstantRef> {
        match indices.next() {
            None => Ok(ConstantRef::new(val)),
            Some(index) => {
                if let Constant::Struct {
                    name,
                    values,
                    is_packed,
                } = s
                {
                    let to_replace = values
                        .get(index as usize)
                        .ok_or_else(|| {
                            Error::MalformedInstruction(
                                "Constant::InsertValue index out of range".to_owned(),
                            )
                        })?
                        .clone();
                    let mut values = values.clone();
                    values[index as usize] = Self::simplify_const_iv(&to_replace, val, indices)?;
                    Ok(ConstantRef::new(Constant::Struct {
                        name: name.clone(),
                        values,
                        is_packed: *is_packed,
                    }))
                } else {
                    panic!("simplify_const_iv: not a Constant::Struct: {:?}", s)
                }
            },
        }
    }

    /// Get the offset of the element (in bytes, as a `BV` of `result_bits` bits).
    ///
    /// If `base_type` is a `NamedStructType`, the struct should be defined in the current module.
    fn get_offset_recursive<'a>(
        &self,
        mut indices: impl Iterator<Item = &'a ConstantRef>,
        base_type: &Type,
        result_bits: u32,
    ) -> Result<B::BV> {
        if let Type::NamedStructType { name } = base_type {
            match self.cur_loc.module.types.named_struct_def(name) {
                None => {
                    return Err(Error::MalformedInstruction(format!("get_offset on a struct type not defined in the current module (struct name {:?})", name)));
                },
                Some(NamedStructDef::Opaque) => {
                    return Err(Error::MalformedInstruction(format!(
                        "get_offset on an opaque struct type ({:?})",
                        name
                    )));
                },
                Some(NamedStructDef::Defined(ty)) => {
                    return self.get_offset_recursive(indices, &ty, result_bits);
                },
            }
        }
        match indices.next() {
            None => Ok(self.zero(result_bits)),
            Some(index) => match base_type {
                Type::PointerType { .. } | Type::ArrayType { .. } | Type::VectorType { .. } => {
                    let index = self.const_to_bv(index)?.zero_extend_to_bits(result_bits);
                    let (offset, nested_ty) =
                        self.get_offset_bv_index(base_type, &index, self.solver.clone())?;
                    self.get_offset_recursive(indices, nested_ty, result_bits)
                        .map(|bv| bv.add(&offset))
                },
                Type::StructType { .. } => match index.as_ref() {
                    Constant::Int { value: index, .. } => {
                        let (offset, nested_ty) =
                            self.get_offset_constant_index(base_type, *index as usize)?;
                        self.get_offset_recursive(indices, &nested_ty, result_bits)
                            .map(|bv| bv.add(&self.bv_from_u64(offset as u64, result_bits)))
                    },
                    _ => Err(Error::MalformedInstruction(format!(
                        "Expected index into struct type to be a constant int, but got index {:?}",
                        index
                    ))),
                },
                Type::NamedStructType { .. } => {
                    panic!("NamedStructType case should have been handled above")
                },
                _ => panic!("get_offset_recursive with base type {:?}", base_type),
            },
        }
    }

    /// Given a `BV`, interpret it as a function pointer, and return a
    /// description of the possible `Function`s which it would point to.
    ///
    /// `n`: Maximum number of distinct `Callable`s to check for.
    /// If there are more than `n` possible `Callable`s, this returns a
    /// `PossibleSolutions::AtLeast` with `n+1` `Callable`s.
    ///
    /// Possible errors:
    ///   - `Error::SolverError` if the solver query fails
    ///   - `Error::FailedToResolveFunctionPointer` if it finds that it is possible
    ///     that the `BV` points to something that's not a `Function` in the
    ///     `Project`
    pub(crate) fn interpret_as_function_ptr(
        &mut self,
        bv: B::BV,
        n: usize,
    ) -> Result<PossibleSolutions<Callable<'p, B>>> {
        if n == 0 {
            unimplemented!("n == 0 in interpret_as_function_ptr")
        }

        // First try to interpret without a full solve (i.e., with `as_u64()`)
        let addrs: Vec<u64> = match bv.as_u64() {
            Some(addr) => vec![addr], // there is only one possible solution, and it's this `addr`
            None => {
                // Check if whatever solution we used last time for this `Location` still applies
                // (see notes on the `function_ptr_cache` field of `State`)
                match self.function_ptr_cache.get(&self.cur_loc) {
                    Some(addr)
                        if self
                            .bvs_must_be_equal(&bv, &self.bv_from_u64(*addr, bv.get_width()))? =>
                    {
                        vec![*addr]
                    },
                    _ => {
                        // Ok, use `get_possible_solutions_for_bv()`
                        match self
                            .get_possible_solutions_for_bv(&bv, n)?
                            .as_u64_solutions()
                            .unwrap()
                        {
                            PossibleSolutions::Exactly(v) => v.into_iter().collect(),
                            PossibleSolutions::AtLeast(v) => v.into_iter().collect(),
                        }
                    },
                }
            },
        };

        // save the value we found into the cache for next time
        if addrs.len() == 1 {
            self.function_ptr_cache
                .insert(self.cur_loc.clone(), addrs[0]);
        }

        let callables = addrs
            .into_iter()
            .map(|addr| {
                self.global_allocations
                    .get_func_for_address(addr, self.cur_loc.module)
                    .ok_or_else(|| Error::FailedToResolveFunctionPointer(addr))
            })
            .collect::<Result<HashSet<_>>>()?;
        if callables.len() > n {
            Ok(PossibleSolutions::AtLeast(callables))
        } else {
            Ok(PossibleSolutions::Exactly(callables))
        }
    }

    /// Get a pointer to the given function name. The name must be the
    /// fully-mangled function name, as it appears in the LLVM. The name will be
    /// resolved in the current module; this means that it will first look for a
    /// module-private (e.g., C `static`) definition in the current module, then
    /// search for a public definition in the same or different module. It will
    /// never return a module-private definition from a different module.
    ///
    /// Returns `None` if no function was found with that name.
    pub fn get_pointer_to_function(&self, funcname: impl Into<String>) -> Option<&B::BV> {
        self.global_allocations
            .get_global_allocation(&Name::from(funcname.into()), self.cur_loc.module)
            .map(|ga| ga.get_addr())
    }

    /// Get a pointer to the currently active _hook_ for the given function name.
    ///
    /// Returns `None` if no function was found with that name, _or_ if there is no currently
    /// active hook for that function.
    pub fn get_pointer_to_function_hook(&self, funcname: &str) -> Option<&B::BV> {
        self.global_allocations
            .get_function_hook_address(self.config.function_hooks.get_hook_for(funcname)?)
    }

    /// Get a `Function` by name. The name must be the fully-mangled function
    /// name, as it appears in the LLVM. The name will be resolved in the current
    /// module; this means that it will first look for a module-private (e.g., C
    /// `static`) definition in the current module, then search for a public
    /// definition in the same or different module. It will never return a
    /// module-private definition from a different module.
    ///
    /// Also returns the `Module` in which the prevailing definition of the `Function` was found.
    ///
    /// Returns `None` if no function was found with that name.
    pub fn get_func_by_name(
        &self,
        funcname: impl Into<String>,
    ) -> Option<(&'p Function, &'p Module)> {
        let funcname = funcname.into();
        self.global_allocations
            .get_global_allocation(&Name::from(funcname.clone()), self.cur_loc.module)
            .and_then(|ga| match ga {
                GlobalAllocation::Function { func, module, .. } => Some((*func, *module)),
                GlobalAllocation::GlobalVariable { .. } => panic!(
                    "get_func_by_name: {} refers to a global variable, not a function",
                    funcname
                ),
            })
    }

    /// Read a value `bits` bits long from memory at `addr`.
    /// Note that `bits` can be arbitrarily large.
    pub fn read(&self, addr: &B::BV, bits: u32) -> Result<B::BV> {
        let retval = match self.mem.borrow().read(addr, bits) {
            Ok(val) => val,
            e @ Err(Error::NullPointerDereference) => {
                if self.config.null_pointer_checking == NullPointerChecking::SplitPath {
                    // save a backtracking point to re-execute the current
                    // instruction with the address constrained to be non-null,
                    // and continue from there
                    self.save_backtracking_point_at_location(
                        self.cur_loc.clone(),
                        addr._ne(&self.zero(addr.get_width())),
                    );
                }
                return e; // report the null-pointer dereference
            },
            e @ Err(_) => return e, // propagate any other kind of error
        };
        for (name, watchpoint) in self.mem_watchpoints.get_triggered_watchpoints(addr, bits)? {
            let pretty_loc = if self.config.print_module_name {
                self.cur_loc.to_string_with_module()
            } else {
                self.cur_loc.to_string_no_module()
            };
            info!(
                "Memory watchpoint {:?} {} read by {{{}}}",
                name, watchpoint, pretty_loc
            );
        }
        Ok(retval)
    }

    /// Write a value into memory at `addr`.
    /// Note that `val` can be an arbitrarily large bitvector.
    pub fn write(&mut self, addr: &B::BV, val: B::BV) -> Result<()> {
        self.write_without_mut(addr, val)
    }

    /// For internal use: since `self.mem` is a `RefCell`, we can write even
    /// without having a `&mut self` reference. This is necessary to support,
    /// for instance, lazy global initialization. But, we don't want to skip
    /// watchpoint checks by calling `self.mem.borrow_mut()` directly, so we
    /// have this
    fn write_without_mut(&self, addr: &B::BV, val: B::BV) -> Result<()> {
        let write_width = val.get_width();
        let result = self.mem.borrow_mut().write(addr, val);
        // we do this awkward `let result` / `match result` because it forces
        // the mutable borrow of self.mem to end, which is necessary because
        // save_backtracking_point_at_location requires a borrow of self.mem
        match result {
            Ok(()) => (),
            e @ Err(Error::NullPointerDereference) => {
                if self.config.null_pointer_checking == NullPointerChecking::SplitPath {
                    // save a backtracking point to re-execute the current
                    // instruction with the address constrained to be non-null,
                    // and continue from there
                    self.save_backtracking_point_at_location(
                        self.cur_loc.clone(),
                        addr._ne(&self.zero(addr.get_width())),
                    );
                }
                return e; // report the null-pointer dereference
            },
            e @ Err(_) => return e, // propagate any other kind of error
        };
        for (name, watchpoint) in self
            .mem_watchpoints
            .get_triggered_watchpoints(addr, write_width)?
        {
            let pretty_loc = if self.config.print_module_name {
                self.cur_loc.to_string_with_module()
            } else {
                self.cur_loc.to_string_no_module()
            };
            // Log the new value of the watched location (regardless of which part of the watched location the write may have touched).
            // Note that the write operation itself has already been performed, so we get the updated value with a `read()`.
            let watchpoint_low =
                self.bv_from_u64(watchpoint.get_lower_bound(), self.pointer_size_bits);
            let watchpoint_size_bits =
                (watchpoint.get_upper_bound() - watchpoint.get_lower_bound() + 1) * 8;
            let new_value = self
                .mem
                .borrow()
                .read(&watchpoint_low, watchpoint_size_bits as u32)?; // performs a read without using `state.read()` which would trigger watchpoints (we don't want to trigger watchpoints with this read)
            info!(
                "Memory watchpoint {:?} {} written by {{{}}}; new value is {:?}",
                name, watchpoint, pretty_loc, new_value
            );
        }
        Ok(())
    }

    /// Get the size of the `Type`, in bits.
    ///
    /// Accounts for the `Project`'s pointer size and named struct definitions.
    ///
    /// Note that some types have size 0 bits, and this may return `0`.
    ///
    /// Panics if `ty` is a struct which has no definition in the entire `Project`,
    /// or if it is a struct/array/vector where one of the elements is a struct with no
    /// definition in the entire `Project`.
    #[deprecated = "Prefer size_in_bits()"]
    pub fn size(&self, ty: &Type) -> u32 {
        self.proj
            .size_in_bits(ty)
            .expect("state.size() encountered a struct with no definition in the entire Project")
    }

    /// Get the size of the `Type`, in bits.
    ///
    /// Accounts for the `Project`'s pointer size and named struct definitions.
    ///
    /// Note that some types have size 0 bits, and this may return `Some(0)`.
    ///
    /// Returns `None` for structs which have no definition in the entire `Project`,
    /// or for structs/arrays/vectors where one of the elements is a struct with no
    /// definition in the entire `Project`.
    #[deprecated = "Renamed to size_in_bits()"]
    pub fn size_opaque_aware(&self, ty: &Type, _proj: &'p Project) -> Option<u32> {
        self.proj.size_in_bits(ty)
    }

    /// Get the size of the `Type`, in bits.
    ///
    /// Accounts for the `Project`'s pointer size and named struct definitions.
    ///
    /// Note that some types have size 0 bits, and this may return `Some(0)`.
    ///
    /// Returns `None` for structs which have no definition in the entire `Project`,
    /// or for structs/arrays/vectors where one of the elements is a struct with no
    /// definition in the entire `Project`.
    pub fn size_in_bits(&self, ty: &Type) -> Option<u32> {
        self.proj.size_in_bits(ty)
    }

    /// Get the size of the `FPType`, in bits
    #[deprecated = "Renamed to fp_size_in_bits"]
    pub fn fp_size(fpt: FPType) -> u32 {
        Self::fp_size_in_bits(fpt)
    }

    pub fn fp_size_in_bits(fpt: FPType) -> u32 {
        match fpt {
            FPType::Half => 16,
            #[cfg(LLVM_VERSION_11_OR_GREATER)]
            FPType::BFloat => 16,
            FPType::Single => 32,
            FPType::Double => 64,
            FPType::FP128 => 128,
            FPType::X86_FP80 => 80,
            FPType::PPC_FP128 => 128,
        }
    }

    /// Get the offset (in _bytes_) of the element at the given index, as well as the
    /// `Type` of the element at that index.
    ///
    /// If `base_type` is a `NamedStructType`, the struct should be defined in the current module.
    pub fn get_offset_constant_index(
        &self,
        base_type: &Type,
        index: usize,
    ) -> Result<(u32, TypeRef)> {
        match base_type {
            Type::PointerType {
                pointee_type: element_type,
                ..
            }
            | Type::ArrayType { element_type, .. }
            | Type::VectorType { element_type, .. } => {
                let el_size_bits = self.size_in_bits(element_type).ok_or_else(|| {
                    Error::MalformedInstruction(format!(
                        "get_offset encountered an opaque struct type: {:?}",
                        element_type
                    ))
                })?;
                if el_size_bits % 8 != 0 {
                    Err(Error::UnsupportedInstruction(format!(
                        "Encountered a type with size {} bits",
                        el_size_bits
                    )))
                } else {
                    let el_size_bytes = el_size_bits / 8;
                    let index: u32 = index.try_into().unwrap();
                    Ok((index * el_size_bytes, element_type.clone()))
                }
            },
            Type::StructType { element_types, .. } => {
                let mut offset_bits = 0;
                for ty in element_types.iter().take(index) {
                    let element_size_bits = self.size_in_bits(ty).ok_or_else(|| {
                        Error::MalformedInstruction(format!(
                            "get_offset encountered an opaque struct type: {:?}",
                            ty
                        ))
                    })?;
                    offset_bits += element_size_bits;
                }
                if offset_bits % 8 != 0 {
                    Err(Error::UnsupportedInstruction(format!(
                        "Struct offset of {} bits",
                        offset_bits
                    )))
                } else {
                    Ok((offset_bits / 8, element_types[index].clone()))
                }
            },
            Type::NamedStructType { name } => {
                match self.cur_loc.module.types.named_struct_def(name) {
                    None => Err(Error::MalformedInstruction(format!(
                        "get_offset on a struct type not found in the current module: {:?}",
                        name
                    ))),
                    Some(NamedStructDef::Opaque) => Err(Error::MalformedInstruction(format!(
                        "get_offset on an opaque struct type: {:?}",
                        name
                    ))),
                    Some(NamedStructDef::Defined(ty)) => self.get_offset_constant_index(&ty, index),
                }
            },
            _ => panic!("get_offset_constant_index with base type {:?}", base_type),
        }
    }

    /// Get the offset (in _bytes_) of the element at the given index, as well as a
    /// reference to the `Type` of the element at that index.
    ///
    /// This function differs from `get_offset_constant_index` in that it takes an
    /// arbitrary `BV` as index instead of a `usize`, and likewise returns its offset
    /// as a `BV`.
    ///
    /// The result `BV` will have the same width as the input `index`.
    pub fn get_offset_bv_index<'t, V: BV>(
        &self,
        base_type: &'t Type,
        index: &V,
        solver: V::SolverRef,
    ) -> Result<(V, &'t Type)> {
        match base_type {
            Type::PointerType { pointee_type: element_type, .. }
            | Type::ArrayType { element_type, .. }
            | Type::VectorType { element_type, .. }
            => {
                let el_size_bits = self.size_in_bits(element_type)
                    .ok_or_else(|| Error::OtherError(format!("get_offset encountered an opaque struct type: {:?}", element_type)))?;
                if el_size_bits % 8 != 0 {
                    Err(Error::UnsupportedInstruction(format!("Encountered a type with size {} bits", el_size_bits)))
                } else {
                    let el_size_bytes = el_size_bits / 8;
                    Ok((index.mul(&V::from_u64(solver, el_size_bytes as u64, index.get_width())), &element_type))
                }
            },
            Type::StructType { .. } | Type::NamedStructType { .. } => {
                Err(Error::MalformedInstruction("Index into struct type must be constant; consider using `get_offset_constant_index` instead of `get_offset_bv_index`".to_owned()))
            },
            _ => panic!("get_offset_bv_index with base type {:?}", base_type),
        }
    }

    /// Add a memory watchpoint. It will be enabled unless/until
    /// `disable_watchpoint()` is called on it.
    ///
    /// If a watchpoint with the same name was previously added, this will
    /// replace that watchpoint and return `true`. Otherwise, this will return
    /// `false`.
    ///
    /// When any watched memory is read or written to, an INFO-level log message
    /// will be generated.
    pub fn add_mem_watchpoint(&mut self, name: impl Into<String>, watchpoint: Watchpoint) -> bool {
        self.mem_watchpoints.add(name, watchpoint)
    }

    /// Remove the memory watchpoint with the given `name`.
    ///
    /// Returns `true` if the operation was successful, or `false` if no
    /// watchpoint with that name was found.
    pub fn rm_mem_watchpoint(&mut self, name: &str) -> bool {
        self.mem_watchpoints.remove(name)
    }

    /// Disable the memory watchpoint with the given `name`. Disabled
    /// watchpoints will not generate any log messages unless/until
    /// `enable_watchpoint()` is called on them.
    ///
    /// Returns `true` if the operation is successful, or `false` if no
    /// watchpoint with that name was found. Disabling an already-disabled
    /// watchpoint will have no effect and will return `true`.
    pub fn disable_watchpoint(&mut self, name: &str) -> bool {
        self.mem_watchpoints.disable(name)
    }

    /// Enable the memory watchpoint(s) with the given name.
    ///
    /// Returns `true` if the operation is successful, or `false` if no
    /// watchpoint with that name was found. Enabling an already-enabled
    /// watchpoint will have no effect and will return `true`.
    pub fn enable_watchpoint(&mut self, name: &str) -> bool {
        self.mem_watchpoints.enable(name)
    }

    /// Allocate a value of size `bits`; return a pointer to the newly allocated object
    pub fn allocate(&mut self, bits: impl Into<u64>) -> B::BV {
        let raw_ptr = self.alloc.alloc(bits);
        self.bv_from_u64(raw_ptr, self.pointer_size_bits)
    }

    /// Get the size, in bits, of the allocation at the given address, or `None`
    /// if that address is not the result of an `alloc()`.
    pub fn get_allocation_size(&mut self, addr: &B::BV) -> Result<Option<u64>> {
        // First try to obtain the address without a full solve (i.e., with `as_u64()`)
        match addr.as_u64() {
            Some(addr) => Ok(self.alloc.get_allocation_size(addr)),
            None => {
                match self.get_possible_solutions_for_bv(addr, 1)? {
                    PossibleSolutions::AtLeast(_) => Err(Error::OtherError(format!(
                        "get_allocation_size: address is not a constant: {:?}",
                        addr
                    ))), // must be at least 2 solutions, since we passed in n==1
                    PossibleSolutions::Exactly(v) => {
                        let addr =
                            v.iter()
                                .next()
                                .ok_or(Error::Unsat)?
                                .as_u64()
                                .ok_or_else(|| {
                                    Error::OtherError(
                                        "get_allocation_size: address is more than 64 bits wide"
                                            .to_owned(),
                                    )
                                })?;
                        Ok(self.alloc.get_allocation_size(addr))
                    },
                }
            },
        }
    }

    /// Record the current location as a `PathEntry` in the current path.
    pub fn record_path_entry(&mut self) {
        let entry = PathEntry(self.cur_loc.clone());
        debug!("Recording a path entry {:?}", entry);
        self.path.push(entry);
    }

    /// Get the `PathEntry`s that have been recorded, in order
    pub fn get_path(&self) -> &Vec<PathEntry<'p>> {
        &self.path
    }

    /// Record entering a normal `Call` at the current location
    pub fn push_callsite(&mut self, call: &'p instruction::Call) {
        self.push_generic_callsite(Either::Left(call))
    }

    /// Record entering the given `Invoke` at the current location
    pub fn push_invokesite(&mut self, invoke: &'p terminator::Invoke) {
        self.push_generic_callsite(Either::Right(invoke))
    }

    fn push_generic_callsite(
        &mut self,
        instr: Either<&'p instruction::Call, &'p terminator::Invoke>,
    ) {
        self.stack.push(StackFrame {
            callsite: Callsite {
                loc: self.cur_loc.clone(),
                instr,
            },
            // TODO: taking this `restore_info` every time a callsite is pushed
            // may be expensive, and is only necessary if the call we're going
            // to make will eventually (directly or indirectly) recurse. In the
            // future we could check the LLVM 'norecurse' attribute to know when
            // this is not necessary.
            restore_info: self
                .varmap
                .get_restore_info_for_fn(self.cur_loc.func.name.clone()),
        })
    }

    /// Record leaving the current function. Returns the `Callsite` at which the
    /// current function was called, or `None` if the current function was the
    /// top-level function.
    ///
    /// Also restores the caller's local variables.
    pub fn pop_callsite(&mut self) -> Option<Callsite<'p>> {
        if let Some(StackFrame {
            callsite,
            restore_info,
        }) = self.stack.pop()
        {
            self.varmap.restore_fn_vars(restore_info);
            Some(callsite)
        } else {
            None
        }
    }

    /// Returns the current callstack depth. `0` indicates we're in the toplevel
    /// function, `1` indicates we're in a function directly called by the
    /// toplevel function, etc.
    pub fn current_callstack_depth(&self) -> usize {
        self.stack.len()
    }

    /// Save the current state, about to enter the `BasicBlock` with the given `Name` (which must be
    /// in the same `Module` and `Function` as `state.cur_loc`), as a backtracking point.
    /// The constraint will be added only if we end up backtracking to this point, and only then.
    pub fn save_backtracking_point(&mut self, bb_to_enter: &Name, constraint: B::BV) {
        debug!(
            "Saving a backtracking point, which would enter bb {:?} with constraint {:?}",
            bb_to_enter, constraint
        );
        let bb_to_enter = self
            .cur_loc
            .func
            .get_bb_by_name(&bb_to_enter)
            .unwrap_or_else(|| {
                panic!(
                    "Failed to find bb named {} in function {:?}",
                    bb_to_enter, self.cur_loc.func.name
                )
            });
        let backtrack_loc = Location {
            module: self.cur_loc.module,
            func: self.cur_loc.func,
            bb: bb_to_enter,
            instr: BBInstrIndex::Instr(0),
            source_loc: None,
        };
        self.save_backtracking_point_at_location(backtrack_loc, constraint);
    }

    /// Internal version of `save_backtracking_point()` which takes an arbitrary
    /// `Location` instead of just the basic block to start at.
    ///
    /// Also it doesn't require `&mut self`. This allows us to save backtracking
    /// points even when we're inside methods that only have `&self`.
    fn save_backtracking_point_at_location(
        &self,
        loc_to_start_at: Location<'p>,
        constraint: B::BV,
    ) {
        self.solver.push(1);
        self.backtrack_points.borrow_mut().push(BacktrackPoint {
            loc: loc_to_start_at,
            stack: self.stack.clone(),
            constraint,
            varmap: self.varmap.clone(),
            mem: self.mem.borrow().clone(),
            path_len: self.path.len(),
        });
    }

    /// returns `Ok(true)` if the operation was successful, `Ok(false)` if there are
    /// no saved backtracking points, or `Err` for other errors
    pub fn revert_to_backtracking_point(&mut self) -> Result<bool> {
        if let Some(bp) = self.backtrack_points.borrow_mut().pop() {
            debug!("Reverting to backtracking point {}", bp);
            self.solver.pop(1);
            self.varmap = bp.varmap;
            self.mem.replace(bp.mem);
            self.stack = bp.stack;
            self.path.truncate(bp.path_len);
            self.cur_loc = bp.loc;
            bp.constraint.assert()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// returns the number of saved backtracking points
    pub fn count_backtracking_points(&self) -> usize {
        self.backtrack_points.borrow().len()
    }

    /// returns a `String` containing a formatted view of the current backtrace
    /// (in terms of LLVM locations, and possibly also source locations depending
    /// on the `Config`)
    pub fn pretty_backtrace(&self) -> String {
        let mut locdescrs = std::iter::once(LocationDescription::from(self.cur_loc.clone()))
            .chain(
                self.stack
                    .iter()
                    .rev()
                    .map(|frame| LocationDescription::from(frame.callsite.loc.clone())),
            )
            .collect::<Vec<LocationDescription>>();
        for locdescr in locdescrs.iter_mut() {
            self.demangle_locdescr(locdescr);
        }
        locdescrs
            .into_iter()
            .zip(1 ..)
            .map(|(locdescr, framenum)| {
                let pretty_locdescr = if self.config.print_module_name {
                    locdescr.to_string_with_module()
                } else {
                    locdescr.to_string_no_module()
                };
                let mut frame_string = format!("  #{}: {}\n", framenum, pretty_locdescr);
                match locdescr.source_loc {
                    Some(source_loc) if self.config.print_source_info => {
                        frame_string
                            .push_str(&format!("         ({})\n", pretty_source_loc(source_loc)));
                    },
                    _ => {},
                };
                frame_string
            })
            .collect()
    }

    /// returns a `String` containing a formatted view of the full path which led
    /// to this point, in terms of LLVM locations
    pub fn pretty_path_llvm(&self) -> String {
        let mut path_str = String::new();
        for path_entry in self.get_path() {
            path_str.push_str(&format!(
                "  {}\n",
                if self.config.print_module_name {
                    path_entry.to_string_with_module()
                } else {
                    path_entry.to_string_no_module()
                },
            ));
        }
        path_str
    }

    /// returns a `String` containing a formatted view of the full path which led
    /// to this point, in terms of source locations
    pub fn pretty_path_source(&self) -> String {
        let mut path_str = String::new();
        let mut source_locs = self
            .get_path()
            .iter()
            .flat_map(|path_entry| path_entry.get_all_source_locs());
        // handle the first one special, so we can print this help message if necessary
        match source_locs.next() {
            None => {
                path_str.push_str("  No source locations available in the path.\n");
                path_str.push_str(
                    "  This may be because the LLVM bitcode was not compiled with debuginfo.\n",
                );
                path_str.push_str(
                    "  To compile C/C++ or Rust sources with debuginfo, pass the `-g` flag\n",
                );
                path_str.push_str("    to `clang`, `clang++`, or `rustc`.\n");
            },
            Some(first_source_loc) => {
                path_str.push_str(&format!("  {}\n", pretty_source_loc(first_source_loc)))
            },
        }
        for source_loc in source_locs {
            path_str.push_str(&format!("  {}\n", pretty_source_loc(source_loc)));
        }
        path_str
    }

    /// returns a `String` containing a formatted view of the full path which led
    /// to this point, in terms of both LLVM and source locations (interleaved
    /// appropriately)
    pub fn pretty_path_interleaved(&self) -> String {
        let mut path_str = String::new();
        for path_entry in self.get_path() {
            path_str.push_str(&format!(
                "  {}:\n",
                if self.config.print_module_name {
                    path_entry.to_string_with_module()
                } else {
                    path_entry.to_string_no_module()
                },
            ));
            let mut source_locs = path_entry.get_all_source_locs();
            // handle the first one special, so we can print this help message if necessary
            match source_locs.next() {
                None => path_str.push_str("    (no source locations available)\n"),
                Some(first_source_loc) => {
                    path_str.push_str(&format!("    {}\n", pretty_source_loc(first_source_loc)))
                },
            }
            for source_loc in source_locs {
                path_str.push_str(&format!("    {}\n", pretty_source_loc(source_loc)));
            }
        }
        path_str
    }

    /// Attempt to demangle the given `funcname` as appropriate based on the
    /// `Config`.
    ///
    /// If this fails to demangle `funcname`, it just returns a copy of
    /// `funcname` unchanged.
    pub fn demangle(&self, funcname: &str) -> String {
        match self.config.demangling {
            Some(demangling) => demangling.maybe_demangle(funcname),
            None => panic!("Demangling shouldn't be None here"), // we should resolve it to Some() in the State constructor
        }
    }

    /// Attempts to demangle the function name in the `LocationDescription`, as
    /// appropriate based on the `Config`.
    fn demangle_locdescr(&self, locdescr: &mut LocationDescription) {
        locdescr.funcname = self.demangle(&locdescr.funcname);
    }

    /// Get the most recent `BV` created for each `Name` in the current function.
    /// Returns pairs of the `Name` and the `BV` assigned to that `Name`.
    ///
    /// Returned pairs will be sorted by `Name`.
    pub fn all_vars_in_cur_fn(&self) -> impl Iterator<Item = (&Name, &B::BV)> {
        self.varmap.get_all_vars_in_fn(&self.cur_loc.func.name)
    }

    /// returns a `String` describing a set of satisfying assignments for all variables
    pub fn current_assignments_as_pretty_string(&self) -> Result<String> {
        self.solver.set_opt(BtorOption::ModelGen(ModelGen::All));
        let string = if self.sat()? {
            let printed = self.solver.print_model();
            let sorted = itertools::sorted(printed.lines());
            sorted.fold(String::new(), |s, line| s + "\n" + line)
        } else {
            "<state is unsatisfiable>".to_owned()
        };
        self.solver
            .set_opt(BtorOption::ModelGen(ModelGen::Disabled));
        Ok(string)
    }

    /// Returns a `String` describing both the error and the context in which it
    /// occurred (backtrace, full path to error, variable values at the point of
    /// error, etc). Exactly which information is included is partially dependent
    /// on the environment variables `HAYBALE_DUMP_PATH` and `HAYBALE_DUMP_VARS`,
    /// as explained in the message.
    pub fn full_error_message_with_context(&self, e: Error) -> String {
        let mut err_msg = format!("{}\n\n", e);
        err_msg.push_str(&format!("Backtrace:\n{}\n", self.pretty_backtrace()));
        match PathDumpType::get_from_env_var() {
            PathDumpType::None => {
                err_msg.push_str("note: For a dump of the path that led to this error, rerun with the environment variable `HAYBALE_DUMP_PATH` set to:\n");
                err_msg
                    .push_str("        `LLVM` for a list of the LLVM basic blocks in the path\n");
                err_msg.push_str(
                    "        `SRC` for a list of the source-language locations in the path\n",
                );
                err_msg.push_str("        `BOTH` for both of the above\n");
                err_msg.push_str(
                    "      To get source-language locations, the LLVM bitcode must also contain\n",
                );
                err_msg.push_str("      debuginfo. For example, C/C++ or Rust sources must be compiled with the\n");
                err_msg.push_str("      `-g` flag to `clang`, `clang++`, or `rustc`.\n");
            },
            PathDumpType::LLVM => {
                err_msg.push_str("LLVM path to error:\n");
                err_msg.push_str(&self.pretty_path_llvm());
                err_msg.push_str("note: to also get a dump of the source-language locations in this path, rerun with `HAYBALE_DUMP_PATH=BOTH`.\n");
            },
            PathDumpType::Source => {
                err_msg.push_str("Source-language path to error:\n");
                err_msg.push_str(&self.pretty_path_source());
                err_msg.push_str("note: to also get a dump of the LLVM basic blocks in this path, rerun with `HAYBALE_DUMP_PATH=BOTH`.\n");
            },
            PathDumpType::Interleaved => {
                err_msg.push_str("Full path to error:\n");
                err_msg.push_str(&self.pretty_path_interleaved());
            },
        }
        if std::env::var("HAYBALE_DUMP_VARS") == Ok("1".to_owned()) {
            err_msg
                .push_str("\nLatest values of variables at time of error, in current function:\n");
            err_msg.push_str("(Ignore any values from past the point of error, they may be from other paths)\n\n");
            for (varname, value) in self.all_vars_in_cur_fn() {
                err_msg.push_str(&format!("  {}: {:?}\n", varname, value));
            }
        } else {
            err_msg.push_str("\nnote: For a dump of variable values at time of error, rerun with `HAYBALE_DUMP_VARS=1` environment variable.\n");
        }
        err_msg.push_str("\nnote: to enable detailed logs, ensure that debug-level logging messages are visible.\n");
        err_msg.push_str("  See the documentation for your chosen logging backend (e.g., env_logger, log4rs, etc)\n");
        err_msg.push_str("  for help with configuration.\n");
        err_msg.push_str("\n");
        err_msg
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum PathDumpType {
    /// Don't dump the path
    None,
    /// Dump just the LLVM path
    LLVM,
    /// Dump just the source path
    Source,
    /// Dump both LLVM and source path (interleaved)
    Interleaved,
}

impl PathDumpType {
    fn get_from_env_var() -> Self {
        match std::env::var("HAYBALE_DUMP_PATH") {
            Err(_) => Self::None,
            Ok(mut val) => {
                val.make_ascii_uppercase();
                match val.deref() {
                    "" => Self::None,
                    "LLVM" => Self::LLVM,
                    "SRC" => Self::Source,
                    "BOTH" => Self::Interleaved,
                    "1" => Self::Interleaved, // previous versions of `haybale` used HAYBALE_DUMP_PATH=1, we now treat that equivalently to `BOTH`
                    _ => Self::Interleaved,
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver_utils::SolutionCount;
    use crate::test_utils::*;

    // we don't include tests here for Memory, Alloc, VarMap, or Watchpoints; those are tested in their own modules.
    // Instead, here we just test the nontrivial functionality that `State` has itself.
    // We do repeat many of the tests from the `solver_utils` module, making sure that they also pass when
    // we use the `State` interfaces.

    #[test]
    fn sat() -> Result<()> {
        let func = blank_function("test_func", vec![Name::from("test_bb")]);
        let project = blank_project("test_mod", func);
        let mut state = blank_state(&project, "test_func");

        // empty state should be sat
        assert_eq!(state.sat(), Ok(true));

        // adding True constraint should still be sat
        state.bv_from_bool(true).assert();
        assert_eq!(state.sat(), Ok(true));

        // adding x > 0 constraint should still be sat
        let x = state.new_bv_with_name(Name::from("x"), 64)?;
        x.sgt(&state.zero(64)).assert();
        assert_eq!(state.sat(), Ok(true));

        Ok(())
    }

    #[test]
    fn unsat() -> Result<()> {
        let func = blank_function("test_func", vec![Name::from("test_bb")]);
        let project = blank_project("test_mod", func);
        let state = blank_state(&project, "test_func");

        // adding False constraint should be unsat
        state.bv_from_bool(false).assert();
        assert_eq!(state.sat(), Ok(false));

        Ok(())
    }

    #[test]
    fn unsat_with_extra_constraints() -> Result<()> {
        let func = blank_function("test_func", vec![Name::from("test_bb")]);
        let project = blank_project("test_mod", func);
        let mut state = blank_state(&project, "test_func");

        // adding x > 3 constraint should still be sat
        let x = state.new_bv_with_name(Name::from("x"), 64)?;
        x.ugt(&state.bv_from_u64(3, 64)).assert();
        assert_eq!(state.sat(), Ok(true));

        // adding x < 3 constraint should make us unsat
        let bad_constraint = x.ult(&state.bv_from_u64(3, 64));
        assert_eq!(
            state.sat_with_extra_constraints(std::iter::once(&bad_constraint)),
            Ok(false)
        );

        // the state itself should still be sat, extra constraints weren't permanently added
        assert_eq!(state.sat(), Ok(true));

        Ok(())
    }

    #[test]
    fn get_a_solution() -> Result<()> {
        let func = blank_function("test_func", vec![Name::from("test_bb")]);
        let project = blank_project("test_mod", func);
        let mut state = blank_state(&project, "test_func");

        // add x > 3 constraint
        let x = state.new_bv_with_name(Name::from("x"), 64)?;
        x.ugt(&state.bv_from_u64(3, 64)).assert();

        // check that the computed value of x is > 3
        let x_value = state
            .get_a_solution_for_bv(&x)
            .unwrap()
            .expect("Expected a solution for x")
            .as_u64()
            .unwrap();
        assert!(x_value > 3);

        Ok(())
    }

    #[test]
    fn possible_solutions() -> Result<()> {
        let func = blank_function("test_func", vec![Name::from("test_bb")]);
        let project = blank_project("test_mod", func);
        let mut state = blank_state(&project, "test_func");

        // add x > 3 constraint
        let x = state.new_bv_with_name(Name::from("x"), 64)?;
        x.ugt(&state.bv_from_u64(3, 64)).assert();

        // check that there are more than 2 solutions
        let num_solutions = state.get_possible_solutions_for_bv(&x, 2).unwrap().count();
        assert_eq!(num_solutions, SolutionCount::AtLeast(3));

        // add x < 6 constraint
        x.ult(&state.bv_from_u64(6, 64)).assert();

        // check that there are now exactly two solutions
        let solutions = state
            .get_possible_solutions_for_bv(&x, 2)
            .unwrap()
            .as_u64_solutions();
        assert_eq!(solutions, Some([4, 5].iter().copied().collect()));

        // add x < 5 constraint
        x.ult(&state.bv_from_u64(5, 64)).assert();

        // check that there is now exactly one solution
        let solutions = state
            .get_possible_solutions_for_bv(&x, 2)
            .unwrap()
            .as_u64_solutions();
        assert_eq!(solutions, Some(PossibleSolutions::exactly_one(4)));

        // add x < 3 constraint
        x.ult(&state.bv_from_u64(3, 64)).assert();

        // check that there are now no solutions
        let solutions = state
            .get_possible_solutions_for_bv(&x, 2)
            .unwrap()
            .as_u64_solutions();
        assert_eq!(solutions, Some(PossibleSolutions::empty()));

        Ok(())
    }

    #[test]
    fn lookup_vars_via_operand() {
        let func = blank_function("test_func", vec![Name::from("test_bb")]);
        let project = blank_project("test_mod", func);
        let mut state = blank_state(&project, "test_func");

        // create llvm-ir names
        let name1 = Name::from("val");
        let name2 = Name::from(2);

        // create corresponding BV values
        let var1 = state.new_bv_with_name(name1.clone(), 64).unwrap();
        let var2 = state.new_bv_with_name(name2.clone(), 1).unwrap(); // these clone()s wouldn't normally be necessary but we want to reuse the names to create `Operand`s later

        // check that we can look up the correct BV values via LocalOperands
        let op1 = Operand::LocalOperand {
            name: name1,
            ty: state.cur_loc.module.types.i32(),
        };
        let op2 = Operand::LocalOperand {
            name: name2,
            ty: state.cur_loc.module.types.bool(),
        };
        assert_eq!(state.operand_to_bv(&op1), Ok(var1));
        assert_eq!(state.operand_to_bv(&op2), Ok(var2));
    }

    #[test]
    fn const_bv() {
        let func = blank_function("test_func", vec![Name::from("test_bb")]);
        let project = blank_project("test_mod", func);
        let state = blank_state(&project, "test_func");

        // create an llvm-ir value which is constant 3
        let constint = Constant::Int { bits: 64, value: 3 };

        // this should create a corresponding BV value which is also constant 3
        let bv = state
            .operand_to_bv(&Operand::ConstantOperand(ConstantRef::new(constint)))
            .unwrap();

        // check that the BV value was evaluated to 3
        let solution = state
            .get_a_solution_for_bv(&bv)
            .unwrap()
            .expect("Expected a solution for the bv")
            .as_u64()
            .unwrap();
        assert_eq!(solution, 3);
    }

    #[test]
    fn const_bool() {
        let func = blank_function("test_func", vec![Name::from("test_bb")]);
        let project = blank_project("test_mod", func);
        let state = blank_state(&project, "test_func");

        // create llvm-ir constants true and false
        let consttrue = Constant::Int { bits: 1, value: 1 };
        let constfalse = Constant::Int { bits: 1, value: 0 };

        // this should create BV values true and false
        let bvtrue = state
            .operand_to_bv(&Operand::ConstantOperand(ConstantRef::new(consttrue)))
            .unwrap();
        let bvfalse = state
            .operand_to_bv(&Operand::ConstantOperand(ConstantRef::new(constfalse)))
            .unwrap();

        // check that the BV values are evaluated to true and false respectively
        assert_eq!(
            state
                .get_a_solution_for_bv(&bvtrue)
                .unwrap()
                .expect("Expected a solution for bvtrue")
                .as_bool()
                .unwrap(),
            true,
        );
        assert_eq!(
            state
                .get_a_solution_for_bv(&bvfalse)
                .unwrap()
                .expect("Expected a solution for bvfalse")
                .as_bool()
                .unwrap(),
            false,
        );

        // assert the first one, which should be true, so we should still be sat
        bvtrue.assert();
        assert_eq!(state.sat(), Ok(true));

        // assert the second one, which should be false, so we should be unsat
        bvfalse.assert();
        assert_eq!(state.sat(), Ok(false));
    }

    #[test]
    fn backtracking() -> Result<()> {
        let func = blank_function(
            "test_func",
            vec![Name::from("bb_start"), Name::from("bb_target")],
        );
        let project = blank_project("test_mod", func);
        let mut state = blank_state(&project, "test_func");
        state.record_path_entry();

        // assert x > 11
        let x = state.new_bv_with_name(Name::from("x"), 64)?;
        x.sgt(&state.bv_from_i64(11, 64)).assert();

        // create a backtrack point with constraint y > 5
        let y = state.new_bv_with_name(Name::from("y"), 64)?;
        let constraint = y.sgt(&state.bv_from_i64(5, 64));
        let bb = project
            .get_func_by_name("test_func")
            .map(|(func, _)| func)
            .expect("Expected to find function named 'test_func'")
            .get_bb_by_name(&Name::from("bb_target"))
            .expect("Expected to find bb named 'bb_target'");
        state.save_backtracking_point(&bb.name, constraint);

        // check that the constraint y > 5 wasn't added: adding y < 4 should keep us sat
        assert_eq!(
            state.sat_with_extra_constraints(std::iter::once(&y.slt(&state.bv_from_i64(4, 64)))),
            Ok(true),
        );

        // assert x < 8 to make us unsat
        x.slt(&state.bv_from_i64(8, 64)).assert();
        assert_eq!(state.sat(), Ok(false));

        // note the pre-rollback location
        let pre_rollback = state.cur_loc.clone();

        // roll back to backtrack point; check that we ended up at the right loc
        // and with the right path
        assert!(state.revert_to_backtracking_point().unwrap());
        assert_eq!(state.cur_loc.func, pre_rollback.func);
        assert_eq!(state.cur_loc.bb.name, bb.name);
        let path = state.get_path();
        assert_eq!(path.len(), 1);
        let path_entry = &path[0];
        assert_eq!(path_entry.0.module.name, "test_mod");
        assert_eq!(path_entry.0.func.name, "test_func");
        assert_eq!(path_entry.0.bb.name, Name::from("bb_start"));
        assert_eq!(path_entry.0.instr, BBInstrIndex::Instr(0));

        // check that the constraint x < 8 was removed: we're sat again
        assert_eq!(state.sat(), Ok(true));

        // check that the constraint y > 5 was added: y evaluates to something > 5
        assert!(
            state
                .get_a_solution_for_bv(&y)
                .unwrap()
                .expect("Expected a solution for y")
                .as_u64()
                .unwrap()
                > 5
        );

        // check that the first constraint remained in place: x > 11
        assert!(
            state
                .get_a_solution_for_bv(&x)
                .unwrap()
                .expect("Expected a solution for x")
                .as_u64()
                .unwrap()
                > 11
        );

        // check that trying to backtrack again fails
        assert!(!state.revert_to_backtracking_point().unwrap());

        Ok(())
    }

    #[test]
    fn fork() {
        let func = blank_function("test_func", vec![Name::from("test_bb")]);
        let project = blank_project("test_mod", func);
        let mut state = blank_state(&project, "test_func");

        // assert x < 42
        let x = state.new_bv_with_name(Name::from("x"), 64).unwrap();
        x.ult(&state.bv_from_u32(42, 64)).assert();

        // `y` is equal to `x + 7`
        let y = x.add(&state.bv_from_u32(7, 64));
        state.assign_bv_to_name(Name::from("y"), y).unwrap();

        // fork the state
        let mut state_2 = state.fork();

        // get the copies of `x` and `y` in each state, via operand lookups
        let op_x = Operand::LocalOperand {
            name: Name::from("x"),
            ty: state.cur_loc.module.types.i64(),
        };
        let op_y = Operand::LocalOperand {
            name: Name::from("y"),
            ty: state.cur_loc.module.types.i64(),
        };
        let x_1 = state.operand_to_bv(&op_x).unwrap();
        let x_2 = state_2.operand_to_bv(&op_x).unwrap();
        let y_1 = state.operand_to_bv(&op_y).unwrap();
        let y_2 = state_2.operand_to_bv(&op_y).unwrap();

        // in one state, we assert `x > 3`, while in the other, we assert `x < 3`
        x_1.ugt(&state.bv_from_u32(3, 64)).assert();
        x_2.ult(&state_2.bv_from_u32(3, 64)).assert();

        // check that in the first state, `y > 10` (looking up two different ways)
        let y_solution = state
            .get_a_solution_for_bv(&y_1)
            .unwrap()
            .expect("Expected a solution for y")
            .as_u64()
            .unwrap();
        assert!(y_solution > 10);
        let y_solution = state
            .get_a_solution_for_irname(&"test_func".to_owned(), &Name::from("y"))
            .unwrap()
            .expect("Expected a solution for y")
            .as_u64()
            .unwrap();
        assert!(y_solution > 10);

        // check that in the second state, `y < 10` (looking up two different ways)
        let y_2_solution = state_2
            .get_a_solution_for_bv(&y_2)
            .unwrap()
            .expect("Expected a solution for y_2")
            .as_u64()
            .unwrap();
        assert!(y_2_solution < 10);
        let y_2_solution = state_2
            .get_a_solution_for_irname(&"test_func".to_owned(), &Name::from("y"))
            .unwrap()
            .expect("Expected a solution for y_2")
            .as_u64()
            .unwrap();
        assert!(y_2_solution < 10);
    }
}
