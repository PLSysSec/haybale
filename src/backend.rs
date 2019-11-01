//! Traits which abstract over the backend (BV types, memory implementation,
//! SMT solver) being used.

use boolector::{Btor, BVSolution};
use boolector::option::BtorOption;
use crate::error::Result;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

/// A `Backend` is just a collection of types which together implement the necessary traits
pub trait Backend: Clone {
    type SolverRef: SolverRef<BV=Self::BV>;
    type BV: BV<SolverRef=Self::SolverRef>;
    type Memory: Memory<SolverRef=Self::SolverRef, Index=Self::BV, Value=Self::BV>;
}

/// Trait for something which acts as a reference to a `boolector::Btor` (and
/// possibly may carry other information as well).
///
/// `default()` should initialize a fresh solver instance and return a
/// `SolverRef` to it.
///
/// The [`BtorRef`](struct.BtorRef.html) in this module provides a simple
/// implementation of `SolverRef` which is under-the-hood just an `Rc<Btor>`.
pub trait SolverRef: Default + Clone + Deref<Target=Btor> {
    type BV: BV<SolverRef=Self>;
    type Array;

    /// As opposed to `clone()` which merely clones the reference, this function
    /// produces a deep copy of the underlying solver instance
    fn duplicate(&self) -> Self;

    /// Given a `BV` originally created for any `SolverRef`, get the
    /// corresponding `BV` in this `SolverRef`. This is only guaranteed to work
    /// if the `BV` was created before the relevant `SolverRef::duplicate()` was
    /// called; that is, it is intended to be used to find the copied version of
    /// a given `BV` in the new `SolverRef`.
    ///
    /// It's also fine to call this with a `BV` created for this `SolverRef`
    /// itself, in which case you'll just get back `Some(bv.clone())`.
    fn match_bv(&self, bv: &Self::BV) -> Option<Self::BV>;

    /// Given an `Array` originally created for any `SolverRef`, get the
    /// corresponding `Array` in this `SolverRef`. This is only guaranteed to
    /// work if the `Array` was created before the relevant
    /// `SolverRef::duplicate()` was called; that is, it is intended to be used
    /// to find the copied version of a given `Array` in the new `SolverRef`.
    ///
    /// It's also fine to call this with an `Array` created for this `SolverRef`
    /// itself, in which case you'll just get back `Some(array.clone())`.
    fn match_array(&self, array: &Self::Array) -> Option<Self::Array>;
}

/// A wrapper around `Rc<Btor>` which implements `SolverRef`
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BtorRef( Rc<Btor> );

/// This `Default` implementation ensures that specific options we need are set
/// on the `Btor` instance. No other initialization is required.
//
// Note: We used to set model generation here, but now we toggle it so it's only
// on when needed (profiling shows that a sat check with model gen enabled is
// much, much more expensive than a sat check without model gen enabled, at
// least for our frequent incremental sat checks)
impl Default for BtorRef {
    fn default() -> Self {
        let btor = Btor::new();
        btor.set_opt(BtorOption::Incremental(true));
        Self(Rc::new(btor))
    }
}

impl SolverRef for BtorRef {
    type BV = boolector::BV<Rc<Btor>>;
    type Array = boolector::Array<Rc<Btor>>;

    fn duplicate(&self) -> Self {
        Self(Rc::new(self.0.duplicate()))
    }

    fn match_bv(&self, bv: &boolector::BV<Rc<Btor>>) -> Option<boolector::BV<Rc<Btor>>> {
        Btor::get_matching_bv(self.clone().into(), bv)
    }

    fn match_array(&self, array: &boolector::Array<Rc<Btor>>) -> Option<boolector::Array<Rc<Btor>>> {
        Btor::get_matching_array(self.clone().into(), array)
    }
}

impl Deref for BtorRef {
    type Target = Btor;

    fn deref(&self) -> &boolector::Btor {
        &self.0
    }
}

