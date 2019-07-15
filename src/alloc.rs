use crate::memory::Memory;

/// An extremely simple bump-allocator which never frees
pub struct Alloc {
    cursor: u64,
}

impl Alloc {
    pub const ALLOC_START: u64 = 0x1000_0000;  // we allocate from this address upwards

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
        let bits_in_byte: u64 = Memory::BITS_IN_BYTE.into();
        let cell_bytes: u64 = Memory::CELL_BYTES.into();
        if bits % bits_in_byte != 0 {
            unimplemented!("Alloc for {} bits, which is not a multiple of {}", bits, Memory::BITS_IN_BYTE);
        }
        let bytes = bits / bits_in_byte;
        let current_offset_bytes = self.cursor % cell_bytes;
        let bytes_remaining_in_cell = cell_bytes - current_offset_bytes;
        if bytes > bytes_remaining_in_cell {
            self.cursor += bytes_remaining_in_cell;
            assert_eq!(self.cursor % cell_bytes, 0);
        }
        let rval = self.cursor;
        self.cursor += bytes;
        rval
    }
}
