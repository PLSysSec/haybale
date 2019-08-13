use crate::possible_solutions::*;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

/// A `Backend` is just a collection of types which together implement the necessary traits
pub trait Backend<'ctx> : 'ctx {
    type BV: BV<'ctx, AssociatedBool = Self::Bool> + 'ctx;
    type Bool: Bool<'ctx, AssociatedBV = Self::BV> + 'ctx;
    type Memory: Memory<'ctx, Index=Self::BV, Value=Self::BV, BackendState=Self::State> + 'ctx;
    type Solver: Solver<'ctx, Constraint=Self::Bool, Value=Self::BV, BackendState=Self::State> + 'ctx;
    /// Any additional state that the `Backend` needs. This will be stored in the
    /// `state::State` struct.
    ///
    /// Must be `Default`, and the `default()` method will be used to construct
    /// the initial backend state when a blank `state::State` is constructed.
    type State: Clone + Default;
}

/// Trait for things which can act like bitvectors
pub trait BV<'ctx> : Clone + PartialEq + Eq + fmt::Debug {
    type AssociatedBool: Bool<'ctx>;

    fn new(ctx: &'ctx z3::Context, name: impl Into<z3::Symbol>, size: u32) -> Self;
    fn from_i64(ctx: &'ctx z3::Context, i: i64, size: u32) -> Self;
    fn from_u64(ctx: &'ctx z3::Context, u: u64, size: u32) -> Self;
    fn as_i64(&self) -> Option<i64>;
    fn as_u64(&self) -> Option<u64>;
    fn get_size(&self) -> u32;
    fn not(&self) -> Self;
    fn neg(&self) -> Self;
    fn and(&self, other: &Self) -> Self;
    fn or(&self, other: &Self) -> Self;
    fn xor(&self, other: &Self) -> Self;
    fn nand(&self, other: &Self) -> Self;
    fn nor(&self, other: &Self) -> Self;
    fn xnor(&self, other: &Self) -> Self;
    fn redand(&self) -> Self;
    fn redor(&self) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn udiv(&self, other: &Self) -> Self;
    fn sdiv(&self, other: &Self) -> Self;
    fn urem(&self, other: &Self) -> Self;
    fn srem(&self, other: &Self) -> Self;
    fn smod(&self, other: &Self) -> Self;
    fn ult(&self, other: &Self) -> Self::AssociatedBool;
    fn slt(&self, other: &Self) -> Self::AssociatedBool;
    fn ule(&self, other: &Self) -> Self::AssociatedBool;
    fn sle(&self, other: &Self) -> Self::AssociatedBool;
    fn uge(&self, other: &Self) -> Self::AssociatedBool;
    fn sge(&self, other: &Self) -> Self::AssociatedBool;
    fn ugt(&self, other: &Self) -> Self::AssociatedBool;
    fn sgt(&self, other: &Self) -> Self::AssociatedBool;
    fn shl(&self, other: &Self) -> Self;
    fn lshr(&self, other: &Self) -> Self;
    fn ashr(&self, other: &Self) -> Self;
    fn rotl(&self, other: &Self) -> Self;
    fn rotr(&self, other: &Self) -> Self;
    fn concat(&self, other: &Self) -> Self;
    fn extract(&self, high: u32, low: u32) -> Self;
    fn sign_ext(&self, i: u32) -> Self;
    fn zero_ext(&self, i: u32) -> Self;
    fn _eq(&self, other: &Self) -> Self::AssociatedBool;
    fn simplify(&self) -> Self {
        // default implementation, many implementors will do better
        self.clone()
    }
}

/// Trait for things which can act like booleans
pub trait Bool<'ctx> : Clone + PartialEq + Eq + fmt::Debug {
    type AssociatedBV: BV<'ctx>;

    fn new(ctx: &'ctx z3::Context, name: impl Into<z3::Symbol>) -> Self;
    fn from_bool(ctx: &'ctx z3::Context, b: bool) -> Self;
    fn as_bool(&self) -> Option<bool>;
    fn bvite(&self, a: &Self::AssociatedBV, b: &Self::AssociatedBV) -> Self::AssociatedBV;
    fn boolite(&self, a: &Self, b: &Self) -> Self;
    fn and(&self, other: &[&Self]) -> Self;
    fn or(&self, other: &[&Self]) -> Self;
    fn xor(&self, other: &Self) -> Self;
    fn not(&self) -> Self;
    fn iff(&self, other: &Self) -> Self;
    fn implies(&self, other: &Self) -> Self;
    fn _eq(&self, other: &Self) -> Self;
    fn simplify(&self) -> Self {
        // default implementation, many implementors will do better
        self.clone()
    }
}

/// Trait for things which can act like 'memories', that is, maps from bitvector (addresses) to bitvector (values)
pub trait Memory<'ctx> : Clone + PartialEq + Eq {
    type Index: BV<'ctx>;
    type Value: BV<'ctx>;
    type BackendState;

    fn new_uninitialized(ctx: &'ctx z3::Context, backend_state: Rc<RefCell<Self::BackendState>>) -> Self;
    fn new_zero_initialized(ctx: &'ctx z3::Context, backend_state: Rc<RefCell<Self::BackendState>>) -> Self;
    fn read(&self, index: &Self::Index, bits: u32) -> Self::Value;
    fn write(&mut self, index: &Self::Index, value: Self::Value);
}

pub trait Solver<'ctx> {
    type Constraint: Bool<'ctx, AssociatedBV = Self::Value>;
    type Value: BV<'ctx, AssociatedBool = Self::Constraint>;
    type BackendState;

    /// A new `Solver` with no constraints
    fn new(ctx: &'ctx z3::Context, backend_state: Rc<RefCell<Self::BackendState>>) -> Self;

    /// Get the `Context` this `Solver` was created with
    fn get_context(&self) -> &'ctx z3::Context;

    /// Add `constraint` as a constraint, i.e., assert that `constraint` must be true
    fn assert(&mut self, constraint: &Self::Constraint);

    /// Returns `true` if current constraints are satisfiable, `false` if not.
    ///
    /// Returns `Err` if the query failed (e.g., was interrupted or timed out).
    fn check(&mut self) -> Result<bool, &'static str>;

    /// Returns `true` if the current constraints plus the additional constraints `conds`
    /// are together satisfiable, or `false` if not.
    ///
    /// Returns `Err` if the query failed (e.g., was interrupted or timed out).
    ///
    /// Does not permanently add the constraints in `conds` to the solver.
    fn check_with_extra_constraints<'a>(&'a mut self, constraints: impl Iterator<Item = &'a Self::Constraint>) -> Result<bool, &'static str> where Self::Constraint: 'a {
        // a default implementation in terms of check(), assert(), push(), and pop()
        self.push();
        for constraint in constraints {
            self.assert(constraint);
        }
        let retval = self.check();
        self.pop(1);
        retval
    }

    fn push(&mut self);

    /// `n`: number of `push`es to backtrack
    fn pop(&mut self, n: usize);

    /// Get one possible concrete value for the `BV`.
    /// Returns `Ok(None)` if no possible solution, or `Err` if solver query failed.
    fn get_a_solution_for_bv(&mut self, bv: &Self::Value) -> Result<Option<u64>, &'static str>;

    /// Get one possible concrete value for specified bits (`high`, `low`) of the `BV`, inclusive on both ends.
    /// Returns `Ok(None)` if no possible solution, or `Err` if solver query failed.
    fn get_a_solution_for_specified_bits_of_bv(&mut self, bv: &Self::Value, high: u32, low: u32) -> Result<Option<u64>, &'static str>;

    /// Get one possible concrete value for the `Bool`.
    /// Returns `Ok(None)` if no possible solution, or `Err` if solver query failed.
    fn get_a_solution_for_bool(&mut self, b: &Self::Constraint) -> Result<Option<bool>, &'static str>;

    /// Get a description of the possible solutions for the `BV`.
    ///
    /// `n`: Maximum number of distinct solutions to return.
    /// If there are more than `n` possible solutions, this simply
    /// returns `PossibleSolutions::MoreThanNPossibleSolutions(n)`.
    fn get_possible_solutions_for_bv(&mut self, bv: &Self::Value, n: usize) -> Result<PossibleSolutions<u64>, &'static str> {
        // a default implementation in terms of get_a_solution_for_bv(), assert(), get_context(), push(), and pop()
        let mut solutions = vec![];
        self.push();
        while solutions.len() <= n {
            match self.get_a_solution_for_bv(bv)? {
                None => break,  // no more possible solutions, we're done
                Some(val) => {
                    solutions.push(val);
                    // Temporarily constrain that the solution can't be `val`, to see if there is another solution
                    self.assert(&bv._eq(&Self::Value::from_u64(self.get_context(), val, bv.get_size())).not());
                }
            }
        }
        self.pop(1);
        if solutions.len() > n {
            Ok(PossibleSolutions::MoreThanNPossibleSolutions(n))
        } else {
            Ok(PossibleSolutions::PossibleSolutions(solutions))
        }
    }

    /// Get a description of the possible solutions for the `Bool`.
    ///
    /// Since there cannot be more than two solutions (`true` and `false`),
    /// this should never return the `PossibleSolutions::MoreThanNPossibleSolutions` variant.
    /// Instead, it should only return one of these four things:
    ///   - `PossibleSolutions::PossibleSolutions(vec![])` indicating no possible solution,
    ///   - `PossibleSolutions::PossibleSolutions(vec![true])`,
    ///   - `PossibleSolutions::PossibleSolutions(vec![false])`,
    ///   - `PossibleSolutions::PossibleSolutions(vec![true, false])`
    fn get_possible_solutions_for_bool(&mut self, b: &Self::Constraint) -> Result<PossibleSolutions<bool>, &'static str> {
        // a default implementation in terms of get_a_solution_for_bool(), assert(), get_context(), push(), and pop()
        self.push();
        let retval = match self.get_a_solution_for_bool(b)? {
            None => PossibleSolutions::PossibleSolutions(vec![]),
            Some(val) => {
                // Temporarily constrain that the solution can't be `val`, to see if there is another solution
                self.assert(&b._eq(&Bool::from_bool(self.get_context(), val)).not());
                match self.get_a_solution_for_bool(b)? {
                    None => PossibleSolutions::PossibleSolutions(vec![val]),
                    Some(_) => PossibleSolutions::PossibleSolutions(vec![true, false]),
                }
            }
        };
        self.pop(1);
        Ok(retval)
    }

    fn current_model_to_pretty_string(&self) -> String;
}

/// The prototypical `BV`, `Bool`, `Memory`, and `Solver` implementations:
///   `z3::ast::BV`, `z3::ast::Bool`, `crate::memory::Memory`, and `crate::solver::Solver`

impl<'ctx> BV<'ctx> for z3::ast::BV<'ctx> {
    type AssociatedBool = z3::ast::Bool<'ctx>;

    fn new(ctx: &'ctx z3::Context, name: impl Into<z3::Symbol>, size: u32) -> Self {
        z3::ast::BV::new_const(ctx, name, size)
    }
    fn from_i64(ctx: &'ctx z3::Context, i: i64, size: u32) -> Self {
        z3::ast::BV::from_i64(ctx, i, size)
    }
    fn from_u64(ctx: &'ctx z3::Context, u: u64, size: u32) -> Self {
        z3::ast::BV::from_u64(ctx, u, size)
    }
    fn as_i64(&self) -> Option<i64> {
        self.as_i64()
    }
    fn as_u64(&self) -> Option<u64> {
        self.as_u64()
    }
    fn get_size(&self) -> u32 {
        self.get_size()
    }
    fn not(&self) -> Self {
        self.bvnot()
    }
    fn neg(&self) -> Self {
        self.bvneg()
    }
    fn and(&self, other: &Self) -> Self {
        self.bvand(other)
    }
    fn or(&self, other: &Self) -> Self {
        self.bvor(other)
    }
    fn xor(&self, other: &Self) -> Self {
        self.bvxor(other)
    }
    fn nand(&self, other: &Self) -> Self {
        self.bvnand(other)
    }
    fn nor(&self, other: &Self) -> Self {
        self.bvnor(other)
    }
    fn xnor(&self, other: &Self) -> Self {
        self.bvxnor(other)
    }
    fn redand(&self) -> Self {
        self.bvredand()
    }
    fn redor(&self) -> Self {
        self.bvredor()
    }
    fn add(&self, other: &Self) -> Self {
        self.bvadd(other)
    }
    fn sub(&self, other: &Self) -> Self {
        self.bvsub(other)
    }
    fn mul(&self, other: &Self) -> Self {
        self.bvmul(other)
    }
    fn udiv(&self, other: &Self) -> Self {
        self.bvudiv(other)
    }
    fn sdiv(&self, other: &Self) -> Self {
        self.bvsdiv(other)
    }
    fn urem(&self, other: &Self) -> Self {
        self.bvurem(other)
    }
    fn srem(&self, other: &Self) -> Self {
        self.bvsrem(other)
    }
    fn smod(&self, other: &Self) -> Self {
        self.bvsmod(other)
    }
    fn ult(&self, other: &Self) -> Self::AssociatedBool {
        self.bvult(other)
    }
    fn slt(&self, other: &Self) -> Self::AssociatedBool {
        self.bvslt(other)
    }
    fn ule(&self, other: &Self) -> Self::AssociatedBool {
        self.bvule(other)
    }
    fn sle(&self, other: &Self) -> Self::AssociatedBool {
        self.bvsle(other)
    }
    fn uge(&self, other: &Self) -> Self::AssociatedBool {
        self.bvuge(other)
    }
    fn sge(&self, other: &Self) -> Self::AssociatedBool {
        self.bvsge(other)
    }
    fn ugt(&self, other: &Self) -> Self::AssociatedBool {
        self.bvugt(other)
    }
    fn sgt(&self, other: &Self) -> Self::AssociatedBool {
        self.bvsgt(other)
    }
    fn shl(&self, other: &Self) -> Self {
        self.bvshl(other)
    }
    fn lshr(&self, other: &Self) -> Self {
        self.bvlshr(other)
    }
    fn ashr(&self, other: &Self) -> Self {
        self.bvashr(other)
    }
    fn rotl(&self, other: &Self) -> Self {
        self.bvrotl(other)
    }
    fn rotr(&self, other: &Self) -> Self {
        self.bvrotr(other)
    }
    fn concat(&self, other: &Self) -> Self {
        self.concat(other)
    }
    fn extract(&self, high: u32, low: u32) -> Self {
        self.extract(high, low)
    }
    fn sign_ext(&self, i: u32) -> Self {
        self.sign_ext(i)
    }
    fn zero_ext(&self, i: u32) -> Self {
        self.zero_ext(i)
    }
    fn _eq(&self, other: &Self) -> Self::AssociatedBool {
        z3::ast::Ast::_eq(self, &other)
    }
    fn simplify(&self) -> Self {
        z3::ast::Ast::simplify(self)
    }
}

