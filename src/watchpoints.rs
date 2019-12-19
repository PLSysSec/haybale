//! Structures for defining memory watchpoints

use std::fmt;

/// A `Watchpoint` describes a segment of memory to watch.
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Watchpoint {
    /// Lower bound of the memory segment to watch (inclusive).
    low: u64,
    /// Upper bound of the memory segment to watch (inclusive).
    high: u64,
}

impl Watchpoint {
    /// A memory watchpoint for the `bytes` bytes of memory at the given constant
    /// memory address.
    pub fn new(addr: u64, bytes: u64) -> Self {
        if bytes == 0 {
            panic!("Watchpoint::new: `bytes` cannot be 0");
        }
        Self {
            low: addr,
            high: addr + bytes - 1,
        }
    }

    /// Get the lower bound of the watched memory segment (inclusive)
    pub fn lower(&self) -> u64 {
        self.low
    }

    /// Get the upper bound of the watched memory segment (inclusive)
    pub fn upper(&self) -> u64 {
        self.high
    }
}

impl fmt::Display for Watchpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:x}, {:x}]", self.low, self.high)
    }
}
