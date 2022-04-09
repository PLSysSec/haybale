//! Functions and structures for defining and activating function hooks

use crate::backend::Backend;
use crate::demangling;
use crate::error::*;
use crate::hooks;
use crate::return_value::*;
use crate::state::State;
use either::Either;
use llvm_ir::function::{CallingConvention, FunctionAttribute, ParameterAttribute};
use llvm_ir::types::Typed;
use llvm_ir::{instruction::InlineAssembly, Name, Operand, Type};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// A set of function hooks, which will be executed instead of their respective
/// hooked functions if/when the symbolic execution engine encounters a call to
/// one of those hooked functions.
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
/// (2) Haybale provides default hooks for certain LLVM intrinsics like
/// `memcpy`, which have specially reserved names; it will apply these hooks
/// unless a different hook was defined for the intrinsic in (1).
///
/// (3) Else, if the function is not hooked but is defined in an available
/// LLVM `Module`, the function will be symbolically executed (called).
///
/// (4) Else, if a default function hook was supplied with `add_default_hook()`,
/// that hook will be used.
///
/// (5) If none of the above options apply, an error will be raised.
/// Note that this means that calls to external functions will always
/// error unless a hook for them is provided, either by name or via the default
/// hook.
#[derive(Clone)]
pub struct FunctionHooks<'p, B: Backend + 'p> {
    /// `hooks`, `cpp_demangled_hooks`, and `rust_demangled_hooks` are each maps
    /// from function names to the hook to use. In `hooks`, the function names
    /// are exactly as they appear in the LLVM IR. In `cpp_demangled_hooks` and
    /// `rust_demangled_hooks`, the function names are demangled versions (using
    /// the C++ and Rust demanglers respectively) of the names that appear in the
    /// LLVM IR.
    ///
    /// It's intended that a function should only be hooked in one of these maps;
    /// but if both the mangled and demangled function names are hooked, the hook
    /// in `hooks` (that is, for the mangled name) takes priority.
    ///
    /// If a function name isn't in any of these maps, the function isn't hooked.
    hooks: HashMap<String, FunctionHook<'p, B>>,
    cpp_demangled_hooks: HashMap<String, FunctionHook<'p, B>>,
    rust_demangled_hooks: HashMap<String, FunctionHook<'p, B>>,

    /// Hook (if any) to use for calls to inline assembly.
    /// This one hook will handle all calls to any inline assembly, regardless of
    /// the contents; it is responsible for inspecting the contents and acting
    /// appropriately.
    ///
    /// Note that as of this writing, due to a limitation of the LLVM C API (see
    /// the 'Limitations' section of the
    /// [`llvm-ir` README](https://github.com/cdisselkoen/llvm-ir/blob/master/README.md)),
    /// the hook will actually have no way of obtaining the contents of the asm
    /// string itself, although it can still inspect function parameters etc.
    /// For now, this is the best we can do.
    ///
    /// If no hook is provided here, then all calls to inline assembly will
    /// result in errors.
    inline_asm_hook: Option<FunctionHook<'p, B>>,

    /// Hook (if any) to use for functions which are neither defined in the LLVM
    /// IR nor specifically hooked by name.
    ///
    /// If no hook is provided here, then calls to functions which are neither
    /// defined nor hooked will result in `Error::FunctionNotFound` errors.
    default_hook: Option<FunctionHook<'p, B>>,

    /// For internal use in creating unique `id`s for `FunctionHook`s
    cur_id: usize,
}

/// An `Argument` represents a single argument to a called function, together
/// with zero or more attributes which apply to it
pub type Argument = (Operand, Vec<ParameterAttribute>);

/// `IsCall` exists to unify the commonalities between LLVM `Call` and `Invoke`
/// instructions
pub trait IsCall: Typed {
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
    /// You may want to consider
    /// [`FunctionHooks::default()`](struct.FunctionHooks.html#method.default),
    /// which provides predefined hooks for common functions.
    pub fn new() -> Self {
        Self {
            hooks: HashMap::new(),
            cpp_demangled_hooks: HashMap::new(),
            rust_demangled_hooks: HashMap::new(),
            inline_asm_hook: None,
            default_hook: None,
            cur_id: 0,
        }
    }

