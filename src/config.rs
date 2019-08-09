use crate::backend::Backend;
use crate::default_hooks;
use crate::state::State;
use llvm_ir::instruction;
use std::collections::HashMap;

pub struct Config<'ctx, B> where B: Backend<'ctx> {
    /// Maximum number of times to execute any given line of LLVM IR.
    /// This bounds both the number of iterations of loops, and also the depth of recursion.
    /// For inner loops, this bounds the number of total iterations across all invocations of the loop.
    pub loop_bound: usize,

    /// Map from function names to the hook to use (if any).
    /// You can hook internal functions (which are defined in the same
    /// module as the caller), external functions (e.g., calls to external libraries),
    /// LLVM intrinsics, or any other kind of function.
    ///
    /// The function resolution process is as follows:
    ///
    /// (1) If the function is hooked (its name is in the map), then the hook
    /// will be used instead of any other option. That is, the hook has the
    /// highest precedence.
    ///
    /// (2) Else, if the function is not hooked but is defined in the same LLVM
    /// module as the caller, the function will be symbolically executed
    /// (called).
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
pub struct FunctionHook<'ctx, B>(Box<Fn(&mut State<'ctx, '_, B>, &instruction::Call) -> HookResult + 'ctx>)
    where B: Backend<'ctx>;

/// Function hooks should return `Ok(())` if processing succeeded.
///
/// If they return `Err`, this will be treated as an indication that the current
/// path should be killed (and not some other type of error).
pub type HookResult = Result<(), &'static str>;

impl<'ctx, B> FunctionHook<'ctx, B> where B: Backend<'ctx> {
    pub fn new(f: &'ctx Fn(&mut State<'ctx, '_, B>, &instruction::Call) -> HookResult) -> Self {
        Self(Box::new(f))
    }

    pub fn call_hook(&self, state: &mut State<'ctx, '_, B>, call: &instruction::Call) -> HookResult {
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
