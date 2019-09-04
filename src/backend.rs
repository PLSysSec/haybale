//! Traits which abstract over the backend (BV types, memory implementation,
//! SMT solver) being used.

use boolector::{Btor, BVSolution};
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

/// A `Backend` is just a collection of types which together implement the necessary traits
pub trait Backend {
    type BV: BV;
    type Memory: Memory<Index=Self::BV, Value=Self::BV, BackendState=Self::State>;
    /// Any additional state that the `Backend` needs. This will be stored in the
    /// `state::State` struct.
    ///
    /// Must be `Default`, and the `default()` method will be used to construct
    /// the initial backend state when a blank `state::State` is constructed.
    type State: Clone + Default;
}

/// Trait for things which can act like bitvectors
pub trait BV: Clone + PartialEq + Eq + fmt::Debug {
    fn new(btor: Rc<Btor>, width: u32, name: Option<&str>) -> Self;
    fn from_bool(btor: Rc<Btor>, b: bool) -> Self;
    fn from_i32(btor: Rc<Btor>, i: i32, width: u32) -> Self;
    fn from_u32(btor: Rc<Btor>, u: u32, width: u32) -> Self;
    fn from_i64(btor: Rc<Btor>, i: i64, width: u32) -> Self;
    fn from_u64(btor: Rc<Btor>, u: u64, width: u32) -> Self;
    fn zero(btor: Rc<Btor>, width: u32) -> Self;
    fn one(btor: Rc<Btor>, width: u32) -> Self;
    fn ones(btor: Rc<Btor>, width: u32) -> Self;
    fn from_binary_str(btor: Rc<Btor>, bits: &str) -> Self;
    fn from_dec_str(btor: Rc<Btor>, num: &str, width: u32) -> Self;
    fn from_hex_str(btor: Rc<Btor>, num: &str, width: u32) -> Self;
    fn as_binary_str(&self) -> Option<String>;
    fn as_u64(&self) -> Option<u64>;
    fn as_bool(&self) -> Option<bool>;
    fn get_a_solution(&self) -> BVSolution;
    fn get_btor(&self) -> Rc<Btor>;
    fn get_id(&self) -> i32;
    fn get_width(&self) -> u32;
    fn get_symbol(&self) -> Option<&str>;
    fn set_symbol(&mut self, symbol: Option<&str>);
    fn is_const(&self) -> bool;
    fn has_same_width(&self, other: &Self) -> bool;
    fn assert(&self);
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
    type Index: BV;
    type Value: BV;
    type BackendState;

    /// A new `Memory`, whose contents at all addresses are completely uninitialized (unconstrained)
    fn new_uninitialized(btor: Rc<Btor>, backend_state: Rc<RefCell<Self::BackendState>>) -> Self;

    /// A new `Memory`, whose contents at all addresses are initialized to be `0`
    fn new_zero_initialized(btor: Rc<Btor>, backend_state: Rc<RefCell<Self::BackendState>>) -> Self;

    /// Read any number (>0) of bits of memory, at any alignment.
    /// Returned `BV` will have size `bits`.
    fn read(&self, index: &Self::Index, bits: u32) -> Self::Value;

    /// Write any number (>0) of bits of memory, at any alignment.
    fn write(&mut self, index: &Self::Index, value: Self::Value);
}

/// The prototypical `BV` and `Memory` implementations:
///   `boolector::BV` and `crate::memory::Memory`

impl BV for boolector::BV {
    fn new(btor: Rc<Btor>, width: u32, name: Option<&str>) -> Self {
        boolector::BV::new(btor, width, name)
    }
    fn from_bool(btor: Rc<Btor>, b: bool) -> Self {
        boolector::BV::from_bool(btor, b)
    }
    fn from_i32(btor: Rc<Btor>, i: i32, width: u32) -> Self {
        boolector::BV::from_i32(btor, i, width)
    }
    fn from_u32(btor: Rc<Btor>, u: u32, width: u32) -> Self {
        boolector::BV::from_u32(btor, u, width)
    }
    fn from_i64(btor: Rc<Btor>, i: i64, width: u32) -> Self {
        boolector::BV::from_i64(btor, i, width)
    }
    fn from_u64(btor: Rc<Btor>, u: u64, width: u32) -> Self {
        boolector::BV::from_u64(btor, u, width)
    }
    fn zero(btor: Rc<Btor>, width: u32) -> Self {
        boolector::BV::zero(btor, width)
    }
    fn one(btor: Rc<Btor>, width: u32) -> Self {
        boolector::BV::one(btor, width)
    }
    fn ones(btor: Rc<Btor>, width: u32) -> Self {
        boolector::BV::ones(btor, width)
    }
    fn from_binary_str(btor: Rc<Btor>, bits: &str) -> Self {
        boolector::BV::from_binary_str(btor, bits)
    }
    fn from_dec_str(btor: Rc<Btor>, num: &str, width: u32) -> Self {
        boolector::BV::from_dec_str(btor, num, width)
    }
    fn from_hex_str(btor: Rc<Btor>, num: &str, width: u32) -> Self {
        boolector::BV::from_hex_str(btor, num, width)
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
    fn get_a_solution(&self) -> BVSolution {
        self.get_a_solution()
    }
    fn get_btor(&self) -> Rc<Btor> {
        self.get_btor()
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
    fn assert(&self) {
        self.assert()
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
    type Index = boolector::BV;
    type Value = boolector::BV;
    type BackendState = ();

    fn new_uninitialized(btor: Rc<Btor>, _backend_state: Rc<RefCell<Self::BackendState>>) -> Self {
        crate::memory::Memory::new_uninitialized(btor)
    }
    fn new_zero_initialized(btor: Rc<Btor>, _backend_state: Rc<RefCell<Self::BackendState>>) -> Self {
        crate::memory::Memory::new_zero_initialized(btor)
    }
    fn read(&self, index: &Self::Index, bits: u32) -> Self::Value {
        self.read(index, bits)
    }
    fn write(&mut self, index: &Self::Index, value: Self::Value) {
        self.write(index, value)
    }
}

pub struct BtorBackend {}

impl Backend for BtorBackend {
    type BV = boolector::BV;
    type Memory = crate::memory::Memory;
    type State = ();
}
