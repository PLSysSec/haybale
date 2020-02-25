//! Structures for defining and processing memory watchpoints

use crate::backend::BV;
use crate::error::Result;
use crate::solver_utils;
use std::collections::HashMap;
use std::fmt;
use std::iter::FromIterator;

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

    /// Get the lower bound of the memory segment being watched (inclusive).
    pub fn get_lower_bound(&self) -> u64 {
        self.low
    }

    /// Get the upper bound of the memory segment being watched (inclusive).
    pub fn get_upper_bound(&self) -> u64 {
        self.high
    }
}

impl fmt::Display for Watchpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:#x}, {:#x}]", self.low, self.high)
    }
}

/// Stores information about watchpoints and performs operations with them.
///
/// External users (that is, `haybale` users) probably don't want to use this
/// directly - instead, you're probably looking for the watchpoint-related
/// methods on [`State`](../struct.State.html).
//
// Maps watchpoint name to `Watchpoint` object and a `bool` indicating whether
// that `Watchpoint` is currently enabled.
#[derive(Clone, Default)]
pub struct Watchpoints(HashMap<String, (Watchpoint, bool)>);

impl FromIterator<(String, Watchpoint)> for Watchpoints {
    fn from_iter<I: IntoIterator<Item = (String, Watchpoint)>>(iter: I) -> Self {
        Self(iter.into_iter().map(|(name, w)| (name, (w, true))).collect())
    }
}

impl Watchpoints {
    /// Construct a new `Watchpoints` instance with no watchpoints.
    ///
    /// To construct a new `Watchpoints` instance that contains some initial
    /// watchpoints, note that `Watchpoints` implements `FromIterator<(String, Watchpoint)>`,
    /// so you can for instance use `collect()` with an iterator over (watchpoint
    /// name, watchpoint) pairs.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Add a memory watchpoint. It will be enabled unless/until
    /// `disable()` is called on it.
    ///
    /// If a watchpoint with the same name was previously added, this will
    /// replace that watchpoint and return `true`. Otherwise, this will return
    /// `false`.
    pub fn add(&mut self, name: impl Into<String>, watchpoint: Watchpoint) -> bool {
        self.0.insert(name.into(), (watchpoint, true)).is_some()
    }

    /// Remove the memory watchpoint with the given `name`.
    ///
    /// Returns `true` if the operation was successful, or `false` if no
    /// watchpoint with that name was found.
    pub fn remove(&mut self, name: &str) -> bool {
        self.0.remove(name).is_some()
    }

    /// Disable the memory watchpoint with the given `name`.
    ///
    /// Returns `true` if the operation is successful, or `false` if no
    /// watchpoint with that name was found. Disabling an already-disabled
    /// watchpoint will have no effect and will return `true`.
    pub fn disable(&mut self, name: &str) -> bool {
        match self.0.get_mut(name) {
            Some(v) => { v.1 = false; true },
            None => false,
        }
    }

    /// Enable the memory watchpoint(s) with the given name.
    ///
    /// Returns `true` if the operation is successful, or `false` if no
    /// watchpoint with that name was found. Enabling an already-enabled
    /// watchpoint will have no effect and will return `true`.
    pub fn enable(&mut self, name: &str) -> bool {
        match self.0.get_mut(name) {
            Some(v) => { v.1 = true; true },
            None => false,
        }
    }

    /// For a memory operation on the given address with the given bitwidth, get
    /// `(name, watchpoint)` pairs corresponding to the active watchpoints which
    /// are triggered by the operation.
    pub(crate) fn get_triggered_watchpoints<V: BV>(
        &self,
        addr: &V,
        bits: u32,
    ) -> Result<impl Iterator<Item = (&String, &Watchpoint)>> {
        let btor = addr.get_solver();
        let addr_width = addr.get_width();
        let op_lower = addr;
        let bytes = if bits < 8 { 1 } else { bits / 8 };
        let op_upper = addr.add(&V::from_u32(btor.clone(), bytes - 1, addr_width));
        let results = self.0.iter().map(|(name, (watchpoint, enabled))| {
            if *enabled {
                if self.is_watchpoint_triggered(watchpoint, op_lower, &op_upper)? {
                    Ok(Some((name, watchpoint)))
                } else {
                    Ok(None)
                }
            } else {
                Ok(None)
            }
        }).collect::<Result<Vec<Option<(&String, &Watchpoint)>>>>();
        Ok(results?.into_iter().filter_map(|opt| opt))
    }

