//! Simple implementation of a `Memory` based on a Boolector array and 8-bit cells.
//! Like the more complicated `Memory` in `cell_memory.rs`, this handles fully
//! general read and write operations: arbitrary addresses, sizes, and
//! alignments.
//! Despite being simpler, it seems to outperform the `Memory` in `cell_memory.rs`
//! in many situations.

use crate::backend::SolverRef;
use crate::error::*;
use crate::solver_utils::bvs_can_be_equal;
use boolector::Btor;
use log::debug;
use reduce::Reduce;
use std::rc::Rc;

type BV = boolector::BV<Rc<Btor>>;
type Array = boolector::Array<Rc<Btor>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Memory {
    btor: Rc<Btor>,
    mem: Array,
    /// e.g. `64` for a `Memory` which uses 64-bit addresses
    addr_bits: u32,
    name: String,
    null_detection: bool,
}

impl Memory {
    pub const CELL_BITS: u32 = 8; // memory "cells" are 8-bit sized; we will mask if smaller operations are needed
    pub const BITS_IN_BYTE: u32 = 8;
    pub const LOG_BITS_IN_BYTE: u32 = 3; // log base 2 of BITS_IN_BYTE
    pub const CELL_BYTES: u32 = Self::CELL_BITS / Self::BITS_IN_BYTE; // how many bytes in a cell

    /// A new `Memory`, whose contents at all addresses are completely uninitialized (unconstrained)
    ///
    /// `null_detection`: if `true`, all memory accesses will be checked to ensure
    /// their addresses cannot be NULL, throwing `Error::NullPointerDereference`
    /// if NULL is a possible solution for the address
    ///
    /// `name`: a name for this `Memory`, or `None` to use the default name (as of this writing, 'mem')
    ///
    /// `addr_bits`: e.g. `64` for a `Memory` which uses 64-bit addresses
    pub fn new_uninitialized(
        btor: Rc<Btor>,
        null_detection: bool,
        name: Option<&str>,
        addr_bits: u32,
    ) -> Self {
        let default_name = "mem";
        Self {
            mem: Array::new(
                btor.clone(),
                addr_bits,
                Self::CELL_BITS,
                name.or(Some(default_name)),
            ),
            name: name.unwrap_or(default_name).into(),
            null_detection,
            addr_bits,
            btor, // out of order so it can be used above but moved in here
        }
    }

    /// A new `Memory`, whose contents at all addresses are initialized to be `0`
    ///
    /// `null_detection`: if `true`, all memory accesses will be checked to ensure
    /// their addresses cannot be NULL, throwing `Error::NullPointerDereference`
    /// if NULL is a possible solution for the address
    ///
    /// `name`: a name for this `Memory`, or `None` to use the default name (as of this writing, 'mem_initialized')
    ///
    /// `addr_bits`: e.g. `64` for a `Memory` which uses 64-bit addresses
    pub fn new_zero_initialized(
        btor: Rc<Btor>,
        null_detection: bool,
        name: Option<&str>,
        addr_bits: u32,
    ) -> Self {
        let default_name = "mem_initialized";
        Self {
            mem: Array::new_initialized(
                btor.clone(),
                addr_bits,
                Self::CELL_BITS,
                &BV::zero(btor.clone(), Self::CELL_BITS),
            ),
            name: name.unwrap_or(default_name).into(),
            null_detection,
            addr_bits,
            btor, // out of order so it can be used above but moved in here
        }
    }

    /// Get a reference to the `Btor` instance this `Memory` belongs to
    pub fn get_solver(&self) -> Rc<Btor> {
        self.btor.clone()
    }

    /// Adapt the `Memory` to a new `Btor` instance.
    ///
    /// The new `Btor` instance should have been created (possibly transitively)
    /// via `Btor::duplicate()` from the `Btor` this `Memory` was originally
    /// created with (or most recently changed to). Further, no new variables
    /// should have been added since the call to `Btor::duplicate()`.
    pub fn change_solver(&mut self, new_btor: Rc<Btor>) {
        self.mem = new_btor.match_array(&self.mem).unwrap();
        self.btor = new_btor;
    }

