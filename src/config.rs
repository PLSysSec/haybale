use crate::backend::Backend;
use crate::default_hooks;
use crate::error::*;
use crate::return_value::*;
use crate::state::State;
use llvm_ir::instruction;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Clone)]
pub struct Config<'p, B> where B: Backend {
    /// Maximum number of times to execute any given line of LLVM IR.
    /// This bounds both the number of iterations of loops, and also the depth of recursion.
    /// For inner loops, this bounds the number of total iterations across all invocations of the loop.
    pub loop_bound: usize,

    /// When encountering a `memcpy`, `memset`, or `memmove` with multiple
    /// possible lengths, how (if at all) should we concretize?
    pub concretize_memcpy_lengths: Concretize,

    /// Active function hooks
    pub function_hooks: FunctionHooks<'p, B>,
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
    /// `concretize_memcpy_lengths()` options, and no function hooks.
    ///
    /// You may want to consider `Config::default()` which provides defaults for
    /// all parameters and comes with predefined hooks for common functions.
    pub fn new(loop_bound: usize, concretize_memcpy_lengths: Concretize) -> Self {
        Self {
            loop_bound,
            concretize_memcpy_lengths,
            function_hooks: FunctionHooks::new(),
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
            concretize_memcpy_lengths: Concretize::Symbolic,
            function_hooks: FunctionHooks::default(),
        }
    }
}

#[derive(Clone)]
pub struct FunctionHooks<'p, B: Backend + 'p> {
    /// Map from function names to the hook to use.
    /// If a function name isn't in this map, the function isn't hooked.
    hooks: HashMap<String, FunctionHook<'p, B>>,

    /// For internal use in creating unique `id`s for `FunctionHook`s
    cur_id: usize,
}

impl<'p, B: Backend + 'p> FunctionHooks<'p, B> {
    /// Create a blank `FunctionHooks` instance with no function hooks.
    ///
    /// You may want to consider `FunctionHooks::default()` which provides
    /// predefined hooks for common functions.
    pub fn new() -> Self {
        Self {
            hooks: HashMap::new(),
            cur_id: 0,
        }
    }

    /// Adds a function hook. The `hook` will be executed instead of the body of
    /// the `hooked_function`.
    ///
    /// You can hook internal functions (which are defined in some available LLVM
    /// `Module`), external functions (e.g., calls to external libraries), LLVM
    /// intrinsics, or any other kind of function.
    ///
    /// The function resolution process is as follows:
    ///
    /// (1) If the function is hooked, then the hook will be used instead of any
    /// other option. That is, the hook has the highest precedence.
    ///
    /// (2) Else, if the function is not hooked but is defined in an available
    /// LLVM `Module`, the function will be symbolically executed (called).
    ///
    /// (3) Haybale provides default hooks for certain LLVM intrinsics like
    /// `memcpy`, which it will apply if the first two options fail.
    ///
    /// (4) If none of the above options apply, an error will be raised.
    /// Note that this means that calls to external functions will always
    /// error unless a hook for them is provided here.
    pub fn add<H>(&mut self, hooked_function: impl Into<String>, hook: &'p H)
        where H: Fn(&mut State<'p, B>, &'p instruction::Call) -> Result<ReturnValue<B::BV>>
    {
        self.hooks.insert(hooked_function.into(), FunctionHook::new(self.cur_id, hook));
        self.cur_id += 1;
    }

    /// Removes the function hook for the given function. That function will no
    /// longer be hooked.
    pub fn remove(&mut self, hooked_function: &str) {
        self.hooks.remove(hooked_function);
    }

    /// Iterate over all function hooks, as (function name, hook) pairs.
    pub(crate) fn get_all_hooks(&self) -> impl Iterator<Item = (&String, &FunctionHook<'p, B>)> {
        self.hooks.iter()
    }

    /// Get the `FunctionHook` active for the given `funcname`, or `None` if
    /// there is no hook active for the function.
    pub(crate) fn get_hook_for(&self, funcname: &str) -> Option<&FunctionHook<'p, B>> {
        self.hooks.get(funcname)
    }

    /// Determine whether there is an active hook for the given `funcname`
    pub fn is_hooked(&self, funcname: &str) -> bool {
        self.get_hook_for(funcname).is_some()
    }
}

impl<'p, B: Backend + 'p> Default for FunctionHooks<'p, B> {
    /// Provides predefined hooks for common functions. (At the time of this
    /// writing, only `malloc()`, `calloc()`, `realloc()`, and `free()`.)
    ///
    /// If you don't want these hooks, you can use
    /// [`FunctionHooks::remove_function_hook()`](struct.FunctionHooks.html#method.remove_function_hook)
    /// to remove individual hooks, or you can use `FunctionHooks::new()`, which
    /// comes with no predefined hooks.
    fn default() -> Self {
        let mut fhooks = Self::new();
        fhooks.add("malloc", &default_hooks::malloc_hook);
        fhooks.add("calloc", &default_hooks::calloc_hook);
        fhooks.add("realloc", &default_hooks::realloc_hook);
        fhooks.add("free", &default_hooks::free_hook);
        fhooks
    }
}

/// Function hooks are given mutable access to the `State`, and read-only access
/// to the `Call` they are hooking (which includes arguments etc).
///
/// They should return the [`ReturnValue`](enum.ReturnValue.html) representing
/// the return value of the call, or an appropriate [`Error`](enum.Error.html) if
/// they cannot.
pub(crate) struct FunctionHook<'p, B: Backend> {
    /// The actual hook to be executed
    hook: Rc<dyn Fn(&mut State<'p, B>, &'p instruction::Call) -> Result<ReturnValue<B::BV>> + 'p>,

    /// A unique id, used for nothing except equality comparisons between `FunctionHook`s.
    /// This `id` should be globally unique across all created `FunctionHook`s.
    id: usize,
}

impl<'p, B: Backend> Clone for FunctionHook<'p, B> {
    fn clone(&self) -> Self {
        Self { hook: self.hook.clone(), id: self.id }
    }
}

impl<'p, B: Backend> PartialEq for FunctionHook<'p, B> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'p, B: Backend> Eq for FunctionHook<'p, B> {}

impl<'p, B: Backend> Hash for FunctionHook<'p, B> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<'p, B: Backend> FunctionHook<'p, B> {
    /// `id`: A unique id, used for nothing except equality comparisons between `FunctionHook`s.
    /// This `id` should be globally unique across all created `FunctionHook`s.
    pub fn new(id: usize, f: &'p dyn Fn(&mut State<'p, B>, &'p instruction::Call) -> Result<ReturnValue<B::BV>>) -> Self {
        Self { hook: Rc::new(f), id }
    }

    pub fn call_hook(&self, state: &mut State<'p, B>, call: &'p instruction::Call) -> Result<ReturnValue<B::BV>> {
        (self.hook)(state, call)
    }
}