impl<'ctx> Bool<'ctx> for z3::ast::Bool<'ctx> {
    type AssociatedBV = z3::ast::BV<'ctx>;

    fn new(ctx: &'ctx z3::Context, name: impl Into<z3::Symbol>) -> Self {
        Self::new_const(ctx, name)
    }
    fn from_bool(ctx: &'ctx z3::Context, b: bool) -> Self {
        Self::from_bool(ctx, b)
    }
    fn as_bool(&self) -> Option<bool> {
        self.as_bool()
    }
    fn bvite(&self, a: &Self::AssociatedBV, b: &Self::AssociatedBV) -> Self::AssociatedBV {
        self.ite(a, b)
    }
    fn boolite(&self, a: &Self, b: &Self) -> Self {
        self.ite(a, b)
    }
    fn and(&self, other: &[&Self]) -> Self {
        self.and(other)
    }
    fn or(&self, other: &[&Self]) -> Self {
        self.or(other)
    }
    fn xor(&self, other: &Self) -> Self {
        self.xor(other)
    }
    fn not(&self) -> Self {
        self.not()
    }
    fn iff(&self, other: &Self) -> Self {
        self.iff(other)
    }
    fn implies(&self, other: &Self) -> Self {
        self.implies(other)
    }
    fn _eq(&self, other: &Self) -> Self {
        z3::ast::Ast::_eq(self, &other)
    }
    fn simplify(&self) -> Self {
        z3::ast::Ast::simplify(self)
    }
}