    /// Read a byte from the given address.
    fn read_byte(&self, addr: &BV) -> BV {
        assert_eq!(
            addr.get_width(),
            self.addr_bits,
            "Read address has wrong width: expected {} bits but got {} bits",
            self.addr_bits,
            addr.get_width(),
        );
        self.mem.read(addr)
    }

    /// Write a byte to the given address.
    // TODO: to enforce concretization, we could just take a u64 address here
    fn write_byte(&mut self, addr: &BV, val: &BV) {
        assert_eq!(
            addr.get_width(),
            self.addr_bits,
            "Write address has wrong width: expected {} bits but got {} bits",
            self.addr_bits,
            addr.get_width(),
        );
        assert_eq!(
            val.get_width(),
            Self::CELL_BITS,
            "write_byte: expected exactly one byte of data to write"
        );
        self.mem = self.mem.write(addr, val);
    }

    /// Read any number (>0) of bits of memory, at any alignment.
    /// Returned `BV` will have size `bits`.
    pub fn read(&self, addr: &BV, bits: u32) -> Result<BV> {
        debug!("Reading {} bits from {} at {:?}", bits, &self.name, addr);
        let addr_width = addr.get_width();
        assert_eq!(
            addr_width, self.addr_bits,
            "Read address has wrong width: expected {} bits but got {} bits",
            self.addr_bits, addr_width
        );

        if self.null_detection
            && bvs_can_be_equal(&self.btor, addr, &BV::zero(self.btor.clone(), addr_width))?
        {
            return Err(Error::NullPointerDereference);
        }

        let rval = if bits < Self::BITS_IN_BYTE {
            let byte = self.read_byte(&addr);
            byte.slice(bits - 1, 0)
        } else {
            assert_eq!(bits % Self::BITS_IN_BYTE, 0, "Read with size {} bits", bits);
            let bytes = bits / Self::BITS_IN_BYTE;
            assert!(bytes > 0, "Read of length 0");
            (0 .. bytes)
                .map(|byte_num| {
                    let offset_addr = addr.add(&BV::from_u64(
                        self.btor.clone(),
                        u64::from(byte_num),
                        self.addr_bits,
                    ));
                    self.read_byte(&offset_addr)
                })
                .reduce(|a, b| b.concat(&a))
                .unwrap() // because bytes > 0, there must have been at least 1 item in the iterator
        };
        debug!("Value read is {:?}", rval);
        Ok(rval)
    }

    /// Write any number (>0) of bits of memory, at any alignment.
    pub fn write(&mut self, addr: &BV, val: BV) -> Result<()> {
        debug!("Writing {:?} to {} address {:?}", val, &self.name, addr);
        let addr_width = addr.get_width();
        assert_eq!(
            addr_width, self.addr_bits,
            "Write address has wrong width: expected {} bits but got {} bits",
            self.addr_bits, addr_width,
        );

        if self.null_detection
            && bvs_can_be_equal(&self.btor, addr, &BV::zero(self.btor.clone(), addr_width))?
        {
            return Err(Error::NullPointerDereference);
        }

        let write_size = val.get_width();
        let write_data = if write_size < Self::BITS_IN_BYTE {
            // implicitly zero-extend to 8 bits
            val.uext(8 - write_size)
        } else {
            val
        };
        let write_size = write_data.get_width();
        assert_eq!(
            write_size % Self::BITS_IN_BYTE,
            0,
            "Write with size {} bits",
            write_size
        );
        let write_size_bytes = write_size / Self::BITS_IN_BYTE;
        for byte_num in 0 .. write_size_bytes {
            let data_byte = write_data.slice(
                (byte_num + 1) * Self::BITS_IN_BYTE - 1,
                byte_num * Self::BITS_IN_BYTE,
            );
            let offset_addr = addr.add(&BV::from_u64(
                self.btor.clone(),
                u64::from(byte_num),
                addr_width,
            ));
            self.write_byte(&offset_addr, &data_byte);
        }
        Ok(())
    }
}

#[cfg(test)]
/// These tests are adapted directly from those in cell_memory.rs, because the two
/// modules should have exactly the same behavior, potentially with different
/// performance characteristics
mod tests {
    use super::*;
    use crate::error::Result;
    use crate::solver_utils::{self, PossibleSolutions};
    use boolector::option::{BtorOption, ModelGen};
    use boolector::{BVSolution, BV};
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use std::rc::Rc;

