use crate::backend::Backend;
use crate::function_hooks::FunctionHooks;
use crate::watchpoints::Watchpoint;
use std::collections::HashSet;

/// Various settings which affect how the symbolic execution is performed.
#[derive(Clone)]
pub struct Config<'p, B> where B: Backend {
    /// Maximum number of times to execute any given line of LLVM IR.
    /// This bounds both the number of iterations of loops, and also the depth of recursion.
    /// For inner loops, this bounds the number of total iterations across all invocations of the loop.
    pub loop_bound: usize,

    /// If `true`, all memory accesses will be checked to ensure their addresses
    /// cannot be NULL, throwing `Error::NullPointerDereference` if NULL is a
    /// possible solution for the address
    pub null_detection: bool,

    /// When encountering a `memcpy`, `memset`, or `memmove` with multiple
    /// possible lengths, how (if at all) should we concretize?
    pub concretize_memcpy_lengths: Concretize,

    /// The set of currently active function hooks; see
    /// [`FunctionHooks`](struct.FunctionHooks.html) for more details
    pub function_hooks: FunctionHooks<'p, B>,

    /// The initial set of memory watchpoints when a `State` is created (their
    /// names, and the actual watchpoints).
    ///
    /// More watchpoints may be added or removed at any time with
    /// `state.add_mem_watchpoint()` and `state.rm_mem_watchpoint`.
    pub initial_mem_watchpoints: HashSet<(String, Watchpoint)>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Concretize {
    /// Handle everything fully symbolically - that is, have the solver fully
    /// consider all possible values. This may lead to poor solver performance
    /// for some workloads.
    Symbolic,

    /// Pick one possible value arbitrarily. Often this may choose `0` if `0` is
    /// a possible solution, but this behavior is not guaranteed. (To guarantee
    /// this behavior, use `Prefer(0)`.)
    ///
    /// The value will be permanently constrained to be the chosen value (on this
    /// path), and other possibilities will not be considered.
    Arbitrary,

    /// Prefer the given `u64` value if it is a possible value. Otherwise, fall
    /// back on the given `Concretize` strategy.
    ///
    /// If the given `u64` value is a possible value, then the value will be
    /// permanently constrained to be that value (on this path), and other
    /// possibilities will not be considered.
    Prefer(u64, Box<Concretize>),

    /// Choose the maximum possible value. `Maximum` will be interpreted in an
    /// unsigned fashion.
    ///
    /// The value will be permanently constrained to be this value (on this
    /// path), and other possibilities will not be considered.
    Maximum,

    /// Choose the minimum possible value. `Minimum` will be interpreted in an
    /// unsigned fashion.
    ///
    /// The value will be permanently constrained to be this value (on this
    /// path), and other possibilities will not be considered.
    Minimum,
}

impl<'p, B: Backend> Config<'p, B> {
    /// Creates a new `Config` with the given `loop_bound` and
    /// `concretize_memcpy_lengths()` options, and no function hooks or memory
    /// watchpoints.
    ///
    /// You may want to consider `Config::default()` which provides defaults for
    /// all parameters and comes with predefined hooks for common functions.
    pub fn new(loop_bound: usize, null_detection: bool, concretize_memcpy_lengths: Concretize) -> Self {
        Self {
            loop_bound,
            null_detection,
            concretize_memcpy_lengths,
            function_hooks: FunctionHooks::new(),
            initial_mem_watchpoints: HashSet::new(),
        }
    }
}

impl<'p, B: Backend> Default for Config<'p, B> {
    /// Default values for all configuration parameters.
    ///
    /// In particular, this uses
    /// [`FunctionHooks::default()`](struct.FunctionHooks.html#method.default),
    /// and therefore comes with a set of predefined hooks for common functions.
    /// (At the time of this writing, only `malloc()`, `calloc()`, `realloc()`,
    /// and `free()`.)
    ///
    /// For more information, see
    /// [`FunctionHooks::default()`](struct.FunctionHooks.html#method.default).
    fn default() -> Self {
        Self {
            loop_bound: 10,
            null_detection: true,
            concretize_memcpy_lengths: Concretize::Symbolic,
            function_hooks: FunctionHooks::default(),
            initial_mem_watchpoints: HashSet::new(),
        }
    }
}