impl<'ctx> Memory<'ctx> for crate::memory::Memory<'ctx> {
    type Index = z3::ast::BV<'ctx>;
    type Value = z3::ast::BV<'ctx>;
    type BackendState = ();

    fn new_uninitialized(ctx: &'ctx z3::Context, _backend_state: Rc<RefCell<Self::BackendState>>) -> Self {
        crate::memory::Memory::new_uninitialized(ctx)
    }
    fn new_zero_initialized(ctx: &'ctx z3::Context, _backend_state: Rc<RefCell<Self::BackendState>>) -> Self {
        crate::memory::Memory::new_zero_initialized(ctx)
    }
    fn read(&self, index: &Self::Index, bits: u32) -> Self::Value {
        self.read(index, bits)
    }
    fn write(&mut self, index: &Self::Index, value: Self::Value) {
        self.write(index, value)
    }
}

impl<'ctx> Solver<'ctx> for crate::solver::Solver<'ctx> {
    type Constraint = z3::ast::Bool<'ctx>;
    type Value = z3::ast::BV<'ctx>;
    type BackendState = ();

    fn new(ctx: &'ctx z3::Context, _backend_state: Rc<RefCell<Self::BackendState>>) -> Self {
        crate::solver::Solver::new(ctx)
    }
    fn get_context(&self) -> &'ctx z3::Context {
        self.get_context()
    }
    fn assert(&mut self, constraint: &Self::Constraint) {
        self.assert(constraint)
    }
    fn check(&mut self) -> Result<bool, &'static str> {
        self.check()
    }
    fn check_with_extra_constraints<'a>(&'a mut self, constraints: impl Iterator<Item = &'a Self::Constraint>) -> Result<bool, &'static str> {
        self.check_with_extra_constraints(constraints)
    }
    fn push(&mut self) {
        self.push()
    }
    fn pop(&mut self, n: usize) {
        self.pop(n)
    }
    fn get_a_solution_for_bv(&mut self, bv: &Self::Value) -> Result<Option<u64>, &'static str> {
        self.get_a_solution_for_bv(bv)
    }
    fn get_a_solution_for_specified_bits_of_bv(&mut self, bv: &Self::Value, high: u32, low: u32) -> Result<Option<u64>, &'static str> {
        self.get_a_solution_for_specified_bits_of_bv(bv, high, low)
    }
    fn get_a_solution_for_bool(&mut self, b: &Self::Constraint) -> Result<Option<bool>, &'static str> {
        self.get_a_solution_for_bool(b)
    }
    fn get_possible_solutions_for_bv(&mut self, bv: &Self::Value, n: usize) -> Result<PossibleSolutions<u64>, &'static str> {
        self.get_possible_solutions_for_bv(bv, n)
    }
    fn get_possible_solutions_for_bool(&mut self, b: &Self::Constraint) -> Result<PossibleSolutions<bool>, &'static str> {
        self.get_possible_solutions_for_bool(b)
    }
    fn current_model_to_pretty_string(&self) -> String {
        self.current_model_to_pretty_string()
    }
}

pub struct Z3Backend<'ctx> {
    phantomdata: std::marker::PhantomData<&'ctx ()>,
}

impl<'ctx> Backend<'ctx> for Z3Backend<'ctx> {
    type BV = z3::ast::BV<'ctx>;
    type Bool = z3::ast::Bool<'ctx>;
    type Memory = crate::memory::Memory<'ctx>;
    type Solver = crate::solver::Solver<'ctx>;
    type State = ();
}