    /// Adds a function hook. The `hook` will be executed instead of the body of
    /// the `hooked_function`.
    pub fn add<H>(&mut self, hooked_function: impl Into<String>, hook: &'p H)
    where
        H: Fn(&mut State<'p, B>, &'p dyn IsCall) -> Result<ReturnValue<B::BV>>,
    {
        self.hooks
            .insert(hooked_function.into(), FunctionHook::new(self.cur_id, hook));
        self.cur_id += 1;
    }

    /// Exactly like `add()`, but takes the (C++) _demangled_ name of the function
    /// to hook, so you can use a function name like "namespace::function".
    pub fn add_cpp_demangled<H>(&mut self, hooked_function: impl Into<String>, hook: &'p H)
    where
        H: Fn(&mut State<'p, B>, &'p dyn IsCall) -> Result<ReturnValue<B::BV>>,
    {
        self.cpp_demangled_hooks
            .insert(hooked_function.into(), FunctionHook::new(self.cur_id, hook));
        self.cur_id += 1;
    }

    /// Exactly like `add()`, but takes the (Rust) _demangled_ name of the function
    /// to hook, so you can use a function name like "module::function".
    pub fn add_rust_demangled<H>(&mut self, hooked_function: impl Into<String>, hook: &'p H)
    where
        H: Fn(&mut State<'p, B>, &'p dyn IsCall) -> Result<ReturnValue<B::BV>>,
    {
        self.rust_demangled_hooks
            .insert(hooked_function.into(), FunctionHook::new(self.cur_id, hook));
        self.cur_id += 1;
    }

    /// Add a hook to be used for calls to inline assembly.
    /// This one hook will handle all calls to any inline assembly, regardless of
    /// the contents; it is responsible for inspecting the contents and acting
    /// appropriately.
    /// If another inline assembly hook is added, it will replace any inline
    /// assembly hook which was previously present.
    ///
    /// Returns `true` if an inline assembly hook was previously present, or
    /// `false` if no inline assembly hook was present.
    ///
    /// Note that as of this writing, due to a limitation of the LLVM C API (see
    /// the 'Limitations' section of the
    /// [`llvm-ir` README](https://github.com/cdisselkoen/llvm-ir/blob/master/README.md)),
    /// the hook will actually have no way of obtaining the contents of the asm
    /// string itself, although it can still inspect function parameters etc.
    /// For now, this is the best we can do.
    pub fn add_inline_asm_hook<H>(&mut self, hook: &'p H) -> bool
    where
        H: Fn(&mut State<'p, B>, &'p dyn IsCall) -> Result<ReturnValue<B::BV>>,
    {
        match &mut self.inline_asm_hook {
            h @ Some(_) => {
                *h = Some(FunctionHook::new(self.cur_id, hook));
                self.cur_id += 1;
                true
            },
            h @ None => {
                *h = Some(FunctionHook::new(self.cur_id, hook));
                self.cur_id += 1;
                false
            },
        }
    }

    /// Add a hook to be used if no other definition or hook is found for the
    /// call.
    /// If another default hook is added, it will replace any default hook which
    /// was previously present.
    ///
    /// Returns `true` if a default hook was previously present, or `false` if no
    /// default hook was present.
    pub fn add_default_hook<H>(&mut self, hook: &'p H) -> bool
    where
        H: Fn(&mut State<'p, B>, &'p dyn IsCall) -> Result<ReturnValue<B::BV>>,
    {
        match &mut self.default_hook {
            h @ Some(_) => {
                *h = Some(FunctionHook::new(self.cur_id, hook));
                self.cur_id += 1;
                true
            },
            h @ None => {
                *h = Some(FunctionHook::new(self.cur_id, hook));
                self.cur_id += 1;
                false
            },
        }
    }

    /// Removes the function hook for the given function, which was added with
    /// `add()`. That function will no longer be hooked.
    pub fn remove(&mut self, hooked_function: &str) {
        self.hooks.remove(hooked_function);
    }

    /// Removes the function hook for the given function, which was added with
    /// [`add_cpp_demangled()`](struct.FunctionHooks.html#method.add_cpp_demangled).
    /// That function will no longer be hooked.
    pub fn remove_cpp_demangled(&mut self, hooked_function: &str) {
        self.cpp_demangled_hooks.remove(hooked_function);
    }

    /// Removes the function hook for the given function, which was added with
    /// [`add_rust_demangled()`](struct.FunctionHooks.html#method.add_rust_demangled).
    /// That function will no longer be hooked.
    pub fn remove_rust_demangled(&mut self, hooked_function: &str) {
        self.rust_demangled_hooks.remove(hooked_function);
    }

    /// Removes the function hook used for calls to inline assembly, which was
    /// added with [`add_inline_asm_hook()`]. Calls to inline assembly will no
    /// longer be hooked, and thus will result in errors, until the next call to
    /// [`add_inline_asm_hook()`].
    ///
    /// [`add_inline_asm_hook()`]: struct.FunctionHooks.html#method.add_inline_asm_hook
    pub fn remove_inline_asm_hook(&mut self) {
        self.inline_asm_hook = None;
    }

    /// Removes the default function hook which was added with
    /// [`add_default_hook()`]. Calls to functions which are neither defined in
    /// the `Project` nor specifically hooked will thus result in
    /// `Error::FunctionNotFound` errors, until the next call to
    /// [`add_default_hook()`].
    ///
    /// [`add_default_hook()`]: struct.FunctionHooks.html#method.add_default_hook
    pub fn remove_default_hook(&mut self) {
        self.default_hook = None;
    }

    /// Iterate over all function hooks, as (function name, hook) pairs.
    /// Function names may include both mangled and demangled names.
    pub(crate) fn get_all_hooks(&self) -> impl Iterator<Item = (&String, &FunctionHook<'p, B>)> {
        self.hooks
            .iter()
            .chain(self.cpp_demangled_hooks.iter())
            .chain(self.rust_demangled_hooks.iter())
    }

    /// Get the `FunctionHook` active for the given `funcname`, or `None` if
    /// there is no hook active for the function. `funcname` may be either a
    /// mangled or a demangled function name.
    pub(crate) fn get_hook_for(&self, funcname: &str) -> Option<&FunctionHook<'p, B>> {
        self.hooks
            .get(funcname)
            .or_else(|| {
                demangling::try_rust_demangle(funcname)
                    .and_then(|demangled| self.rust_demangled_hooks.get(&demangled))
            })
            .or_else(|| {
                demangling::try_cpp_demangle(funcname)
                    .and_then(|demangled| self.cpp_demangled_hooks.get(&demangled))
            })
    }

    /// Get the `FunctionHook` used for calls to inline assembly, if there is one.
    ///
    /// See docs on `add_inline_asm_hook()` above
    pub(crate) fn get_inline_asm_hook(&self) -> Option<&FunctionHook<'p, B>> {
        self.inline_asm_hook.as_ref()
    }

    /// Get the default `FunctionHook` (used when no LLVM definition or hook is
    /// found), if there is one.
    ///
    /// See docs on `add_default_hook()` above
    pub(crate) fn get_default_hook(&self) -> Option<&FunctionHook<'p, B>> {
        self.default_hook.as_ref()
    }

    /// Determine whether there is an active hook for the given `funcname`
    pub fn is_hooked(&self, funcname: &str) -> bool {
        self.get_hook_for(funcname).is_some()
    }

    /// Is there currently an inline asm hook active?
    /// (See `add_inline_asm_hook()` for more info)
    pub fn has_inline_asm_hook(&self) -> bool {
        self.inline_asm_hook.is_some()
    }

    /// Is there currently a default hook active?
    /// (See `add_default_hook()` for more info)
    pub fn has_default_hook(&self) -> bool {
        self.default_hook.is_some()
    }
}

impl<'p, B: Backend + 'p> Default for FunctionHooks<'p, B> {
    /// Provides predefined hooks for common functions. (At the time of this
    /// writing, this includes malloc-related functions `malloc()`, `calloc()`,
    /// `realloc()`, and `free()`, as well as some C++ exception-handling
    /// functions such as `__cxa_throw()` and `__cxa_allocate_exception()`,
    /// and a few other C and Rust standard library functions.)
    ///
    /// If you don't want these hooks, you can use
    /// [`FunctionHooks::remove_function_hook()`](struct.FunctionHooks.html#method.remove_function_hook)
    /// to remove individual hooks, or you can use
    /// [`FunctionHooks::new()`](struct.FunctionHooks.html#method.new), which
    /// comes with no predefined hooks.
    fn default() -> Self {
        let mut fhooks = Self::new();
        fhooks.add("malloc", &hooks::allocation::malloc_hook);
        fhooks.add("calloc", &hooks::allocation::calloc_hook);
        fhooks.add("realloc", &hooks::allocation::realloc_hook);
        fhooks.add("free", &hooks::allocation::free_hook);
        fhooks.add(
            "__cxa_allocate_exception",
            &hooks::exceptions::cxa_allocate_exception,
        );
        fhooks.add("__cxa_throw", &hooks::exceptions::cxa_throw);
        fhooks.add("__cxa_begin_catch", &hooks::exceptions::cxa_begin_catch);
        fhooks.add("__cxa_end_catch", &hooks::exceptions::cxa_end_catch);
        fhooks.add("llvm.eh.typeid.for", &hooks::exceptions::llvm_eh_typeid_for);
        fhooks.add("exit", &abort_hook);
        fhooks.add_rust_demangled("std::panicking::begin_panic", &abort_hook);
        fhooks.add_rust_demangled("std::panicking::begin_panic_fmt", &abort_hook);
        fhooks.add_rust_demangled("std::panicking::begin_panic_handler", &abort_hook);
        fhooks.add_rust_demangled("core::panicking::panic", &abort_hook);
        fhooks.add_rust_demangled("core::panicking::panic_bounds_check", &abort_hook);
        fhooks.add_rust_demangled("core::result::unwrap_failed", &abort_hook);
        fhooks.add_rust_demangled("core::slice::slice_index_len_fail", &abort_hook);
        fhooks.add_rust_demangled("core::slice::slice_index_order_fail", &abort_hook);
        fhooks.add_rust_demangled("core::slice::slice_index_overflow_fail", &abort_hook);
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
    #[allow(clippy::type_complexity)]
    hook: Rc<dyn Fn(&mut State<'p, B>, &'p dyn IsCall) -> Result<ReturnValue<B::BV>> + 'p>,

    /// A unique id, used for nothing except equality comparisons between `FunctionHook`s.
    /// This `id` should be globally unique across all created `FunctionHook`s.
    id: usize,
}

impl<'p, B: Backend> Clone for FunctionHook<'p, B> {
    fn clone(&self) -> Self {
        Self {
            hook: self.hook.clone(),
            id: self.id,
        }
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
    pub fn new(
        id: usize,
        f: &'p dyn Fn(&mut State<'p, B>, &'p dyn IsCall) -> Result<ReturnValue<B::BV>>,
    ) -> Self {
        Self {
            hook: Rc::new(f),
            id,
        }
    }

    pub fn call_hook(
        &self,
        state: &mut State<'p, B>,
        call: &'p dyn IsCall,
    ) -> Result<ReturnValue<B::BV>> {
        (self.hook)(state, call)
    }
}

/// This hook ignores the function arguments and returns an unconstrained value
/// of the appropriate size for the function's return value (or void for
/// void-typed functions).
///
/// May be used for functions taking any number and type of arguments, and with
/// any return type.
pub fn generic_stub_hook<B: Backend>(
    state: &mut State<B>,
    call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    match state.type_of(call).as_ref() {
        Type::VoidType => Ok(ReturnValue::ReturnVoid),
        ty => {
            let width = state.size_in_bits(ty).ok_or_else(|| {
                Error::OtherError("Call return type is an opaque named struct".into())
            })?;
            assert_ne!(width, 0, "Call return type has size 0 bits but isn't void type"); // void type was handled above
            let bv = state.new_bv_with_name(Name::from("generic_stub_hook_retval"), width)?;
            Ok(ReturnValue::Return(bv))
        },
    }
}

/// This hook ignores the function arguments and returns `ReturnValue::Abort`.
/// It is suitable for hooking functions such as C's `exit()` which abort the
/// program and never return.
pub fn abort_hook<B: Backend>(
    _state: &mut State<B>,
    _call: &dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    Ok(ReturnValue::Abort(99))//d.source_loc
}