impl From<Rc<Btor>> for BtorRef {
    fn from(rc: Rc<Btor>) -> BtorRef {
        BtorRef(rc)
    }
}

impl From<BtorRef> for Rc<Btor> {
    fn from(btor: BtorRef) -> Rc<Btor> {
        btor.0
    }
}

/// Trait for things which can act like bitvectors
pub trait BV: Clone + PartialEq + Eq + fmt::Debug {
    type SolverRef: SolverRef<BV=Self>;

    fn new(solver: Self::SolverRef, width: u32, name: Option<&str>) -> Self;
    fn from_bool(solver: Self::SolverRef, b: bool) -> Self;
    fn from_i32(solver: Self::SolverRef, i: i32, width: u32) -> Self;
    fn from_u32(solver: Self::SolverRef, u: u32, width: u32) -> Self;
    fn from_i64(solver: Self::SolverRef, i: i64, width: u32) -> Self;
    fn from_u64(solver: Self::SolverRef, u: u64, width: u32) -> Self;
    fn zero(solver: Self::SolverRef, width: u32) -> Self;
    fn one(solver: Self::SolverRef, width: u32) -> Self;
    fn ones(solver: Self::SolverRef, width: u32) -> Self;
    fn from_binary_str(solver: Self::SolverRef, bits: &str) -> Self;
    fn from_dec_str(solver: Self::SolverRef, num: &str, width: u32) -> Self;
    fn from_hex_str(solver: Self::SolverRef, num: &str, width: u32) -> Self;
    fn as_binary_str(&self) -> Option<String>;
    fn as_u64(&self) -> Option<u64>;
    fn as_bool(&self) -> Option<bool>;
    fn get_a_solution(&self) -> Result<BVSolution>;
    fn get_id(&self) -> i32;
    fn get_width(&self) -> u32;
    fn get_symbol(&self) -> Option<&str>;
    fn set_symbol(&mut self, symbol: Option<&str>);
    fn is_const(&self) -> bool;
    fn has_same_width(&self, other: &Self) -> bool;
    fn assert(&self) -> Result<()>;
    fn is_failed_assumption(&self) -> bool;
    fn _eq(&self, other: &Self) -> Self;
    fn _ne(&self, other: &Self) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn udiv(&self, other: &Self) -> Self;
    fn sdiv(&self, other: &Self) -> Self;
    fn urem(&self, other: &Self) -> Self;
    fn srem(&self, other: &Self) -> Self;
    fn smod(&self, other: &Self) -> Self;
    fn inc(&self) -> Self;
    fn dec(&self) -> Self;
    fn neg(&self) -> Self;
    fn uaddo(&self, other: &Self) -> Self;
    fn saddo(&self, other: &Self) -> Self;
    fn usubo(&self, other: &Self) -> Self;
    fn ssubo(&self, other: &Self) -> Self;
    fn umulo(&self, other: &Self) -> Self;
    fn smulo(&self, other: &Self) -> Self;
    fn sdivo(&self, other: &Self) -> Self;
    fn not(&self) -> Self;
    fn and(&self, other: &Self) -> Self;
    fn or(&self, other: &Self) -> Self;
    fn xor(&self, other: &Self) -> Self;
    fn nand(&self, other: &Self) -> Self;
    fn nor(&self, other: &Self) -> Self;
    fn xnor(&self, other: &Self) -> Self;
    fn sll(&self, other: &Self) -> Self;
    fn srl(&self, other: &Self) -> Self;
    fn sra(&self, other: &Self) -> Self;
    fn rol(&self, other: &Self) -> Self;
    fn ror(&self, other: &Self) -> Self;
    fn redand(&self) -> Self;
    fn redor(&self) -> Self;
    fn redxor(&self) -> Self;
    fn ugt(&self, other: &Self) -> Self;
    fn ugte(&self, other: &Self) -> Self;
    fn sgt(&self, other: &Self) -> Self;
    fn sgte(&self, other: &Self) -> Self;
    fn ult(&self, other: &Self) -> Self;
    fn ulte(&self, other: &Self) -> Self;
    fn slt(&self, other: &Self) -> Self;
    fn slte(&self, other: &Self) -> Self;
    fn zext(&self, i: u32) -> Self;
    fn sext(&self, i: u32) -> Self;
    fn slice(&self, high: u32, low: u32) -> Self;
    fn concat(&self, other: &Self) -> Self;
    fn repeat(&self, n: u32) -> Self;
    fn iff(&self, other: &Self) -> Self;
    fn implies(&self, other: &Self) -> Self;
    fn cond_bv(&self, truebv: &Self, falsebv: &Self) -> Self;
}