    // Basically the `get_a_solution_for_bv()` method from `State`,
    // without requiring that we construct a `State` or depend on the
    // `State` module
    fn get_a_solution(bv: &BV<Rc<Btor>>) -> Result<Option<BVSolution>> {
        let btor = bv.get_btor();
        btor.set_opt(BtorOption::ModelGen(ModelGen::All));
        let solution = if solver_utils::sat(&btor)? {
            Some(bv.get_a_solution())
        } else {
            None
        };
        btor.set_opt(BtorOption::ModelGen(ModelGen::Disabled));
        Ok(solution)
    }

    #[test]
    fn uninitialized() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mem = Memory::new_uninitialized(btor.clone(), true, None, 64);

        let addr = BV::from_u64(btor.clone(), 0x10000, 64);
        let zero = BV::zero(btor.clone(), 8);

        // Read a byte from (uninitialized) memory
        let read_bv = mem.read(&addr, 8)?;

        // Constrain it to be > 0 and check that we're sat (and get a value > 0)
        btor.push(1);
        read_bv.sgt(&zero).assert();
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let read_val = get_a_solution(&read_bv)?
            .expect("Expected a solution")
            .as_u64()
            .unwrap() as i8;
        assert!(read_val > 0);

        // Alternately, constrain it to be < 0 and check that we're sat (and get a value < 0)
        btor.pop(1);
        read_bv.slt(&zero).assert();
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let read_val = get_a_solution(&read_bv)?
            .expect("Expected a solution")
            .as_u64()
            .unwrap() as i8;
        assert!(read_val < 0);

