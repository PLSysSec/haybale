use crate::memory::Memory;

/// An extremely simple bump-allocator which never frees
pub struct Alloc {
    cursor: u64,
}

impl Alloc {
    pub const ALLOC_START: u64 = 0x10000000;  // we allocate from this address upwards

    pub fn new() -> Self {
        Self {
            cursor: Self::ALLOC_START,
        }
    }

    /// Allocate the specified number of bits, returning a pointer to the allocated object.
    // Internal invariants:
    //   - for sizes <= cell size, allocation never crosses a cell boundary
    //   - for sizes > cell size, allocation always starts at a cell boundary
    pub fn alloc(&mut self, bits: impl Into<u64>) -> u64 {
        let bits: u64 = bits.into();
        if bits % Memory::BITS_IN_BYTE as u64 != 0 {
            unimplemented!("Alloc for {} bits, which is not a multiple of {}", bits, Memory::BITS_IN_BYTE);
        }
        let bytes = bits / Memory::BITS_IN_BYTE as u64;
        let current_offset_bytes = self.cursor % Memory::CELL_BYTES as u64;
        let bytes_remaining_in_cell = Memory::CELL_BYTES as u64 - current_offset_bytes;
        if bytes > bytes_remaining_in_cell {
            self.cursor += bytes_remaining_in_cell;
            assert_eq!(self.cursor % Memory::CELL_BYTES as u64, 0);
        }
        let rval = self.cursor;
        self.cursor += bytes;
        rval
    }
}
