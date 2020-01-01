use crate::backend::Backend;
use crate::error::*;
use crate::default_hooks;
use crate::project::Project;
use crate::return_value::*;
use crate::state::State;
use either::Either;
use llvm_ir::{Operand, Typed, instruction::InlineAssembly};
use llvm_ir::function::{CallingConvention, FunctionAttribute, ParameterAttribute};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// A set of function hooks, which will be executed instead of their respective
/// hooked functions if/when the symbolic execution engine encounters a call to
/// one of those hooked functions.
#[derive(Clone)]
pub struct FunctionHooks<'p, B: Backend + 'p> {
    /// Map from function names to the hook to use.
    /// If a function name isn't in this map, the function isn't hooked.
    hooks: HashMap<String, FunctionHook<'p, B>>,

    /// For internal use in creating unique `id`s for `FunctionHook`s
    cur_id: usize,
}

/// An `Argument` represents a single argument to a called function, together
/// with zero or more attributes which apply to it
pub type Argument = (Operand, Vec<ParameterAttribute>);

/// `IsCall` exists to unify the commonalities between LLVM `Call` and `Invoke`
/// instructions
pub trait IsCall : Typed {
    fn get_called_func(&self) -> &Either<InlineAssembly, Operand>;
    fn get_arguments(&self) -> &Vec<Argument>;
    fn get_return_attrs(&self) -> &Vec<ParameterAttribute>;
    fn get_fn_attrs(&self) -> &Vec<FunctionAttribute>;
    fn get_calling_convention(&self) -> CallingConvention;
}

impl IsCall for llvm_ir::instruction::Call {
    fn get_called_func(&self) -> &Either<InlineAssembly, Operand> {
        &self.function
    }
    fn get_arguments(&self) -> &Vec<Argument> {
        &self.arguments
    }
    fn get_return_attrs(&self) -> &Vec<ParameterAttribute> {
        &self.return_attributes
    }
    fn get_fn_attrs(&self) -> &Vec<FunctionAttribute> {
        &self.function_attributes
    }
    fn get_calling_convention(&self) -> CallingConvention {
        self.calling_convention
    }
}

impl IsCall for llvm_ir::terminator::Invoke {
    fn get_called_func(&self) -> &Either<InlineAssembly, Operand> {
        &self.function
    }
    fn get_arguments(&self) -> &Vec<Argument> {
        &self.arguments
    }
    fn get_return_attrs(&self) -> &Vec<ParameterAttribute> {
        &self.return_attributes
    }
    fn get_fn_attrs(&self) -> &Vec<FunctionAttribute> {
        &self.function_attributes
    }
    fn get_calling_convention(&self) -> CallingConvention {
        self.calling_convention
    }
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
        where H: Fn(&'p Project, &mut State<'p, B>, &'p dyn IsCall) -> Result<ReturnValue<B::BV>>
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
    hook: Rc<dyn Fn(&'p Project, &mut State<'p, B>, &'p dyn IsCall) -> Result<ReturnValue<B::BV>> + 'p>,

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
    pub fn new(id: usize, f: &'p dyn Fn(&'p Project, &mut State<'p, B>, &'p dyn IsCall) -> Result<ReturnValue<B::BV>>) -> Self {
        Self { hook: Rc::new(f), id }
    }

    pub fn call_hook(&self, proj: &'p Project, state: &mut State<'p, B>, call: &'p dyn IsCall) -> Result<ReturnValue<B::BV>> {
        (self.hook)(proj, state, call)
    }
}