/// Trait for things which can act like 'memories', that is, maps from bitvector (addresses) to bitvector (values)
pub trait Memory : Clone + PartialEq + Eq {
    type SolverRef: SolverRef;
    type Index: BV;
    type Value: BV;

    /// A new `Memory`, whose contents at all addresses are completely uninitialized (unconstrained)
    fn new_uninitialized(solver: Self::SolverRef, name: Option<&str>) -> Self;

    /// A new `Memory`, whose contents at all addresses are initialized to be `0`
    fn new_zero_initialized(solver: Self::SolverRef, name: Option<&str>) -> Self;

    /// Read any number (>0) of bits of memory, at any alignment.
    /// Returned `BV` will have size `bits`.
    fn read(&self, index: &Self::Index, bits: u32) -> Result<Self::Value>;

    /// Write any number (>0) of bits of memory, at any alignment.
    fn write(&mut self, index: &Self::Index, value: Self::Value) -> Result<()>;

    /// Adapt the `Memory` to a new solver instance.
    ///
    /// The new solver instance should have been created (possibly transitively)
    /// via `SolverRef::duplicate()` from the `SolverRef` this `Memory` was
    /// originally created with (or most recently changed to). Further, no new
    /// variables should have been added since the call to
    /// `SolverRef::duplicate()`.
    fn change_solver(&mut self, new_solver: Self::SolverRef);
}

/// Some prototypical `BV` and `Memory` implementations:
///   `boolector::BV<Rc<Btor>>`, `crate::memory::Memory`, and `crate::simple_memory::Memory`

impl BV for boolector::BV<Rc<Btor>> {
    type SolverRef = BtorRef;

