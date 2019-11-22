# `haybale`: Symbolic execution of LLVM IR, written in Rust

[![Crates.io](http://meritbadge.herokuapp.com/haybale)](https://crates.io/crates/haybale)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/cdisselkoen/haybale/master/LICENSE)

`haybale` is a general-purpose symbolic execution engine written in Rust.
It operates on LLVM IR, which allows it to analyze programs written in any
language which compiles to LLVM IR - C/C++, Rust, Swift, and more.
In this way, it may be compared to [KLEE], as it has similar goals, except
that `haybale` is written in Rust and makes some different design decisions.
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

### 4. Use built-in analyses

`haybale` currently includes two built-in analyses:
[`get_possible_return_values_of_func`], which describes all the possible
values a function could return for any input, and [`find_zero_of_func`],
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

We can use `find_zero_of_func` to find inputs such that `foo` will return `0`:

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
`foo` above without using the built-in `find_zero_of_func`.
This will illustrate how to write a custom analysis using `haybale`.

### ExecutionManager

All analyses will use an [`ExecutionManager`] to control the progress of the
symbolic execution.
In the code snippet below, we call `symex_function` to create an
`ExecutionManager` which will analyze the function `foo` - it will start at
the top of the function, and end when the function returns.

```rust
let mut em = symex_function("foo", &project, Config::<BtorBackend>::default());
```

Here it was necessary to not only specify the default `haybale`
configuration, as we did when calling `find_zero_of_func`, but also what
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

We're given the function return value, `retval`, as a Boolector `BV` (bitvector)
wrapped in the `ReturnValue` enum.
Since we know that `foo` isn't a void-typed function, we can simply unwrap the
`ReturnValue` to get the `BV`:

```rust
let retval = match retval {
    ReturnValue::ReturnVoid => panic!("Function shouldn't return void"),
    ReturnValue::Return(r) => r,
};
```

### States

Importantly, the `ExecutionManager` provides not only the final return value of
the path as a `BV`, but also the final program `State` at the end of that path,
either immutably with `state()` or mutably with `mut_state()`. (See the
[`ExecutionManager` documentation] for more.)

```rust
let state = em.mut_state();  // the final program state along this path
```

To test whether `retval` can be equal to `0` in this `State`, we can use
`state.bvs_can_be_equal()`:

```rust
let zero = BV::zero(state.solver.clone(), 32);  // The 32-bit constant 0
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

Full documentation for `haybale` can be found [here](https://PLSysSec.github.io/haybale),
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
[`get_possible_return_values_of_func`]: https://PLSysSec.github.io/haybale/haybale/fn.get_possible_return_values_of_func.html
[`find_zero_of_func`]: https://PLSysSec.github.io/haybale/haybale/fn.find_zero_of_func.html
[`ExecutionManager`]: https://PLSysSec.github.io/haybale/haybale/struct.ExecutionManager.html
[`ExecutionManager` documentation]: https://PLSysSec.github.io/haybale/haybale/struct.ExecutionManager.html
