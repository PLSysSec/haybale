# `haybale`: Symbolic execution of LLVM IR, written in Rust

[![crates.io](https://img.shields.io/crates/v/haybale.svg)](https://crates.io/crates/haybale)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/cdisselkoen/haybale/main/LICENSE)

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
add it as a dependency in your `Cargo.toml`, selecting the feature corresponding
to the LLVM version you want:

```toml
[dependencies]
haybale = { version = "0.7.1", features = ["llvm-13"] }
```

Currently, the supported LLVM versions are `llvm-9`, `llvm-10`, `llvm-11`,
`llvm-12`, and `llvm-13`.

`haybale` depends (indirectly) on the LLVM and Boolector libraries.
* LLVM must be available on your system, in the version which matches the
selected feature. (For instance, if you select the `llvm-13` feature, LLVM 13
must be available on your system.) For more details and instructions on
installing LLVM and making sure Cargo can find it, see the [`llvm-sys`] README.
* For Boolector you have two options:
    * You can compile and install Boolector 3.2.1 on your system as a shared library. (Make sure you configure it as a shared library, e.g., using
    `./configure.sh --shared`, and install it, using `make install`.)
    * Or, you can enable the `haybale` feature `vendor-boolector`. With this
    option, Cargo will automatically download and build Boolector and statically
    link to it. E.g.,
      ```toml
      [dependencies]
      haybale = { version = "0.7.1", features = ["llvm-13", "vendor-boolector"] }
      ```
      This option probably only works on Linux and macOS, and requires standard
      build tools to be available on your system -- e.g., for Debian-based
      distributions, `build-essential`, `cmake`, `curl`, and `git`.

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

Note that in order for `haybale` to print source-location information (e.g.,
source filename and line number) in error messages and backtraces, the LLVM
bitcode will need to include debuginfo.
You can ensure debuginfo is included by passing the `-g` flag to `clang`,
`clang++`, or `rustc` when generating bitcode.

### 3. Create a Project

A `haybale` [`Project`] contains all of the code currently being analyzed, which
may be one or more LLVM modules.
To get started, simply create a `Project` from a single bitcode file:

```rust
let project = Project::from_bc_path("/path/to/file.bc")?;
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
match find_zero_of_func("foo", &project, Config::default(), None) {
    Ok(None) => println!("foo can never return 0"),
    Ok(Some(inputs)) => println!("Inputs for which foo returns 0: {:?}", inputs),
    Err(e) => panic!("{}", e),  // use the pretty Display impl for errors
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
let mut em = symex_function("foo", &project, Config::<DefaultBackend>::default(), None);
```

Here it was necessary to not only specify the default `haybale`
configuration, as we did when calling `find_zero_of_func()`, but also what
"backend" we want to use.
The `DefaultBackend` should be fine for most purposes.

### Paths

The `ExecutionManager` acts like an `Iterator` over _paths_ through the function `foo`.
Each path is one possible sequence of control-flow decisions (e.g., which direction
do we take at each `if` statement) leading to the function returning some value.
The function `foo` in this example has two paths, one following the "true" branch and
one following the "false" branch of the `if`.

Let's examine the first path through the function:

```rust
let result = em.next().expect("Expected at least one path");
```

In the common case, `result` contains the function return value on this path,
as a Boolector [`BV`] (bitvector) wrapped in the [`ReturnValue`] enum.
Since we know that `foo` isn't a void-typed function (and won't throw an
exception or abort), we can simply unwrap the `ReturnValue` to get the `BV`:

```rust
let retval = match result {
    Ok(ReturnValue::Return(r)) => r,
    Ok(ReturnValue::ReturnVoid) => panic!("Function shouldn't return void"),
    Ok(ReturnValue::Throw(_)) => panic!("Function shouldn't throw an exception"),
    Ok(ReturnValue::Abort) => panic!("Function shouldn't panic or exit()"),
    ...
```

`result` could also be an `Err` describing an [`Error`] which was encountered
while processing the path. In this case, we could just ignore the error and
keep calling `next()` to try to find paths which didn't have errors. Or we
could get information about the error like this:

```rust
    ...
    Err(e) => panic!("{}", em.state().full_error_message_with_context(e)),
};
```

This gets information about the error from the program `State`, which we'll
discuss next. But for the rest of this tutorial, we'll assume that we got the
`Ok` result, and at this point `retval` is a `BV` representing the function
return value on the first path.

### States

For each path, the [`ExecutionManager`] provides not only the final result of
the path (either a`ReturnValue` or an `Error`), but also the final program
[`State`] at the end of that path.
We can get immutable access to the `State` with `state()`, or mutable access
with `mut_state()`.

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

Full documentation for `haybale` can be found [on docs.rs](https://docs.rs/haybale),
or of course you can generate local documentation with `cargo doc --open`.

## Compatibility

Currently, the official crates.io releases of `haybale` (`0.7.0` and later)
depend on Boolector 3.2.1 and LLVM 9, 10, 11, 12, or 13, selected via feature
flags `llvm-9` through `llvm-13`.
As of this writing, choosing an LLVM version has essentially no effect
on `haybale`'s features or interface; the only difference is the ability to
analyze bitcode generated with newer LLVMs. (And the LLVM 10+ versions
can process `AtomicRMW` instructions; see
[#12](https://github.com/PLSysSec/haybale/issues/12).)

For LLVM 8, you can try the `llvm-8` branch of this repo. This branch is
unmaintained, and is approximately at feature parity with `haybale` 0.2.1.
It may work for your purposes; or you can update to LLVM 9 or later and the
latest `haybale`.

LLVM 7 and earlier are not supported.

`haybale` works on stable Rust, and requires Rust 1.45 or later.

## Under the hood

`haybale` is built using the Rust [`llvm-ir`] crate and the [Boolector] SMT
solver (via the Rust [`boolector`] crate).

## Changelog

### Version 0.7.1 (Oct 21, 2021)

- Support for LLVM 13 via the `llvm-13` feature
- `haybale` now requires Rust 1.45+ (previously 1.43 or 1.44)

### Version 0.7.0 (Aug 26, 2021)

- Support for LLVM 12 via the `llvm-12` feature
- New Cargo feature to vendor Boolector: automatically download, build, and
statically link Boolector as part of the `haybale` build process. See the
"Install" section of the README above.
- [`symex_function()`] now takes an additional argument `params`. You can use
this argument to specify constraints for the function parameters, or even
specify specific hardcoded values. Or, you can just pass `None` and get the
previous `haybale` behavior, treating all parameters as completely
unconstrained.
- [`find_zero_of_func()`] and [`get_possible_return_values_of_func()`] likewise
now take a `params` argument to specify constraints on function parameters.
- [`State`] has a new public field `proj` providing access to the [`Project`].
- Function hooks no longer take a `Project` parameter explicitly. Instead, you
can access the `Project` through the `proj` field of the `State` object.
- [`ExecutionManager`] has a new public method `.func()` which provides access
to the toplevel `Function`.
- [`State`] has a new public method `get_path_length()`, also available as the
toplevel function [`get_path_length()`].
- Updated `llvm-ir` dependency to 0.8.0, which results in minor breaking changes
to parts of `haybale`'s API, where `llvm-ir` types are exposed.

### Version 0.6.4 (Apr 22, 2021)

- Fix the build with Rust 1.51+ ([#16](https://github.com/PLSysSec/haybale/issues/16)).
(Minimum Rust version for `haybale` remains unchanged: 1.43+ for LLVM 9 or 10
users, or 1.44+ for LLVM 11 users.)

### Version 0.6.3 (Oct 26, 2020)

- Fix the documentation build on [docs.rs](https://docs.rs/haybale)
([#13](https://github.com/PLSysSec/haybale/issues/13))

### Version 0.6.2 (Oct 20, 2020)

- Support for LLVM 11 via the `llvm-11` feature
- [`get_possible_return_values_of_func()`] now handles void functions properly
([#10](https://github.com/PLSysSec/haybale/issues/10))
- Support LLVM `atomicrmw` instructions (only for LLVM 10+)
([#12](https://github.com/PLSysSec/haybale/issues/12))
- Support LLVM `freeze` instructions (which only exist in LLVM 10+)
- Built-in support for a few more Rust standard-library functions related to
panic handling
- [`State`] has a new public method [`get_bv_by_irname()`]
- LLVM 11 users need Rust 1.44+, due to requirements of `llvm-ir`. LLVM 9 or
10 users still need only Rust 1.43+.

### Version 0.6.1 (Sep 17, 2020)

- Both [`State`] and [`Project`] now have a method `size_in_bits()` which
gets the size of any `Type` in bits, accounting for the `Project`'s pointer
size and struct definitions. This is intended to replace `state.size()` and
`state.size_opaque_aware()`, both of which are now deprecated and will be
removed in `haybale` 0.7.0. Likewise, `state.fp_size()` was deprecated and
renamed to `state.fp_size_in_bits()`.
    - Note: these deprecated methods were actually removed in 0.7.1.

### Version 0.6.0 (Sep 1, 2020)

- `haybale` now supports both LLVM 9 and LLVM 10, using the same branch and
same crates.io releases.
When using `haybale`, you must choose either the `llvm-9` or the `llvm-10`
feature.
- Updated `llvm-ir` dependency to 0.7.1 (from 0.6.0), which includes runtime
and memory-usage performance improvements, particularly for large bitcode
files. This also involves a few breaking changes to parts of `haybale`'s API.
- `haybale` now requires Rust 1.43+ (previously 1.40+) due to requirements
of `llvm-ir` 0.7.1.

### Version 0.5.1 (Aug 31, 2020)

- Fix for [issue #9](https://github.com/PLSysSec/haybale/issues/9) regarding
zero-element arrays (which particularly may appear when analyzing Rust code)
- Built-in support for the `llvm.ctlz` and `llvm.cttz` intrinsics

### Version 0.5.0 (Jul 29, 2020)

Compatibility:
- `haybale` now depends on LLVM 10 by default (up from LLVM 9). LLVM 9 is
still supported on a separate branch; see "Compatibility" above.
- Updated `boolector` dependency to crate version 0.4.0, which requires
Boolector version 3.2.1 (up from 3.1.0).

Renames which affect the public API:
- Rename `SimpleMemoryBackend` to `DefaultBackend` and make it default.
Rename `BtorBackend` to `CellMemoryBackend`, and the `memory` module to
`cell_memory`.
- Remove the `layout` module. Its functions are now available as methods on
[`State`]. Also, many of these functions now return `u32` instead of `usize`.

32-bit targets and related changes:
- With `DefaultBackend`, `haybale` now supports LLVM bitcode which was
compiled for 32-bit targets (previously only supported 64-bit targets).
- The [`new_uninitialized()`] and [`new_zero_initialized()`] methods on the
[`backend::Memory`] trait, `simple_memory::Memory`, and `cell_memory::Memory`
now take an additional parameter indicating the pointer size.
- `Project` has a new public method [`pointer_size_bits()`].

Other:
- Built-in support for the `llvm.expect` intrinsic, and built-in support for
the `llvm.bswap` intrinsic with vector operands (previously only supported
scalar operands)
- [`solver_utils::PossibleSolutions`] has new constructors `empty()`,
`exactly_one()`, and `exactly_two()` (useful for testing), and also
implements `FromIterator`, allowing you to `.collect()` an iterator into it
- Bugfix for the `{min,max}_possible_solution_for_bv_as_binary_str()`
functions in the `solver_utils` module

### Version 0.4.0 (Mar 31, 2020)

New features:
- Support LLVM `cmpxchg` instructions
- Support for instruction callbacks - see [`Config.callbacks`]. This allows
you to take arbitrary actions based on the instruction about to be processed.

Config:
- `Config.null_detection` has been renamed to
[`Config.null_pointer_checking`], and its type has been changed to allow for
additional options.
- `Config::new()` now takes no parameters. It is now the same as
`Config::default()` except that it comes with no function hooks.

Other utility functions/methods:
- The `hook_utils` module now includes two new functions [`memset_bv`] and
[`memcpy_bv`].
- [`layout::size_opaque_aware`] now returns an `Option` rather than panicking.
- The `to_string_*` methods on [`Location`] are now public, rather than
internal to the crate, allowing users more control over the `String`
representation of a `Location`.

Error handling:
- [`Error`] has three new variants `UnreachableInstruction`,
`FailedToResolveFunctionPointer`, and `HookReturnValueMismatch`. All of these
were previously reported as `Error::OtherError`, but now have dedicated
variants.
- `Error::LoopBoundExceeded` now also includes the value of the loop bound
which was exceeded.

Other notes:
- `haybale` no longer selects features of the `log` crate. This allows
downstream users to select these features or not, and in particular, allows
users to enable debug logging in release builds.

### Version 0.3.2 (Feb 28, 2020)

- New option [`Config.max_callstack_depth`] allows you to limit the callstack
depth for an analysis - automatically ignoring calls of LLVM functions which
would exceed that callstack depth. The default for this setting is no limit,
matching the previous behavior of `haybale`.
- New option [`Config.max_memcpy_length`] allows you to limit the maximum
size of `memcpy`, `memset`, and `memmove` operations. The default for this
setting is no limit, matching the previous behavior of `haybale`.
- New method [`FunctionHooks::add_default_hook()`] allows you to supply a
"default hook" which will be used when no other definition or hook is found
for a function call. If no default hook is provided, this will result in a
`FunctionNotFound` error, just as it did previously.
- Performance improvements for analyzing calls of function pointers.
- Improved a few error messages.

### Version 0.3.1 (Feb 5, 2020)

- Fix some broken links in the README and docs. No functional changes.

### Version 0.3.0 (Feb 5, 2020)

Solver timeouts:
- New setting [`Config.solver_query_timeout`] controls the maximum amount of
time `haybale` will spend on a single solver query before returning
`Error::SolverError`. This setting defaults to 300 seconds (5 minutes).
The setting can also be disabled entirely, which results in the same behavior
as previous versions of `haybale` (no time limit on solver queries).

Error handling:
- The errors returned by `ExecutionManager.next()` are now `haybale::Error`s
instead of `String`s, allowing callers to more easily handle different kinds
of errors different ways. To get a string representation of the `Error`,
`.to_string()` gives the short description, while
[`State.full_error_message_with_context()`] gives the full description which
previously was returned by `ExecutionManager.next()`. The usage example in
the README has been updated accordingly.
- The toplevel function [`find_zero_of_func()`] now returns a
`Result`, with the error type being `String`.
- New setting [`Config.squash_unsats`] controls whether `Error::Unsat`s are
silently squashed (the default behavior, and the behavior of previous
versions of `haybale`), or returned to the user. For more details, see the
docs on that setting.

Logging, error messages, backtraces, etc:
- `haybale` now prints source-location information (e.g., source filename and
line number) in error messages and backtraces when it is available.
Similarly, the `HAYBALE_DUMP_PATH` environment variable now has the options
`LLVM`, `SRC`, and `BOTH`. For more details on all of this, see
[`Config.print_source_info`].
- You can also now _disable_ printing the LLVM module name along with LLVM
location info in error messages, backtraces, path dumps, and log messages.
For more details, see [`Config.print_module_name`].
- `haybale` will now by default autodetect when C++ or Rust demangling is
appropriate for the `Project`, unless a different setting is chosen in
[`Config.demangling`].
- Numeric constants representing `BV` values in log messages,
`HAYBALE_DUMP_VARS` dumps, etc are now all printed in hexadecimal (previously
binary, or an inconsistent mix of binary and hexadecimal).

Function hooks and intrinsics:
- Built-in support for LLVM arithmetic-with-overflow intrinsics.
- Built-in support for LLVM saturating-arithmetic intrinsics.
- Built-in support for the `llvm.assume` intrinsic, with an associated
setting [`Config.trust_llvm_assumes`].
- Built-in support for the `llvm.bswap` intrinsic with argument sizes 48
or 64 bits (previously only supported 16 or 32 bits).
- Default hooks for a number of Rust standard-library functions which
always panic, such as `core::result::unwrap_failed()`.
- New module `hook_utils` contains the implementations of `memset` and
`memcpy` used by the corresponding built-in hooks. These are now publically
available for use in custom hooks for other functions.

Changes to data structures and traits:
- The `Location` and `PathEntry` structs have been refactored to include
source-location information when it is available, to be capable of indicating
basic block terminators in addition to normal instructions, and to support
some internal refactoring.
- The [`backend::BV`] trait has a new required method, `get_solver()`, which
returns a `SolverRef` of the appropriate type. (This is similar to the same
method on the `backend::Memory` trait.)
- Saturating-arithmetic methods (signed and unsigned addition and subtraction)
are now available on [`backend::BV`], with default implementations in terms
of the other trait methods. That means that these come "for free" once the
required trait methods are implemented.
- `zero_extend_to_bits()` and `sign_extend_to_bits()` are also now available
as trait methods on [`backend::BV`], with default implementations in terms of
the other trait methods. Previously they were private utility functions in
`haybale`.
- Many other structures have had minor changes and improvements, including
some small breaking changes.

Compatibility:
- Updated `boolector` dependency to crate version 0.3.0, which requires
Boolector version 3.1.0 (up from 3.0.0).
- This version of `haybale` now requires Rust 1.40+, up from 1.36+ for
previous versions of `haybale`.

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
section in the README.
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
[KLEE]: https://klee.github.io/
[`Project`]: https://docs.rs/haybale/0.7.1/haybale/project/struct.Project.html
[`Project` documentation]: https://docs.rs/haybale/0.7.1/haybale/project/struct.Project.html
[`Project::get_func_by_name()`]: https://docs.rs/haybale/0.7.1/haybale/project/struct.Project.html#method.get_func_by_name
[`get_possible_return_values_of_func()`]: https://docs.rs/haybale/0.7.1/haybale/fn.get_possible_return_values_of_func.html
[`find_zero_of_func()`]: https://docs.rs/haybale/0.7.1/haybale/fn.find_zero_of_func.html
[`ExecutionManager`]: https://docs.rs/haybale/0.7.1/haybale/struct.ExecutionManager.html
[`ExecutionManager` documentation]: https://docs.rs/haybale/0.7.1/haybale/struct.ExecutionManager.html
[`symex_function()`]: https://docs.rs/haybale/0.7.1/haybale/fn.symex_function.html
[`get_path_length()`]: https://docs.rs/haybale/0.7.1/haybale/fn.get_path_length.html
[`Config`]: https://docs.rs/haybale/0.7.1/haybale/config/struct.Config.html
[`BV`]: https://docs.rs/boolector/0.4.2/boolector/struct.BV.html
[`ReturnValue`]: https://docs.rs/haybale/0.7.1/haybale/enum.ReturnValue.html
[`Error`]: https://docs.rs/haybale/0.7.1/haybale/enum.Error.html
[`State`]: https://docs.rs/haybale/0.7.1/haybale/struct.State.html
[`Location`]: https://docs.rs/haybale/0.7.1/haybale/struct.Location.html
[`Project::get_inner_struct_type_from_named()`]: https://docs.rs/haybale/0.7.1/haybale/struct.Project.html#method.get_inner_struct_type_from_named
[`State::add_mem_watchpoint()`]: https://docs.rs/haybale/0.7.1/haybale/struct.State.html#method.add_mem_watchpoint
[`FunctionHooks::add_cpp_demangled()`]: https://docs.rs/haybale/0.7.1/haybale/function_hooks/struct.FunctionHooks.html#method.add_cpp_demangled
[`FunctionHooks::add_rust_demangled()`]: https://docs.rs/haybale/0.7.1/haybale/function_hooks/struct.FunctionHooks.html#method.add_rust_demangled
[`FunctionHooks::add_inline_asm_hook()`]: https://docs.rs/haybale/0.7.1/haybale/function_hooks/struct.FunctionHooks.html#method.add_inline_asm_hook
[`FunctionHooks::add_default_hook()`]: https://docs.rs/haybale/0.7.1/haybale/function_hooks/struct.FunctionHooks.html#method.add_default_hook
[`function_hooks`]: https://docs.rs/haybale/0.7.1/haybale/function_hooks/index.html
[`generic_stub_hook`]: https://docs.rs/haybale/0.7.1/haybale/function_hooks/fn.generic_stub_hook.html
[`abort_hook`]: https://docs.rs/haybale/0.7.1/haybale/function_hooks/fn.abort_hook.html
[`Config.initial_mem_watchpoints`]: https://docs.rs/haybale/0.7.1/haybale/config/struct.Config.html#structfield.initial_mem_watchpoints
[`Config.demangling`]: https://docs.rs/haybale/0.7.1/haybale/config/struct.Config.html#structfield.demangling
[`Config.print_source_info`]: https://docs.rs/haybale/0.7.1/haybale/config/struct.Config.html#structfield.print_source_info
[`Config.print_module_name`]: https://docs.rs/haybale/0.7.1/haybale/config/struct.Config.html#structfield.print_module_name
[`Config.trust_llvm_assumes`]: https://docs.rs/haybale/0.7.1/haybale/config/struct.Config.html#structfield.trust_llvm_assumes
[`Config.solver_query_timeout`]: https://docs.rs/haybale/0.7.1/haybale/config/struct.Config.html#structfield.solver_query_timeout
[`Config.squash_unsats`]: https://docs.rs/haybale/0.7.1/haybale/config/struct.Config.html#structfield.squash_unsats
[`Config.max_callstack_depth`]: https://docs.rs/haybale/0.7.1/haybale/config/struct.Config.html#structfield.max_callstack_depth
[`Config.max_memcpy_length`]: https://docs.rs/haybale/0.7.1/haybale/config/struct.Config.html#structfield.max_memcpy_length
[`Config.callbacks`]: https://docs.rs/haybale/0.7.1/haybale/config/struct.Config.html#structfield.callbacks
[`Config.null_pointer_checking`]: https://docs.rs/haybale/0.7.1/haybale/config/struct.Config.html#structfield.null_pointer_checking
[`backend::BV`]: https://docs.rs/haybale/0.7.1/haybale/backend/trait.BV.html
[`backend::Memory`]: https://docs.rs/haybale/0.7.1/haybale/backend/trait.Memory.html
[`new_uninitialized()`]: https://docs.rs/haybale/0.7.1/haybale/backend/trait.Memory.html#tymethod.new_uninitialized
[`new_zero_initialized()`]: https://docs.rs/haybale/0.7.1/haybale/backend/trait.Memory.html#tymethod.new_zero_initialized
[`State.full_error_message_with_context()`]: https://docs.rs/haybale/0.7.1/haybale/struct.State.html#method.full_error_message_with_context
[`memcpy_bv`]: https://docs.rs/haybale/0.7.1/haybale/hook_utils/fn.memcpy_bv.html
[`memset_bv`]: https://docs.rs/haybale/0.7.1/haybale/hook_utils/fn.memset_bv.html
[`layout::size_opaque_aware`]: https://docs.rs/haybale/0.7.1/haybale/struct.State.html#method.size_opaque_aware
[`pointer_size_bits()`]: https://docs.rs/haybale/0.7.1/haybale/struct.Project.html#method.pointer_size_bits
[`solver_utils::PossibleSolutions`]: https://docs.rs/haybale/0.7.1/haybale/solver_utils/enum.PossibleSolutions.html
[`get_bv_by_irname()`]: https://docs.rs/haybale/0.7.1/haybale/struct.State.html#method.get_bv_by_irname