    fn new(btor: BtorRef, width: u32, name: Option<&str>) -> Self {
        boolector::BV::new(btor.into(), width, name)
    }
    fn from_bool(btor: BtorRef, b: bool) -> Self {
        boolector::BV::from_bool(btor.into(), b)
    }
    fn from_i32(btor: BtorRef, i: i32, width: u32) -> Self {
        boolector::BV::from_i32(btor.into(), i, width)
    }
    fn from_u32(btor: BtorRef, u: u32, width: u32) -> Self {
        boolector::BV::from_u32(btor.into(), u, width)
    }
    fn from_i64(btor: BtorRef, i: i64, width: u32) -> Self {
        boolector::BV::from_i64(btor.into(), i, width)
    }
    fn from_u64(btor: BtorRef, u: u64, width: u32) -> Self {
        boolector::BV::from_u64(btor.into(), u, width)
    }
    fn zero(btor: BtorRef, width: u32) -> Self {
        boolector::BV::zero(btor.into(), width)
    }
    fn one(btor: BtorRef, width: u32) -> Self {
        boolector::BV::one(btor.into(), width)
    }
    fn ones(btor: BtorRef, width: u32) -> Self {
        boolector::BV::ones(btor.into(), width)
    }
    fn from_binary_str(btor: BtorRef, bits: &str) -> Self {
        boolector::BV::from_binary_str(btor.into(), bits)
    }
    fn from_dec_str(btor: BtorRef, num: &str, width: u32) -> Self {
        boolector::BV::from_dec_str(btor.into(), num, width)
    }
    fn from_hex_str(btor: BtorRef, num: &str, width: u32) -> Self {
        boolector::BV::from_hex_str(btor.into(), num, width)
    }
    fn as_binary_str(&self) -> Option<String> {
        self.as_binary_str()
    }
    fn as_u64(&self) -> Option<u64> {
        self.as_u64()
    }
    fn as_bool(&self) -> Option<bool> {
        self.as_bool()
    }
    fn get_a_solution(&self) -> Result<BVSolution> {
        Ok(self.get_a_solution())
    }
    fn get_id(&self) -> i32 {
        self.get_id()
    }
    fn get_width(&self) -> u32 {
        self.get_width()
    }
    fn get_symbol(&self) -> Option<&str> {
        self.get_symbol()
    }
    fn set_symbol(&mut self, symbol: Option<&str>) {
        self.set_symbol(symbol)
    }
    fn is_const(&self) -> bool {
        self.is_const()
    }
    fn has_same_width(&self, other: &Self) -> bool {
        self.has_same_width(other)
    }
    fn assert(&self) -> Result<()> {
        self.assert();
        Ok(())
    }
    fn is_failed_assumption(&self) -> bool {
        self.is_failed_assumption()
    }
    fn _eq(&self, other: &Self) -> Self {
        self._eq(other)
    }
    fn _ne(&self, other: &Self) -> Self {
        self._ne(other)
    }
    fn add(&self, other: &Self) -> Self {
        self.add(other)
    }
    fn sub(&self, other: &Self) -> Self {
        self.sub(other)
    }
    fn mul(&self, other: &Self) -> Self {
        self.mul(other)
    }
    fn udiv(&self, other: &Self) -> Self {
        self.udiv(other)
    }
    fn sdiv(&self, other: &Self) -> Self {
        self.sdiv(other)
    }
    fn urem(&self, other: &Self) -> Self {
        self.urem(other)
    }
    fn srem(&self, other: &Self) -> Self {
        self.srem(other)
    }
    fn smod(&self, other: &Self) -> Self {
        self.smod(other)
    }
    fn inc(&self) -> Self {
        self.inc()
    }
    fn dec(&self) -> Self {
        self.dec()
    }
    fn neg(&self) -> Self {
        self.neg()
    }
    fn uaddo(&self, other: &Self) -> Self {
        self.uaddo(other)
    }
    fn saddo(&self, other: &Self) -> Self {
        self.saddo(other)
    }
    fn usubo(&self, other: &Self) -> Self {
        self.usubo(other)
    }
    fn ssubo(&self, other: &Self) -> Self {
        self.ssubo(other)
    }
    fn umulo(&self, other: &Self) -> Self {
        self.umulo(other)
    }
    fn smulo(&self, other: &Self) -> Self {
        self.smulo(other)
    }
    fn sdivo(&self, other: &Self) -> Self {
        self.sdivo(other)
    }
    fn not(&self) -> Self {
        self.not()
    }
    fn and(&self, other: &Self) -> Self {
        self.and(other)
    }
    fn or(&self, other: &Self) -> Self {
        self.or(other)
    }
    fn xor(&self, other: &Self) -> Self {
        self.xor(other)
    }
    fn nand(&self, other: &Self) -> Self {
        self.nand(other)
    }
    fn nor(&self, other: &Self) -> Self {
        self.nor(other)
    }
    fn xnor(&self, other: &Self) -> Self {
        self.xnor(other)
    }
    fn sll(&self, other: &Self) -> Self {
        self.sll(other)
    }
    fn srl(&self, other: &Self) -> Self {
        self.srl(other)
    }
    fn sra(&self, other: &Self) -> Self {
        self.sra(other)
    }
    fn rol(&self, other: &Self) -> Self {
        self.rol(other)
    }
    fn ror(&self, other: &Self) -> Self {
        self.ror(other)
    }
    fn redand(&self) -> Self {
        self.redand()
    }
    fn redor(&self) -> Self {
        self.redor()
    }
    fn redxor(&self) -> Self {
        self.redxor()
    }
    fn ugt(&self, other: &Self) -> Self {
        self.ugt(other)
    }
    fn ugte(&self, other: &Self) -> Self {
        self.ugte(other)
    }
    fn sgt(&self, other: &Self) -> Self {
        self.sgt(other)
    }
    fn sgte(&self, other: &Self) -> Self {
        self.sgte(other)
    }
    fn ult(&self, other: &Self) -> Self {
        self.ult(other)
    }
    fn ulte(&self, other: &Self) -> Self {
        self.ulte(other)
    }
    fn slt(&self, other: &Self) -> Self {
        self.slt(other)
    }
    fn slte(&self, other: &Self) -> Self {
        self.slte(other)
    }
    fn zext(&self, i: u32) -> Self {
        self.uext(i)
    }
    fn sext(&self, i: u32) -> Self {
        self.sext(i)
    }
    fn slice(&self, high: u32, low: u32) -> Self {
        self.slice(high, low)
    }
    fn concat(&self, other: &Self) -> Self {
        self.concat(other)
    }
    fn repeat(&self, n: u32) -> Self {
        self.repeat(n)
    }
    fn iff(&self, other: &Self) -> Self {
        self.iff(other)
    }
    fn implies(&self, other: &Self) -> Self {
        self.implies(other)
    }
    fn cond_bv(&self, truebv: &Self, falsebv: &Self) -> Self {
        self.cond_bv(truebv, falsebv)
    }
}

