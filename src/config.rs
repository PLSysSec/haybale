use crate::backend::Backend;
use crate::state::State;
use llvm_ir::instruction;
use std::collections::HashMap;

pub struct Config<B> {
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
    pub function_hooks: HashMap<String, FunctionHook<B>>,
}

pub struct FunctionHook<B>(Box<Fn(&mut State<B>, &instruction::Call)>);

impl<'ctx, 'm, B> FunctionHook<B> where B: Backend<'ctx> {
    pub fn call_hook(&self, state: &mut State<'ctx, 'm, B>, call: &'m instruction::Call) {
        (self.0)(state, call)
    }
}

impl<B> Default for Config<B> {
    fn default() -> Self {
        Self {
            loop_bound: 10,
            function_hooks: HashMap::new(),
        }
    }
}