    /// Is the given watchpoint triggered on any address in the given interval (with both endpoints inclusive)?
    pub(crate) fn is_watchpoint_triggered<V: BV>(
        &self,
        watchpoint: &Watchpoint,
        interval_lower: &V,
        interval_upper: &V,
    ) -> Result<bool> {
        let btor = interval_lower.get_solver();
        let width = interval_lower.get_width();
        assert_eq!(width, interval_upper.get_width());

        let watchpoint_lower = V::from_u64(btor.clone(), watchpoint.low, width);
        let watchpoint_upper = V::from_u64(btor.clone(), watchpoint.high, width);

        // There are exactly 3 possibilities for how the watchpoint could be triggered:
        //
        // - the lower endpoint of the current mem read/write is contained in the watched interval
        //   current mem op:            -----
        //   watchpoint:           --------
        //
        // - the upper endpoint of the current mem read/write is contained in the watched interval
        //   current mem op:        -----
        //   watchpoint:              --------
        //
        // - neither endpoint of the current mem read/write is contained, but the read/write contains the entire watched interval
        //   current mem op:        ---------------
        //   watchpoint:              --------
        //
        // - (you may think there's a fourth case, where the watched interval contains the
        //      current mem read/write, but that will trigger both #1 and #2)
        let interval_lower_contained = interval_lower.ugte(&watchpoint_lower).and(&interval_lower.ulte(&watchpoint_upper));
        let interval_upper_contained = interval_upper.ugte(&watchpoint_lower).and(&interval_upper.ulte(&watchpoint_upper));
        let contains_entire_watchpoint = interval_lower.ulte(&watchpoint_lower).and(&interval_upper.ugte(&watchpoint_upper));

        solver_utils::sat_with_extra_constraints(&btor, std::iter::once(
            &interval_lower_contained.or(&interval_upper_contained).or(&contains_entire_watchpoint)
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use llvm_ir::Name;

    #[test]
    fn watchpoints() -> Result<()> {
        let func = blank_function("test_func", vec![Name::from("test_bb")]);
        let project = blank_project("test_mod", func);
        let state = blank_state(&project, "test_func");

        let mut watchpoints = Watchpoints::new();
        watchpoints.add("w1", Watchpoint::new(0x1000, 8));
        watchpoints.add("w2", Watchpoint::new(0x2000, 32));

        // Experiments on the first watchpoint
        let addr = state.bv_from_u32(0x1000, 64);

        // check that we can trigger it with a 1-byte read from 0x1000
        assert!(watchpoints.get_triggered_watchpoints(&addr, 8)?.next().is_some());

        // check that we can trigger it with an 8-byte read from 0x1000
        assert!(watchpoints.get_triggered_watchpoints(&addr, 64)?.next().is_some());

        // check that we can trigger it with a 1-byte read from 0x1002
        let addr = state.bv_from_u32(0x1002, 64);
        assert!(watchpoints.get_triggered_watchpoints(&addr, 8)?.next().is_some());

        // check that we can trigger it with a 8-byte read from 0x1002
        assert!(watchpoints.get_triggered_watchpoints(&addr, 64)?.next().is_some());

        // check that we don't trigger it with a 1-byte read from 0x0fff
        let addr = state.bv_from_u32(0x0fff, 64);
        assert!(watchpoints.get_triggered_watchpoints(&addr, 8)?.next().is_none());

        // check that we can trigger it with an 8-byte read from 0x0fff
        assert!(watchpoints.get_triggered_watchpoints(&addr, 64)?.next().is_some());

        // check that we don't trigger it with a 1-byte read from 0x1008
        let addr = state.bv_from_u32(0x1008, 64);
        assert!(watchpoints.get_triggered_watchpoints(&addr, 8)?.next().is_none());

        // check that we do trigger it with a 0x100-byte read from 0x0ff0
        let addr = state.bv_from_u32(0x0ff0, 64);
        assert!(watchpoints.get_triggered_watchpoints(&addr, 0x100 * 8)?.next().is_some());

        // disable it and check that we no longer trigger it
        assert!(watchpoints.disable("w1"));
        let addr = state.bv_from_u32(0x1002, 64);
        assert!(watchpoints.get_triggered_watchpoints(&addr, 8)?.next().is_none());

        // re-enable it
        assert!(watchpoints.enable("w1"));
        // also check that trying to disable or enable a non-existent watchpoint returns `false`
        assert!(!watchpoints.disable("foo"));
        assert!(!watchpoints.enable("foo"));

        // Experiments on the second watchpoint
        let addr = state.bv_from_u32(0x2000, 64);

        // check that we can trigger it with a 1-byte read from 0x2000
        assert!(watchpoints.get_triggered_watchpoints(&addr, 8)?.next().is_some());

        // check that we can trigger it with a 1-byte read from 0x2010
        let addr = state.bv_from_u32(0x2010, 64);
        assert!(watchpoints.get_triggered_watchpoints(&addr, 8)?.next().is_some());

        // check that a read touching both watchpoints does trigger
        let addr = state.bv_from_u32(0x0ff0, 64);
        assert!(watchpoints.get_triggered_watchpoints(&addr, 0x10000)?.next().is_some());

        // check that a read in between the two watchpoints doesn't trigger
        let addr = state.bv_from_u32(0x1f00, 64);
        assert!(watchpoints.get_triggered_watchpoints(&addr, 16)?.next().is_none());

        // fully remove the second watchpoint
        assert!(watchpoints.remove("w2"));

        // check that it is no longer triggered
        let addr = state.bv_from_u32(0x2000, 64);
        assert!(watchpoints.get_triggered_watchpoints(&addr, 8)?.next().is_none());

        // check that trying to re-enable it now returns false
        assert!(!watchpoints.enable("w2"));

        Ok(())
    }
}