impl Memory for crate::memory::Memory {
    type SolverRef = BtorRef;
    type Index = boolector::BV<Rc<Btor>>;
    type Value = boolector::BV<Rc<Btor>>;

    fn new_uninitialized(btor: BtorRef, name: Option<&str>) -> Self {
        crate::memory::Memory::new_uninitialized(btor, name)
    }
    fn new_zero_initialized(btor: BtorRef, name: Option<&str>) -> Self {
        crate::memory::Memory::new_zero_initialized(btor, name)
    }
    fn read(&self, index: &Self::Index, bits: u32) -> Result<Self::Value> {
        Ok(self.read(index, bits))
    }
    fn write(&mut self, index: &Self::Index, value: Self::Value) -> Result<()> {
        self.write(index, value);
        Ok(())
    }
    fn change_solver(&mut self, new_btor: BtorRef) {
        self.change_solver(new_btor)
    }
}

impl Memory for crate::simple_memory::Memory {
    type SolverRef = BtorRef;
    type Index = boolector::BV<Rc<Btor>>;
    type Value = boolector::BV<Rc<Btor>>;

    fn new_uninitialized(btor: BtorRef, name: Option<&str>) -> Self {
        crate::simple_memory::Memory::new_uninitialized(btor, name)
    }
    fn new_zero_initialized(btor: BtorRef, name: Option<&str>) -> Self {
        crate::simple_memory::Memory::new_zero_initialized(btor, name)
    }
    fn read(&self, index: &Self::Index, bits: u32) -> Result<Self::Value> {
        Ok(self.read(index, bits))
    }
    fn write(&mut self, index: &Self::Index, value: Self::Value) -> Result<()> {
        self.write(index, value);
        Ok(())
    }
    fn change_solver(&mut self, new_btor: BtorRef) {
        self.change_solver(new_btor)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BtorBackend {}

impl Backend for BtorBackend {
    type SolverRef = BtorRef;
    type BV = boolector::BV<Rc<Btor>>;
    type Memory = crate::memory::Memory;
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SimpleMemoryBackend {}

impl Backend for SimpleMemoryBackend {
    type SolverRef = BtorRef;
    type BV = boolector::BV<Rc<Btor>>;
    type Memory = crate::simple_memory::Memory;
}
