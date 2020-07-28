use crate::cell_memory::Memory;
use log::{debug, warn};
use std::collections::HashMap;

/// An extremely simple bump-allocator which never frees
#[derive(Clone)]
pub struct Alloc {
    /// Pointer to available, unallocated memory
    cursor: u64,

    /// Map from allocation address to its size in bits
    sizes: HashMap<u64, u64>,
}

impl Alloc {
    pub const ALLOC_START: u64 = 0x1000_0000; // we allocate from this address upwards

    pub fn new() -> Self {
        Self {
            cursor: Self::ALLOC_START,
            sizes: HashMap::new(),
        }
    }

    /// Allocate the specified number of bits, returning a pointer to the allocated object.
    // Internal invariants:
    //   - for sizes <= cell size, allocation never crosses a cell boundary
    //   - for sizes > cell size, allocation always starts at a cell boundary
    pub fn alloc(&mut self, bits: impl Into<u64>) -> u64 {
        let bits: u64 = bits.into();
        if bits == 0 {
            warn!("An allocation of 0 bits was requested");
        }
        let bits_in_byte: u64 = Memory::BITS_IN_BYTE.into();
        let cell_bytes: u64 = Memory::CELL_BYTES.into();
        let bytes = {
            let mut bytes = bits / bits_in_byte;
            if bits % bits_in_byte != 0 {
                bytes += 1; // round up to nearest byte
            }
            bytes
        };
        let current_offset_bytes = self.cursor % cell_bytes;
        let bytes_remaining_in_cell = cell_bytes - current_offset_bytes;
        if bytes > bytes_remaining_in_cell {
            self.cursor += bytes_remaining_in_cell;
            assert_eq!(self.cursor % cell_bytes, 0);
        }
        let rval = self.cursor;
        self.cursor += bytes;
        self.sizes.insert(rval, bits);
        debug!("Allocated {} bits at 0x{:x}", bits, rval);
        rval
    }

    /// Get the size, in bits, of the allocation at the given address, or `None`
    /// if that address is not the result of an `alloc()`.
    pub fn get_allocation_size(&self, addr: impl Into<u64>) -> Option<u64> {
        self.sizes.get(&addr.into()).copied()
    }
}
