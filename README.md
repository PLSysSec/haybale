# `haybale`: Symbolic execution of LLVM IR, written in Rust

[![Crates.io](http://meritbadge.herokuapp.com/haybale)](https://crates.io/crates/haybale)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/cdisselkoen/haybale/master/LICENSE)

`haybale` is a general-purpose symbolic execution engine written in Rust.
It operates on LLVM IR, which allows it to analyze programs written in C/C++,
Rust, Swift, or any other language which compiles to LLVM IR.
In this way, it may be compared to [KLEE], as it has similar goals, except
that `haybale` is written in Rust and makes some different design decisions.
That said, `haybale` makes no claim of being at feature parity with KLEE.

### Okay, but what is a symbolic execution engine?

A symbolic execution engine is a way of reasoning - rigorously and
mathematically - about the behavior of a function or program.
It can reason about _all possible inputs_ to a function without literally
brute-forcing every single one.
For instance, a symbolic execution engine like `haybale` can answer questions
like:

- Are there any inputs to (some function) that cause it to return 0? What are they?
- Is it possible for this loop to execute exactly 17 times?
- Can this pointer ever be NULL?

Symbolic execution engines answer these questions by converting each variable in
the program or function into a mathematical expression which depends on the
function or program inputs.
Then they use an SMT solver to answer questions about these expressions, such
as the questions listed above.

## Getting started

### 1. Install

`haybale` is on [crates.io](https://crates.io/crates/haybale), so you can simply
add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
haybale = "0.2.1"
```

`haybale` also depends (indirectly) on the LLVM 9 and Boolector libraries, which
must both be available on your system.
See the [`llvm-sys`] or [`boolector-sys`] READMEs for more details and instructions.

### 2. Acquire bitcode to analyze

Since `haybale` operates on LLVM bitcode, you'll need some bitcode to get started.
If the program or function you want to analyze is written in C, you can generate
LLVM bitcode (`*.bc` files) with `clang`'s `-c` and `-emit-llvm` flags:

```bash
clang -c -emit-llvm source.c -o source.bc
```

For debugging purposes, you may also want LLVM text-format (`*.ll`) files, which
you can generate with `clang`'s `-S` and `-emit-llvm` flags:

```bash
clang -S -emit-llvm source.c -o source.ll
```

If the program or function you want to analyze is written in Rust, you can likewise
use `rustc`'s `--emit=llvm-bc` and `--emit=llvm-ir` flags.

### 3. Create a Project

A `haybale` [`Project`] contains all of the code currently being analyzed, which
may be one or more LLVM modules.
To get started, simply create a `Project` from a single bitcode file:

```rust
let project = Project::from_bc_path(&Path::new("/path/to/file.bc"))?;
```

For more ways to create `Project`s, including analyzing entire libraries, see
the [`Project` documentation].

### 4. Use built-in analyses

`haybale` currently includes two simple built-in analyses:
[`get_possible_return_values_of_func()`], which describes all the possible
values a function could return for any input, and [`find_zero_of_func()`],
which finds a set of inputs to a function such that it returns `0`.
These analyses are provided both because they may be of some use themselves,
but also because they illustrate how to use `haybale`.

For an introductory example, let's suppose `foo` is the following C function:

```c
int foo(int a, int b) {
    if (a > b) {
        return (a-1) * (b-1);
    } else {
        return (a + b) % 3 + 10;
    }
}
```

We can use `find_zero_of_func()` to find inputs such that `foo` will return `0`:

```rust
match find_zero_of_func("foo", &project, Config::default()) {
    None => println!("foo can never return 0"),
    Some(inputs) => println!("Inputs for which foo returns 0: {:?}", inputs),
}
```

## Writing custom analyses

`haybale` can do much more than just describe possible function return values
and find function zeroes.
In this section, we'll walk through how we could find a zero of the function
`foo` above without using the built-in `find_zero_of_func()`.
This will illustrate how to write a custom analysis using `haybale`.

### ExecutionManager

All analyses will use an [`ExecutionManager`] to control the progress of the
symbolic execution.
In the code snippet below, we call [`symex_function()`] to create an
`ExecutionManager` which will analyze the function `foo` - it will start at
the top of the function, and end when the function returns. In between, it
will also analyze any functions called by `foo`, as necessary and depending
on the [`Config`] settings.

```rust
let mut em = symex_function("foo", &project, Config::<BtorBackend>::default());
```

Here it was necessary to not only specify the default `haybale`
configuration, as we did when calling `find_zero_of_func()`, but also what
"backend" we want to use.
The default `BtorBackend` should be fine for most purposes.

### Paths

The `ExecutionManager` acts like an `Iterator` over _paths_ through the function `foo`.
Each path is one possible sequence of control-flow decisions (e.g., which direction
do we take at each `if` statement) leading to the function returning some value.
The function `foo` in this example has two paths, one following the "true" branch and
one following the "false" branch of the `if`.

Let's examine the first path through the function:

```rust
let retval = em.next().expect("Expected at least one path")?;
```

We're given the function return value, `retval`, as a Boolector [`BV`] (bitvector)
wrapped in the [`ReturnValue`] enum.
Since we know that `foo` isn't a void-typed function (and won't throw an
exception or abort), we can simply unwrap the `ReturnValue` to get the `BV`:

```rust
let retval = match retval {
    ReturnValue::Return(r) => r,
    ReturnValue::ReturnVoid => panic!("Function shouldn't return void"),
    ReturnValue::Throw(_) => panic!("Function shouldn't throw an exception"),
    ReturnValue::Abort => panic!("Function shouldn't panic or exit()"),
};
```

### States

Importantly, the `ExecutionManager` provides not only the final return value of
the path as a `BV`, but also the final program [`State`] at the end of that path,
either immutably with `state()` or mutably with `mut_state()`. (See the
[`ExecutionManager` documentation] for more.)

```rust
let state = em.mut_state();  // the final program state along this path
```

To test whether `retval` can be equal to `0` in this `State`, we can use
`state.bvs_can_be_equal()`:

```rust
let zero = state.zero(32);  // The 32-bit constant 0
if state.bvs_can_be_equal(&retval, &zero)? {
    println!("retval can be 0!");
}
```

### Getting solutions for variables

If `retval` can be `0`, let's find what values of the function parameters
would cause that.
First, we'll add a constraint to the `State` requiring that the return value
must be `0`:

```rust
retval._eq(&zero).assert();
```

and then we'll ask for solutions for each of the parameters, given this constraint:

```rust
// Get a possible solution for the first parameter.
// In this case, from looking at the text-format LLVM IR, we know the variable
// we're looking for is variable #0 in the function "foo".
let a = state.get_a_solution_for_irname(&String::from("foo"), Name::from(0))?
    .expect("Expected there to be a solution")
    .as_u64()
    .expect("Expected solution to fit in 64 bits");

// Likewise the second parameter, which is variable #1 in "foo"
let b = state.get_a_solution_for_irname(&String::from("foo"), Name::from(1))?
    .expect("Expected there to be a solution")
    .as_u64()
    .expect("Expected solution to fit in 64 bits");

println!("Parameter values for which foo returns 0: a = {}, b = {}", a, b);
```

Alternately, we could also have gotten the parameter `BV`s from the `ExecutionManager`
like this:

```rust
let a_bv = em.param_bvs()[0].clone();
let b_bv = em.param_bvs()[1].clone();

let a = em.state().get_a_solution_for_bv(&a_bv)?
    .expect("Expected there to be a solution")
    .as_u64()
    .expect("Expected solution to fit in 64 bits");

let b = em.state().get_a_solution_for_bv(&b_bv)?
    .expect("Expected there to be a solution")
    .as_u64()
    .expect("Expected solution to fit in 64 bits");

println!("Parameter values for which foo returns 0: a = {}, b = {}", a, b);
```

## Documentation

Full documentation for `haybale` can be found [here](https://PLSysSec.github.io/haybale),
or of course you can generate local documentation with `cargo doc --open`.

## Compatibility

Currently, `haybale` only supports LLVM 9. A version of `haybale` supporting
LLVM 8 is available on the `llvm-8` branch of this repo, and is approximately
at feature parity with `haybale` version 0.2.0. However, there is no promise
that future `haybale` features will be backported to the `llvm-8` branch.

`haybale` works on stable Rust, and requires Rust 1.36+.

## Under the hood

`haybale` is built using the Rust [`llvm-ir`] crate and the [Boolector] SMT
solver (via the Rust [`boolector`] crate).

## Changelog

### Version 0.2.1 (Jan 15, 2020)

- New `HAYBALE_DUMP_PATH` and `HAYBALE_DUMP_VARS` environment-variable options
  - `HAYBALE_DUMP_PATH`: if set to `1`, then on error, `haybale` will print a
  description of the path to the error: every LLVM basic block touched from
  the top of the function until the error location, in order.
  - `HAYBALE_DUMP_VARS`: if set to `1`, then on error, `haybale` will print the
  latest value assigned to each variable in the function containing the error.
- New setting `Config.demangling` allows you to apply C++ or Rust demangling
to function names in error messages and backtraces
- Support hooking calls to inline assembly, with some limitations inherited
from [`llvm-ir`] (see comments on [`FunctionHooks::add_inline_asm_hook()`])
- Built-in support for (the most common cases of) the `llvm.bswap` intrinsic
- Other tiny tweaks - e.g., downgrade one panic to a warning

### Version 0.2.0 (Jan 8, 2020)

- Support LLVM `extractvalue` and `insertvalue` instructions
- Support LLVM `invoke`, `resume`, and `landingpad` instructions, and thus
C++ `throw`/`catch`. Also provide built-in hooks for some related C++ ABI
functions such as `__cxa_throw()`. This support isn't perfect, particularly
surrounding the matching of catch blocks to exceptions: `haybale` may explore
some additional paths which aren't actually valid. But all actually valid
paths should be found and explored correctly.
- Since functions can be called not only with the LLVM `call` instruction but
also with the LLVM `invoke` instruction, function hooks now receive a
`&dyn IsCall` object which may represent either a `call` or `invoke` instruction.
- `haybale` now uses LLVM 9 rather than LLVM 8. See the "Compatibility"
section above.
- Improvements for `Project`s containing C++ and/or Rust code:
  - For the function-name arguments to [`symex_function()`],
    [`get_possible_return_values_of_func()`], [`find_zero_of_func()`], and
    [`Project::get_func_by_name()`], you may now pass either the (mangled)
    function name as it appears in LLVM (as was supported previously), or the
    demangled function name. That is, you can pass in `"foo::bar"` rather than
    `"_ZN3foo3barE"`.
  - Likewise, you may add function hooks based on the demangled name of
    the hooked function. See [`FunctionHooks::add_cpp_demangled()`] and
    [`FunctionHooks::add_rust_demangled()`].
  - Also, `llvm-ir` versions 0.3.3 and later contain an important bugfix for
    parsing LLVM bitcode generated by `rustc`. `haybale` 0.2.0 uses `llvm-ir`
    0.4.1.
- The [`ReturnValue`] enum now has additional options `Throw`, indicating an
uncaught exception, and `Abort`, indicating a program abort (e.g. Rust
panic, or call to C `exit()`).
- Relatedly, `haybale` now has built-in hooks for the C `exit()` function and
for Rust panics (and for a few more LLVM intrinsics).
- `haybale` also now contains a built-in [`generic_stub_hook`] and
[`abort_hook`] which you can supply as hooks for any functions which you want
to ignore the implementation of, or which always abort, respectively. See
docs on the [`function_hooks`] module.
- [`Config.initial_mem_watchpoints`] is now a `HashMap` instead of a `HashSet`
of pairs.

### Version 0.1.3 (Jan 1, 2020)

- Memory watchpoints: specify a range of memory addresses, and get
a log message for any memory operation which reads or writes any data in
that range. See [`State::add_mem_watchpoint()`].
- Convenience methods on [`State`] for constructing constant-valued `BV`s
(rather than having to use the corresponding methods on `BV` and pass
`state.solver`): `bv_from_i32()`, `bv_from_u32()`, `bv_from_i64()`,
`bv_from_u64()`, `bv_from_bool()`, `zero()`, `one()`, and `ones()`.
- Some internal code refactoring to prepare for 0.2.0 features

### Version 0.1.2 (Dec 18, 2019)

- New method [`Project::get_inner_struct_type_from_named()`] which handles
opaque struct types by searching the entire `Project` for a definition of
the given struct
- Support memory reads of size 1-7 bits (in particular, reads of LLVM `i1`)
- Performance optimization: during `State` initialization, global variables
are now only allocated, and not initialized until first use (lazy
initialization). This gives the SMT solver fewer memory writes to think
about, and helps especially for large `Project`s which may contain many
global variables that won't actually be used in a given analysis.
- Minor bugfixes and improved error messages

### Version 0.1.1 (Nov 26, 2019)

Changes to README text only; no functional changes.

### Version 0.1.0 (Nov 25, 2019)

Initial release!

[`llvm-ir`]: https://crates.io/crates/llvm-ir
[Boolector]: https://boolector.github.io/
[`boolector`]: https://crates.io/crates/boolector
[`llvm-sys`]: https://crates.io/crates/llvm-sys
[`boolector-sys`]: https://crates.io/crates/boolector-sys/
[KLEE]: https://klee.github.io/
[`Project`]: https://PLSysSec.github.io/haybale/haybale/project/struct.Project.html
[`Project` documentation]: https://PLSysSec.github.io/haybale/haybale/project/struct.Project.html
[`Project::get_func_by_name()`]: https://PLSysSec.github.io/haybale/haybale/project/struct.Project.html#method.get_func_by_name
[`get_possible_return_values_of_func()`]: https://PLSysSec.github.io/haybale/haybale/fn.get_possible_return_values_of_func.html
[`find_zero_of_func()`]: https://PLSysSec.github.io/haybale/haybale/fn.find_zero_of_func.html
[`ExecutionManager`]: https://PLSysSec.github.io/haybale/haybale/struct.ExecutionManager.html
[`ExecutionManager` documentation]: https://PLSysSec.github.io/haybale/haybale/struct.ExecutionManager.html
[`symex_function()`]: https://PLSysSec.github.io/haybale/haybale/fn.symex_function.html
[`Config`]: https://PLSysSec.github.io/haybale/haybale/struct.Config.html
[`BV`]: https://docs.rs/boolector/0.1.2/boolector/struct.BV.html
[`ReturnValue`]: https://PLSysSec.github.io/haybale/haybale/enum.ReturnValue.html
[`State`]: https://PLSysSec.github.io/haybale/haybale/struct.State.html
[`Project::get_inner_struct_type_from_named()`]: https://PLSysSec.github.io/haybale/haybale/struct.Project.html#method.get_inner_struct_type_from_named
[`State::add_mem_watchpoint()`]: https://PLSysSec.github.io/haybale/haybale/struct.State.html#method.add_mem_watchpoint
[`FunctionHooks::add_cpp_demangled()`]: https://PLSysSec.github.io/haybale/haybale/function_hooks/struct.FunctionHooks.html#method.add_cpp_demangled
[`FunctionHooks::add_rust_demangled()`]: https://PLSysSec.github.io/haybale/haybale/function_hooks/struct.FunctionHooks.html#method.add_rust_demangled
[`FunctionHooks::add_inline_asm_hook()`]: https://PLSysSec.github.io/haybale/haybale/function_hooks/struct.FunctionHooks.html#method.add_inline_asm_hook
[`function_hooks`]: https://PLSysSec.github.io/haybale/haybale/function_hooks/index.html
[`generic_stub_hook`]: https://PLSysSec.github.io/haybale/haybale/function_hooks/fn.generic_stub_hook.html
[`abort_hook`]: https://PLSysSec.github.io/haybale/haybale/function_hooks/fn.abort_hook.html
[`Config.initial_mem_watchpoints`]: https://PLSysSec.github.io/haybale/haybale/struct.Config.html#structfield.initial_mem_watchpoints