        Ok(())
    }

    #[test]
    fn zero_initialized() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mem = Memory::new_zero_initialized(btor.clone(), true, None, 64);

        let addr = BV::from_u64(btor.clone(), 0x10000, 64);

        // Read a value from (zero-initialized) memory and check that the only possible value is 0
        let read_bv = mem.read(&addr, Memory::CELL_BITS)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(0)))
        );

        Ok(())
    }

    #[test]
    fn read_and_write_to_cell_zero() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut mem = Memory::new_uninitialized(btor.clone(), false, None, 64);

        // Store a byte of data to address 0
        let data_val = 0x7c;
        let data = BV::from_u32(btor.clone(), data_val, Memory::CELL_BITS);
        let zero = BV::zero(btor.clone(), 64);
        mem.write(&zero, data)?;

        // Ensure that we can read it back again
        let read_bv = mem.read(&zero, Memory::CELL_BITS)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(data_val as u64)))
        );

        Ok(())
    }

    #[test]
    fn read_and_write_cell_aligned() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut mem = Memory::new_uninitialized(btor.clone(), true, None, 64);

        // Store a byte of data to a nonzero, but aligned, address
        let data_val = 0xba;
        let data = BV::from_u32(btor.clone(), data_val, Memory::CELL_BITS);
        let aligned = BV::from_u64(btor.clone(), 0x10000, 64);
        mem.write(&aligned, data)?;

        // Ensure that we can read it back again
        let read_bv = mem.read(&aligned, Memory::CELL_BITS)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(data_val as u64)))
        );

        Ok(())
    }

    #[test]
    fn read_and_write_small() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut mem = Memory::new_uninitialized(btor.clone(), true, None, 64);

        // Store 8 bits of data to an aligned address
        let data_val = 0x4F;
        let data = BV::from_u64(btor.clone(), data_val, 8);
        let addr = BV::from_u64(btor.clone(), 0x10000, 64);
        mem.write(&addr, data)?;

        // Ensure that we can read it back again
        let read_bv = mem.read(&addr, 8)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(data_val)))
        );

        Ok(())
    }

    /// Essentially the same as the above test but with 32-bit addresses
    #[test]
    fn read_and_write_small_32bitaddr() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut mem = Memory::new_uninitialized(btor.clone(), true, None, 32);

        // Store 8 bits of data to an aligned address
        let data_val = 0x4F;
        let data = BV::from_u64(btor.clone(), data_val, 8);
        let addr = BV::from_u64(btor.clone(), 0x10000, 32);
        mem.write(&addr, data)?;

        // Ensure that we can read it back again
        let read_bv = mem.read(&addr, 8)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(data_val)))
        );

        Ok(())
    }

    #[test]
    fn read_single_bit() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut mem = Memory::new_uninitialized(btor.clone(), true, None, 64);

        // Store 8 bits of data to an aligned address
        let data_val = 0x55;
        let data = BV::from_u64(btor.clone(), data_val, 8);
        let addr = BV::from_u64(btor.clone(), 0x10000, 64);
        mem.write(&addr, data)?;

        // Ensure that we can read a single bit
        let read_bv = mem.read(&addr, 1)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(1)))
        ); // we should read the least significant bit, which should have value 1

        Ok(())
    }

    #[test]
    fn read_and_write_unaligned() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut mem = Memory::new_uninitialized(btor.clone(), true, None, 64);

        // Store 8 bits of data to offset 1 in a cell
        let data_val = 0x4F;
        let data = BV::from_u64(btor.clone(), data_val, 8);
        let unaligned = BV::from_u64(btor.clone(), 0x10001, 64);
        mem.write(&unaligned, data)?;

        // Ensure that we can read it back again
        let read_bv = mem.read(&unaligned, 8)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(data_val)))
        );

        Ok(())
    }

    #[test]
    fn read_and_write_64_bits() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut mem = Memory::new_uninitialized(btor.clone(), true, None, 64);

        // Store 64 bits of data
        let data_val: u64 = 0x12345678_9abcdef0;
        let data = BV::from_u64(btor.clone(), data_val, 64);
        let addr = BV::from_u64(btor.clone(), 0x10004, 64);
        mem.write(&addr, data)?;

        // Ensure that we can read it back again
        let read_bv = mem.read(&addr, 64)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(data_val)))
        );

        Ok(())
    }

    #[test]
    fn read_and_write_symbolic_addr() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut mem = Memory::new_uninitialized(btor.clone(), false, None, 64);

        // Store 64 bits of data to a symbolic address
        let data_val: u64 = 0x12345678_9abcdef0;
        let data = BV::from_u64(btor.clone(), data_val, 64);
        let addr = BV::new(btor.clone(), 64, Some("symbolic_addr"));
        mem.write(&addr, data)?;

        // Ensure that we can read it back again
        let read_bv = mem.read(&addr, 64)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(data_val)))
        );

        Ok(())
    }

    #[test]
    fn read_and_write_200bits() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut mem = Memory::new_uninitialized(btor.clone(), true, None, 64);

        // Store 200 bits of data to an aligned address
        let data_val_0: u64 = 0x12345678_9abcdef0;
        let data_val_1: u64 = 0x2468ace0_13579bdf;
        let data_val_2: u64 = 0xfedcba98_76543210;
        let data_val_3: u64 = 0xef;
        let write_val = BV::from_u64(btor.clone(), data_val_3, 8)
            .concat(&BV::from_u64(btor.clone(), data_val_2, 64))
            .concat(&BV::from_u64(btor.clone(), data_val_1, 64))
            .concat(&BV::from_u64(btor.clone(), data_val_0, 64));
        assert_eq!(write_val.get_width(), 200);
        let addr = BV::from_u64(btor.clone(), 0x10000, 64);
        mem.write(&addr, write_val)?;

        // Ensure that we can read it back again
        let read_bv = mem.read(&addr, 200)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let read_val_0 = get_a_solution(&read_bv.slice(63, 0))?
            .expect("Expected a solution")
            .as_u64()
            .unwrap();
        assert_eq!(read_val_0, data_val_0);
        let read_val_1 = get_a_solution(&read_bv.slice(127, 64))?
            .expect("Expected a solution")
            .as_u64()
            .unwrap();
        assert_eq!(read_val_1, data_val_1);
        let read_val_2 = get_a_solution(&read_bv.slice(191, 128))?
            .expect("Expected a solution")
            .as_u64()
            .unwrap();
        assert_eq!(read_val_2, data_val_2);
        let read_val_3 = get_a_solution(&read_bv.slice(199, 192))?
            .expect("Expected a solution")
            .as_u64()
            .unwrap();
        assert_eq!(read_val_3, data_val_3);

        Ok(())
    }

    #[test]
    fn read_and_write_200bits_unaligned() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut mem = Memory::new_uninitialized(btor.clone(), true, None, 64);

        // Store 200 bits of data to an unaligned address
        let data_val_0: u64 = 0x12345678_9abcdef0;
        let data_val_1: u64 = 0x2468ace0_13579bdf;
        let data_val_2: u64 = 0xfedcba98_76543210;
        let data_val_3: u64 = 0xef;
        let write_val = BV::from_u64(btor.clone(), data_val_3, 8)
            .concat(&BV::from_u64(btor.clone(), data_val_2, 64))
            .concat(&BV::from_u64(btor.clone(), data_val_1, 64))
            .concat(&BV::from_u64(btor.clone(), data_val_0, 64));
        assert_eq!(write_val.get_width(), 200);
        let addr = BV::from_u64(btor.clone(), 0x10003, 64);
        mem.write(&addr, write_val)?;

        // Ensure that we can read it back again
        let read_bv = mem.read(&addr, 200)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let read_val_0 = get_a_solution(&read_bv.slice(63, 0))?
            .expect("Expected a solution")
            .as_u64()
            .unwrap();
        assert_eq!(read_val_0, data_val_0);
        let read_val_1 = get_a_solution(&read_bv.slice(127, 64))?
            .expect("Expected a solution")
            .as_u64()
            .unwrap();
        assert_eq!(read_val_1, data_val_1);
        let read_val_2 = get_a_solution(&read_bv.slice(191, 128))?
            .expect("Expected a solution")
            .as_u64()
            .unwrap();
        assert_eq!(read_val_2, data_val_2);
        let read_val_3 = get_a_solution(&read_bv.slice(199, 192))?
            .expect("Expected a solution")
            .as_u64()
            .unwrap();
        assert_eq!(read_val_3, data_val_3);

        Ok(())
    }

    #[test]
    fn read_and_write_200bits_symbolic_addr() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut mem = Memory::new_uninitialized(btor.clone(), false, None, 64);

        // Store 200 bits of data to a symbolic address
        let data_val_0: u64 = 0x12345678_9abcdef0;
        let data_val_1: u64 = 0x2468ace0_13579bdf;
        let data_val_2: u64 = 0xfedcba98_76543210;
        let data_val_3: u64 = 0xef;
        let write_val = BV::from_u64(btor.clone(), data_val_3, 8)
            .concat(&BV::from_u64(btor.clone(), data_val_2, 64))
            .concat(&BV::from_u64(btor.clone(), data_val_1, 64))
            .concat(&BV::from_u64(btor.clone(), data_val_0, 64));
        assert_eq!(write_val.get_width(), 200);
        let addr = BV::new(btor.clone(), 64, Some("symbolic_addr"));
        mem.write(&addr, write_val)?;

        // Ensure that we can read it back again
        let read_bv = mem.read(&addr, 200)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let read_val_0 = get_a_solution(&read_bv.slice(63, 0))?
            .expect("Expected a solution")
            .as_u64()
            .unwrap();
        assert_eq!(read_val_0, data_val_0);
        let read_val_1 = get_a_solution(&read_bv.slice(127, 64))?
            .expect("Expected a solution")
            .as_u64()
            .unwrap();
        assert_eq!(read_val_1, data_val_1);
        let read_val_2 = get_a_solution(&read_bv.slice(191, 128))?
            .expect("Expected a solution")
            .as_u64()
            .unwrap();
        assert_eq!(read_val_2, data_val_2);
        let read_val_3 = get_a_solution(&read_bv.slice(199, 192))?
            .expect("Expected a solution")
            .as_u64()
            .unwrap();
        assert_eq!(read_val_3, data_val_3);

        Ok(())
    }

    #[test]
    fn write_twice_read_once() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut mem = Memory::new_uninitialized(btor.clone(), true, None, 64);

        // Store 8 bits of data
        let data_val = 0x4F;
        let data = BV::from_u64(btor.clone(), data_val, 8);
        let addr = BV::from_u64(btor.clone(), 0x10000, 64);
        mem.write(&addr, data)?;

        // Store a different 8 bits of data to the same address
        let data_val = 0x3A;
        let data = BV::from_u64(btor.clone(), data_val, 8);
        mem.write(&addr, data)?;

        // Ensure that we get back the most recent data
        let read_bv = mem.read(&addr, 8)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(data_val)))
        );

        Ok(())
    }

    #[test]
    fn write_different_locations() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut mem = Memory::new_uninitialized(btor.clone(), true, None, 64);

        // Store 32 bits of data
        let data_val = 0x1234_5678;
        let data = BV::from_u64(btor.clone(), data_val, 32);
        let addr = BV::from_u64(btor.clone(), 0x10000, 64);
        mem.write(&addr, data)?;

        // Store a different 32 bits of data to a different location
        let data_val_2 = 0xfedc_ba98;
        let data_2 = BV::from_u64(btor.clone(), data_val_2, 32);
        let addr_2 = BV::from_u64(btor.clone(), 0x10008, 64);
        mem.write(&addr_2, data_2)?;

        // Ensure that we can read them both individually
        let read_bv = mem.read(&addr, 32)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(data_val)))
        );
        let read_bv = mem.read(&addr_2, 32)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(data_val_2)))
        );

        Ok(())
    }

    #[test]
    fn write_adjacent_locations() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut mem = Memory::new_uninitialized(btor.clone(), true, None, 64);

        // Store 32 bits of data
        let data_val = 0x1234_5678;
        let data = BV::from_u64(btor.clone(), data_val, 32);
        let addr = BV::from_u64(btor.clone(), 0x10000, 64);
        mem.write(&addr, data)?;

        // Store a different 32 bits of data adjacent to it
        let data_val_2 = 0xfedc_ba98;
        let data_2 = BV::from_u64(btor.clone(), data_val_2, 32);
        let addr_2 = BV::from_u64(btor.clone(), 0x10004, 64);
        mem.write(&addr_2, data_2)?;

        // Ensure that we can read them both individually
        let read_bv = mem.read(&addr, 32)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(data_val)))
        );
        let read_bv = mem.read(&addr_2, 32)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(data_val_2)))
        );

        Ok(())
    }

    #[test]
    fn write_small_read_big() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut mem = Memory::new_zero_initialized(btor.clone(), true, None, 64);

        // Store 8 bits of data
        let data_val = 0x4F;
        let data = BV::from_u64(btor.clone(), data_val, 8);
        let unaligned = BV::from_u64(btor.clone(), 0x10001, 64);
        mem.write(&unaligned, data)?;

        // Ensure that reading 16 bits starting 8 bits earlier adds zeroed low-order bits
        // (we are little-endian)
        let aligned = BV::from_u64(btor.clone(), 0x10000, 64);
        let read_bv = mem.read(&aligned, 16)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(0x4F00)))
        );

        // Ensure that reading 16 bits starting at the written address adds zeroed high-order bits
        let read_bv = mem.read(&unaligned, 16)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(0x004F)))
        );

        // Ensure that reading elsewhere gives all zeroes
        let garbage_addr_1 = BV::from_u64(btor.clone(), 0x10004, 64);
        let garbage_addr_2 = BV::from_u64(btor.clone(), 0x10008, 64);
        let read_bv_1 = mem.read(&garbage_addr_1, 8)?;
        let read_bv_2 = mem.read(&garbage_addr_2, 8)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps_1 = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv_1, 1)?
            .as_u64_solutions()
            .unwrap();
        let ps_2 = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv_2, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps_1,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(0)))
        );
        assert_eq!(
            ps_2,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(0)))
        );

        Ok(())
    }

    #[test]
    fn write_big_read_small() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut mem = Memory::new_uninitialized(btor.clone(), true, None, 64);

        // Store 32 bits of data
        let data_val = 0x1234_5678;
        let data = BV::from_u64(btor.clone(), data_val, 32);
        let offset_2 = BV::from_u64(btor.clone(), 0x10002, 64);
        mem.write(&offset_2, data)?;

        // Ensure that reading 8 bits from that location gives the low-order byte
        // (we are little-endian)
        let read_bv = mem.read(&offset_2, 8)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(0x78)))
        );

        // Ensure that reading 8 bits from the end of that location gives the high-order byte
        // (we are little-endian)
        let offset_5 = BV::from_u64(btor.clone(), 0x10005, 64);
        let read_bv = mem.read(&offset_5, 8)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(0x12)))
        );

        // Ensure that reading 16 bits from the middle gives the middle two bytes
        let offset_3 = BV::from_u64(btor.clone(), 0x10003, 64);
        let read_bv = mem.read(&offset_3, 16)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(0x3456)))
        );

        Ok(())
    }

    /// Essentially the same as the above test but with 32-bit addresses
    #[test]
    fn write_big_read_small_32bitaddr() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut mem = Memory::new_uninitialized(btor.clone(), true, None, 32);

        // Store 32 bits of data
        let data_val = 0x1234_5678;
        let data = BV::from_u64(btor.clone(), data_val, 32);
        let offset_2 = BV::from_u64(btor.clone(), 0x10002, 32);
        mem.write(&offset_2, data)?;

        // Ensure that reading 8 bits from that location gives the low-order byte
        // (we are little-endian)
        let read_bv = mem.read(&offset_2, 8)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(0x78)))
        );

        // Ensure that reading 8 bits from the end of that location gives the high-order byte
        // (we are little-endian)
        let offset_5 = BV::from_u64(btor.clone(), 0x10005, 32);
        let read_bv = mem.read(&offset_5, 8)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(0x12)))
        );

        // Ensure that reading 16 bits from the middle gives the middle two bytes
        let offset_3 = BV::from_u64(btor.clone(), 0x10003, 32);
        let read_bv = mem.read(&offset_3, 16)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(0x3456)))
        );

        Ok(())
    }

    #[test]
    fn partial_overwrite_aligned() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut mem = Memory::new_uninitialized(btor.clone(), true, None, 64);

        // Write 64 bits
        let data = BV::from_u64(btor.clone(), 0x12345678_12345678, 64);
        let addr = BV::from_u64(btor.clone(), 0x10000, 64);
        mem.write(&addr, data)?;

        // Write over just the first part
        let overwrite_data_val = 0xdcba;
        let overwrite_data = BV::from_u64(btor.clone(), overwrite_data_val, 16);
        mem.write(&addr, overwrite_data)?;

        // Ensure that we can read the smaller overwrite back
        let read_bv = mem.read(&addr, 16)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(overwrite_data_val)))
        );

        // Ensure that reading the whole 64 bits back reflects the partial overwrite
        let read_bv = mem.read(&addr, 64)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(0x12345678_1234dcba)))
        );

        Ok(())
    }

    #[test]
    fn partial_overwrite_unaligned() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut mem = Memory::new_uninitialized(btor.clone(), true, None, 64);

        // Write 64 bits
        let data = BV::from_u64(btor.clone(), 0x12345678_12345678, 64);
        let addr = BV::from_u64(btor.clone(), 0x10000, 64);
        mem.write(&addr, data)?;

        // Write over just part of the middle
        let overwrite_addr = BV::from_u64(btor.clone(), 0x10002, 64);
        let overwrite_data_val = 0xdcba;
        let overwrite_data = BV::from_u64(btor.clone(), overwrite_data_val, 16);
        mem.write(&overwrite_addr, overwrite_data)?;

        // Ensure that we can read the smaller overwrite back
        let read_bv = mem.read(&overwrite_addr, 16)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(overwrite_data_val)))
        );

        // Ensure that reading the whole 64 bits back reflects the partial overwrite
        let read_bv = mem.read(&addr, 64)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(0x12345678_dcba5678)))
        );

        // Now a different partial read with some original data and some overwritten
        let new_addr = BV::from_u64(btor.clone(), 0x10003, 64);
        let read_bv = mem.read(&new_addr, 16)?;
        assert_eq!(solver_utils::sat(&btor), Ok(true));
        let ps = solver_utils::get_possible_solutions_for_bv(btor.clone(), &read_bv, 1)?
            .as_u64_solutions()
            .unwrap();
        assert_eq!(
            ps,
            PossibleSolutions::Exactly(HashSet::from_iter(std::iter::once(0x78dc)))
        );

        Ok(())
    }
}
