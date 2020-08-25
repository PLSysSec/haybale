//! The `Config` structure for configuring `haybale`, and other associated definitions

use crate::backend::Backend;
use crate::callbacks::Callbacks;
pub use crate::demangling::Demangling;
use crate::function_hooks::FunctionHooks;
use crate::watchpoints::Watchpoint;
use std::collections::HashMap;
use std::time::Duration;

/// Various settings which affect how the symbolic execution is performed.
///
/// `Config` uses the (new-to-Rust-1.40) `#[non_exhaustive]` attribute to
/// indicate that fields may be added even in a point release (that is, without
/// incrementing the major or minor version). See
/// [here](https://blog.rust-lang.org/2019/12/19/Rust-1.40.0.html#[non_exhaustive]-structs,-enums,-and-variants)
/// for more details.
///
/// In general, you'll want to start with `Config::default()` and then change the
/// settings you want to change; `#[non_exhaustive]` will prevent users from
/// constructing a `Config` directly.
#[non_exhaustive]
#[derive(Clone)]
pub struct Config<'p, B: Backend> {
    /// Maximum number of times to execute any given line of LLVM IR.
    /// This bounds both the number of iterations of loops, and also the depth of recursion.
    /// For inner loops, this bounds the number of total iterations across all invocations of the loop.
    ///
    /// Default is `10`.
    pub loop_bound: usize,

    /// Maximum callstack depth to allow when symbolically executing.
    /// If symbolic execution encounters a call which would result in a
    /// stack depth exceeding this number, and the call is not hooked (see
    /// [`function_hooks`](struct.Config.html#structfield.function_hooks)), then
    /// the call will simply be ignored - as if
    /// [`generic_stub_hook`](../function_hooks/fn.generic_stub_hook.html) were
    /// applied to that call.
    ///
    /// For example, if this setting is set to `Some(1)`, and we're executing a
    /// function `foo()` which calls `bar()` which calls `baz()`, then the call
    /// to `bar()` will be fully analyzed, but fully calling `baz()` would result
    /// in a stack depth of `2`, so instead, `bar()`'s call to `baz()` will be
    /// ignored.
    ///
    /// As another example, the setting `Some(0)` means that all calls will be
    /// ignored, unless they are hooked in `function_hooks`.
    ///
    /// Note that this considers the LLVM callstack depth.
    /// If calls have been inlined in the LLVM bitcode, `haybale` sees this as
    /// a single function, and "entering" an inlined function doesn't affect
    /// the callstack depth.
    ///
    /// A value of `None` for this setting indicates no limit to the callstack depth;
    /// all calls will be fully analyzed, to the extent possible and unless
    /// overridden by `function_hooks`.
    ///
    /// Default is `None`.
    pub max_callstack_depth: Option<usize>,

    /// Maximum amount of time to allow for any single solver query.
    ///
    /// If `Some`, any solver query lasting longer than the given limit will
    /// be killed.  This will result in an `Error::SolverError` for that path.
    ///
    /// If `None`, there will be no time limit for solver queries.
    ///
    /// Default is 300 seconds (5 minutes).
    pub solver_query_timeout: Option<Duration>,

    /// Should we check each memory access for possible `NULL` dereference,
    /// and if so, how should we report any errors?
    ///
    /// Default is `NullPointerChecking::Simple`.
    pub null_pointer_checking: NullPointerChecking,

    /// When encountering a `memcpy`, `memset`, or `memmove` with multiple
    /// possible lengths, how (if at all) should we concretize the length?
    ///
    /// Default is `Concretize::Symbolic` - that is, no concretization.
    pub concretize_memcpy_lengths: Concretize,

    /// Maximum supported length of a `memcpy`, `memset`, or `memmove` operation.
    ///
    /// Setting this to `Some(x)` means that if we encounter a `memcpy`,
    /// `memset`, or `memmove` with length which may be greater than `x` bytes,
    /// we will constrain the length to be at most `x` bytes. (`haybale` will
    /// also emit a warning when doing this.) If the only possible values for the
    /// length are greater than `x` bytes, we will raise an error.
    ///
    /// Setting this to `None` means that there is no limit to the size of these
    /// operations.
    ///
    /// Default is `None` - that is, no limit.
    pub max_memcpy_length: Option<u64>,

    /// `Error::Unsat` is an error type which is used internally, but may not be
    /// useful for `ExecutionManager.next()` to return to consumers. In most
    /// cases, consumers probably don't care about paths which were partially
    /// explored and resulted in an unsat error; they are probably interested in
    /// only those paths that are actually feasible, or ended in one of the other
    /// error types.
    ///
    /// If this setting is `false`, the `ExecutionManager` will return an
    /// `Error::Unsat` to the consumer whenever one is encountered, just as it
    /// does for other error types.
    /// If this setting is `true`, paths ending in `Error::Unsat` will be
    /// silently ignored by the `ExecutionManager`, and it will move on to the
    /// next path, as if a filter were applied to the iterator.
    ///
    /// Note that many unsat paths are never even started processing, so they
    /// never actually result in an unsat error. In fact, many executions may
    /// never encounter an unsat error, despite having unsat paths. Furthermore,
    /// `haybale`'s behavior regarding which unsat paths actually result in an
    /// unsat error is not guaranteed to be stable, and may change even in point
    /// releases (that is, without incrementing the major or minor version).
    ///
    /// Default is `true`.
    pub squash_unsats: bool,

    /// When encountering the `llvm.assume()` intrinsic, should we only consider
    /// paths where the assumption holds (`true`), or should we also consider
    /// paths where the assumption does not hold, if that is possible (`false`)?
    ///
    /// Note that you may also provide a custom hook for `llvm.assume()` in
    /// [`function_hooks`](struct.Config.html#structfield.function_hooks).
    /// If you do, that overrides this setting.
    ///
    /// Default is `true`.
    pub trust_llvm_assumes: bool,

    /// The set of currently active function hooks; see
    /// [`FunctionHooks`](../function_hooks/struct.FunctionHooks.html) for more details.
    ///
    /// Default is
    /// [`FunctionHooks::default()`](../function_hooks/struct.FunctionHooks.html#method.default);
    /// see docs there for more details.
    pub function_hooks: FunctionHooks<'p, B>,

    /// The set of currently active callbacks; see
    /// [`Callbacks`](../callbacks/struct.Callbacks.html) for more details.
    ///
    /// Default is no callbacks.
    pub callbacks: Callbacks<'p, B>,

    /// The initial memory watchpoints when a `State` is created (mapping from
    /// watchpoint name to the actual watchpoint).
    ///
    /// More watchpoints may be added or removed at any time with
    /// `state.add_mem_watchpoint()` and `state.rm_mem_watchpoint()`.
    ///
    /// Default is no watchpoints.
    pub initial_mem_watchpoints: HashMap<String, Watchpoint>,

    /// Controls the (attempted) demangling of function names in error messages
    /// and backtraces.
    ///
    /// If `None`, `haybale` will attempt to autodetect which mangling is
    /// appropriate, based on the LLVM metadata.
    ///
    /// `Some` can be used to force `haybale` to attempt to demangle with a
    /// particular demangler.
    ///
    /// Any symbol that isn't valid for the chosen demangler will simply be left
    /// unchanged, regardless of this setting.
    ///
    /// Default is `None`.
    pub demangling: Option<Demangling>,

    /// If `true`, then `haybale` will attempt to print source location info
    /// (e.g., filename, line number, column number) along with the LLVM location
    /// info in error messages and backtraces. (This applies to error messages
    /// returned by
    /// [`State.full_error_message_with_context()`](../struct.State.html#method.full_error_message_with_context).)
    ///
    /// For this to work, the LLVM bitcode must contain debuginfo. For example,
    /// C/C++ or Rust sources must be compiled with the `-g` flag to `clang`,
    /// `clang++`, or `rustc`.
    ///
    /// In addition, some LLVM instructions simply don't correspond to a
    /// particular source location; e.g., they may be just setting up the stack
    /// frame for a function.
    ///
    /// For path dumps in the case of error, the value of the environment
    /// variable `HAYBALE_DUMP_PATH` has precedence over this setting.
    /// `HAYBALE_DUMP_PATH` may be set to:
    ///     `LLVM` for a list of the LLVM basic blocks in the path;
    ///     `SRC` for a list of the source-language locations in the path;
    ///     `BOTH` for both of the above.
    ///
    /// Default is `true`.
    pub print_source_info: bool,

    /// If `true`, then `haybale` will include the module name along with the
    /// LLVM location info in error messages, backtraces, log messages, and
    /// when dumping paths. If `false`, the module name will be omitted.
    /// You may want to use `false` for `Project`s with only a single bitcode
    /// file, or if the LLVM module is clear from the function name.
    ///
    /// Default is `true`.
    pub print_module_name: bool,
}

