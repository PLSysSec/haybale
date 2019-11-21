# `haybale`: Symbolic execution of LLVM IR, written in Rust

[![Crates.io](http://meritbadge.herokuapp.com/haybale)](https://crates.io/crates/haybale)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/cdisselkoen/haybale/master/LICENSE)

`haybale` is a general-purpose symbolic execution engine written in Rust.
It operates on LLVM IR, which allows it to analyze programs written in any
language which compiles to LLVM IR - C/C++, Rust, Swift, and more.
In this way, it may be compared to [KLEE], as it has similar goals, except
that `haybale` is written in Rust.
That said, `haybale` makes no claim of being at feature parity with KLEE.

### Okay, but what is a symbolic execution engine?

A symbolic execution engine is essentially a way of mathematically reasoning
about the behavior of a function or program.
It can reason about _all possible inputs_ to a function without literally
brute-forcing every single one.
For instance, a symbolic execution engine like `haybale` can answer questions
like:

- Are there any inputs to <some function> that cause it to return 0? What are they?
- Is it possible for this loop to execute exactly 17 times?
- Can this pointer ever be NULL?

Symbolic execution engines answer these questions by converting each variable in
the program or function into a mathematical expression which depends on the
function or program inputs.
It then uses an SMT solver to answer questions about these expressions, such
as the questions listed above.

## Getting started

### 1. Install

`haybale` is on [crates.io](https://crates.io/crates/haybale), so you can simply
add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
haybale = "0.1.0"
```

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

### 4. Analyze!

For an introductory example, let's suppose we're analyzing the following C function:

```c
int foo(int a, int b) {
    if (a > b) {
        return (a-1) * (b-1);
    } else {
        return (a + b) % 3 + 10;
    }
}
```

and we want to know if the function ever returns `0`, and if so, for what inputs.

First, we create an [`ExecutionManager`] which controls the progress of the symbolic
execution:

```rust
let mut em = symex_function("foo", &project, Config::<BtorBackend>::default());
```

Here we used the `project` created above, and we use the default `haybale`
configuration and backend.

The `ExecutionManager` acts like an `Iterator` over _paths_ through the function `foo`.
Each path is one possible sequence of control-flow decisions (e.g., which direction
do we take at each `if` statement) leading to the function returning some value.
The function `foo` in this example has two paths, one following the "true" branch and
one following the "false" branch of the `if`.

Let's examine the first path through the function:

```rust
let retval = em.next().expect("Expected at least one path")?;
```

We're given the function return value, `retval`, as a Boolector `BV` (bitvector).
We're interested in whether that `retval` can ever equal `0`:

```rust
let retval = match retval {
    ReturnValue::ReturnVoid => panic!("Function shouldn't return void"),
    ReturnValue::Return(r) => r,
};
let state = em.mut_state();  // the final program state along this path
let zero = BV::zero(state.solver.clone(), 32);  // The 32-bit constant 0
if state.bvs_can_be_equal(&retval, &zero)? {
    println!("retval can be 0!");
}
```

If it can be `0`, let's find what values of the function parameters would cause that:

```rust
// Constrain that the return value must be 0
retval._eq(&zero).assert();

// Get a possible solution for the first parameter. In this case, from looking at the
// text-format LLVM IR, we know the variable we're looking for is variable #0 in the
// function "foo".
let a = state.get_a_solution_for_bv_by_irname(&String::from("foo"), Name::from(0))?
    .expect("Expected there to be a solution")
    .as_u64()
    .expect("Expected solution to fit in 64 bits");

// Likewise the second parameter, which is variable #1 in "foo"
let b = state.get_a_solution_for_bv_by_irname(&String::from("foo"), Name::from(1))?
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

Documentation for `haybale` can be found [here](https://PLSysSec.github.io/haybale),
or of course you can generate local documentation with `cargo doc --open`.

## Compatibility

Currently, `haybale` only supports LLVM 8.

`haybale` works on stable Rust, and requires Rust 1.36+.

## Under the hood

`haybale` is built using the Rust [`llvm-ir`] crate and the [Boolector] SMT
solver (via the Rust [`boolector`] crate).

[`llvm-ir`]: https://crates.io/crates/llvm-ir
[Boolector]: https://boolector.github.io/
[`boolector`]: https://crates.io/crates/boolector
[KLEE]: https://klee.github.io/
[`Project`]: https://PLSysSec.github.io/haybale/haybale/project/struct.Project.html
[`Project` documentation]: https://PLSysSec.github.io/haybale/haybale/project/struct.Project.html
[`ExecutionManager`]: https://PLSysSec.github.io/haybale/haybale/struct.ExecutionManager.html
