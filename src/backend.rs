use std::convert::TryInto;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

/// A `Backend` is just a collection of types which together implement the necessary traits
pub trait Backend<'ctx> : 'ctx {
    type BV: BV<'ctx, AssociatedBool = Self::Bool> + 'ctx;
    type Bool: Bool<'ctx, AssociatedBV = Self::BV> + 'ctx;
    type Array: Array<'ctx, Index=Self::BV, Value=Self::BV, BackendState=Self::State> + 'ctx;
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

    /*
    /// for the purposes of `assert()`ing in a solver
    fn as_assertion(&self) -> &z3::ast::Bool<'ctx>;
    */
}

/// Trait for things which can act like arrays of bitvectors, indexed by bitvectors
pub trait Array<'ctx> : Clone + PartialEq + Eq {
    type Index: BV<'ctx>;
    type Value: BV<'ctx>;
    type BackendState;

    fn new(ctx: &'ctx z3::Context, backend_state: Rc<RefCell<Self::BackendState>>, name: impl Into<z3::Symbol>, indexbits: u32, valuebits: u32) -> Self;
    fn select(&self, index: Self::Index) -> Self::Value;
    fn store(&self, index: Self::Index, value: Self::Value) -> Self;
    fn simplify(&self) -> Self {
        // default implementation, many implementors will do better
        self.clone()
    }
}

pub trait Solver<'ctx> {
    type Constraint: Bool<'ctx>;
    type Value: BV<'ctx>;
    type BackendState;

    fn new(ctx: &'ctx z3::Context, backend_state: Rc<RefCell<Self::BackendState>>) -> Self;
    fn assert(&mut self, constraint: &Self::Constraint);
    fn check(&mut self) -> bool;
    fn check_with_extra_constraints<'a>(&'a mut self, constraints: impl Iterator<Item = &'a Self::Constraint>) -> bool where Self::Constraint: 'a;
    fn push(&mut self);
    fn pop(&mut self, n: usize);
    fn get_a_solution_for_bv(&mut self, bv: &Self::Value) -> Option<u64>;
    fn get_a_solution_for_specified_bits_of_bv(&mut self, bv: &Self::Value, high: u32, low: u32) -> Option<u64>;
    fn get_a_solution_for_bool(&mut self, b: &Self::Constraint) -> Option<bool>;
    fn current_model_to_pretty_string(&self) -> String;
}

/// The prototypical `BV`, `Bool`, `Array`, and `Solver` implementations:
///   `z3::ast::BV`, `z3::ast::Bool`, `z3::ast::Array`, and `crate::solver::Solver`

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

impl<'ctx> Array<'ctx> for z3::ast::Array<'ctx> {
    type Index = z3::ast::BV<'ctx>;
    type Value = z3::ast::BV<'ctx>;
    type BackendState = ();

    fn new(ctx: &'ctx z3::Context, _backend_state: Rc<RefCell<Self::BackendState>>, name: impl Into<z3::Symbol>, indexbits: u32, valuebits: u32) -> Self {
        Self::new_const(ctx, name, &z3::Sort::bitvector(ctx, indexbits), &z3::Sort::bitvector(ctx, valuebits))
    }
    fn select(&self, index: Self::Index) -> Self::Value {
        self.select(&index.into()).try_into().unwrap()
    }
    fn store(&self, index: Self::Index, value: Self::Value) -> Self {
        self.store(&index.into(), &value.into())
    }
    fn simplify(&self) -> Self {
        z3::ast::Ast::simplify(self)
    }
}

impl<'ctx> Solver<'ctx> for crate::solver::Solver<'ctx> {
    type Constraint = z3::ast::Bool<'ctx>;
    type Value = z3::ast::BV<'ctx>;
    type BackendState = ();

    fn new(ctx: &'ctx z3::Context, _backend_state: Rc<RefCell<Self::BackendState>>) -> Self {
        crate::solver::Solver::new(ctx)
    }
    fn assert(&mut self, constraint: &Self::Constraint) {
        self.assert(constraint)
    }
    fn check(&mut self) -> bool {
        self.check()
    }
    fn check_with_extra_constraints<'a>(&'a mut self, constraints: impl Iterator<Item = &'a Self::Constraint>) -> bool {
        self.check_with_extra_constraints(constraints)
    }
    fn push(&mut self) {
        self.push()
    }
    fn pop(&mut self, n: usize) {
        self.pop(n)
    }
    fn get_a_solution_for_bv(&mut self, bv: &Self::Value) -> Option<u64> {
        self.get_a_solution_for_bv(bv)
    }
    fn get_a_solution_for_specified_bits_of_bv(&mut self, bv: &Self::Value, high: u32, low: u32) -> Option<u64> {
        self.get_a_solution_for_specified_bits_of_bv(bv, high, low)
    }
    fn get_a_solution_for_bool(&mut self, b: &Self::Constraint) -> Option<bool> {
        self.get_a_solution_for_bool(b)
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
    type Array = z3::ast::Array<'ctx>;
    type Solver = crate::solver::Solver<'ctx>;
    type State = ();
}
