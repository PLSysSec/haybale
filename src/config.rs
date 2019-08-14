use crate::backend::Backend;
use crate::default_hooks;
use crate::error::*;
use crate::state::State;
use llvm_ir::instruction;
use std::collections::HashMap;

pub struct Config<'ctx, B> where B: Backend<'ctx> {
    /// Maximum number of times to execute any given line of LLVM IR.
    /// This bounds both the number of iterations of loops, and also the depth of recursion.
    /// For inner loops, this bounds the number of total iterations across all invocations of the loop.
    pub loop_bound: usize,

    /// Map from function names to the hook to use (if any).
    /// You can hook internal functions (which are defined in some available LLVM
    /// `Module`), external functions (e.g., calls to external libraries), LLVM
    /// intrinsics, or any other kind of function.
    ///
    /// Note that the function [`default_hooks()`](fn.default_hooks.html)
    /// provides a set of predefined hooks for common functions.
    /// (At the time of this writing, only `malloc()` and `free()`.)
    /// `Config::default()` uses these predefined hooks and no others.
    /// If you define your own hooks, you may want to consider starting with
    /// `default_hooks()` rather than an empty map.
    ///
    /// The function resolution process is as follows:
    ///
    /// (1) If the function is hooked (its name is in the map), then the hook
    /// will be used instead of any other option. That is, the hook has the
    /// highest precedence.
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
    pub function_hooks: HashMap<String, FunctionHook<'ctx, B>>,
}

/// Function hooks are given mutable access to the `State`, and read-only access
/// to the `Call` they are hooking (which includes arguments etc).
///
/// They should return the [`ReturnValue`](enum.ReturnValue.html) representing
/// the return value of the call, or an appropriate [`Error`](enum.Error.html) if
/// they cannot.
pub struct FunctionHook<'ctx, B>(Box<Fn(&mut State<'ctx, '_, B>, &instruction::Call) -> Result<ReturnValue<B::BV>> + 'ctx>)
    where B: Backend<'ctx>;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ReturnValue<V> {
    /// Return this value
    Return(V),
    /// The hooked call returns void
    ReturnVoid,
}

impl<'ctx, B> FunctionHook<'ctx, B> where B: Backend<'ctx> {
    pub fn new(f: &'ctx Fn(&mut State<'ctx, '_, B>, &instruction::Call) -> Result<ReturnValue<B::BV>>) -> Self {
        Self(Box::new(f))
    }

    pub fn call_hook(&self, state: &mut State<'ctx, '_, B>, call: &instruction::Call) -> Result<ReturnValue<B::BV>> {
        (self.0)(state, call)
    }
}

impl<'ctx, B> Default for Config<'ctx, B> where B: Backend<'ctx> {
    fn default() -> Self {
        Self {
            loop_bound: 10,
            function_hooks: default_hooks::default_hooks(),
        }
    }
}