/// Enum used for the `null_pointer_checking` option in `Config`.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum NullPointerChecking {
    /// All memory accesses will be checked to ensure their addresses cannot be
    /// `NULL`. If `NULL` is a possible solution for the address of a memory
    /// access, we will return `Error::NullPointerDereference` and not continue
    /// along the path.
    Simple,

    /// All memory accesses will be checked to ensure their addresses cannot be
    /// `NULL`. If `NULL` is a possible solution for the address of a memory
    /// access, but not the only possible solution, we will split into two paths:
    /// one in which the address is constrained to be `NULL`, and which returns
    /// `Error::NullPointerDereference`; and another in which the address is
    /// constrained to be non-`NULL`, and which will continue execution.
    SplitPath,

    /// Memory accesses will not be checked for `NULL` addresses. This may result
    /// in fewer solver queries and thus improved performance for some workloads.
    None,
}

/// Enum used for the `concretize_memcpy_lengths` option in `Config`.
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
    /// Creates a new `Config` with defaults for all the options, except with
    /// no function hooks.
    ///
    /// You may want to consider
    /// [`Config::default()`](struct.Config.html#method.default), which comes
    /// with predefined hooks for common functions.
    pub fn new() -> Self {
        let mut config = Self::default();
        config.function_hooks = FunctionHooks::new();
        config
    }
}

impl<'p, B: Backend> Default for Config<'p, B> {
    /// Default values for all configuration parameters.
    ///
    /// In particular, this uses
    /// [`FunctionHooks::default()`](../function_hooks/struct.FunctionHooks.html#method.default),
    /// and therefore comes with a set of predefined hooks for common functions.
    ///
    /// For more information, see
    /// [`FunctionHooks::default()`](../function_hooks/struct.FunctionHooks.html#method.default).
    fn default() -> Self {
        Self {
            loop_bound: 10,
            max_callstack_depth: None,
            solver_query_timeout: Some(Duration::from_secs(300)),
            null_pointer_checking: NullPointerChecking::Simple,
            concretize_memcpy_lengths: Concretize::Symbolic,
            max_memcpy_length: None,
            squash_unsats: true,
            trust_llvm_assumes: true,
            function_hooks: FunctionHooks::default(),
            callbacks: Callbacks::default(),
            initial_mem_watchpoints: HashMap::new(),
            demangling: None,
            print_source_info: true,
            print_module_name: true,
        }
    }
}
