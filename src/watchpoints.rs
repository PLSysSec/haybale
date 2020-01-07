//! Structures for defining and processing memory watchpoints

use crate::backend::{Backend, BV};
use crate::error::Result;
use crate::state::State;
use log::info;
use std::collections::HashMap;
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
#[derive(Clone)]
pub struct Watchpoints(HashMap<String, (Watchpoint, bool)>);

impl Watchpoints {
    /// Construct a new `Watchpoints` instance with no watchpoints
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Construct a new `Watchpoints` instance from an iterator over (watchpoint
    /// name, watchpoint) pairs
    pub fn from_iter(iter: impl IntoIterator<Item = (String, Watchpoint)>) -> Self {
        Self(iter.into_iter().map(|(name, w)| (name, (w, true))).collect())
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

    /// For a memory operation on the given address with the given bitwidth,
    /// check whether it triggers any active watchpoints.
    ///
    /// If it does, then generate the appropriate log messages for watchpoints
    /// being triggered.
    ///
    /// Returns `true` if any watchpoints were triggered, `false` if not.
    ///
    /// `is_write`: whether the operation is a read (`false`) or write (`true`),
    /// used only for composing the log message.
    pub(crate) fn process_watchpoint_triggers<B: Backend>(
        &self,
        state: &State<B>,
        addr: &B::BV,
        bits: u32,
        is_write: bool,
    ) -> Result<bool> {
        let mut retval = false;
        if !self.0.is_empty() {
            let addr_width = addr.get_width();
            let op_lower = addr;
            let bytes = if bits < 8 { 1 } else { bits / 8 };
            let op_upper = addr.add(&state.bv_from_u32(bytes - 1, addr_width));
            for (name, (watchpoint, enabled)) in self.0.iter() {
                if *enabled && self.is_watchpoint_triggered(state, watchpoint, op_lower, &op_upper)? {
                    retval = true;
                    info!("Memory watchpoint {:?} {} {} by {:?}", name, watchpoint, if is_write { "written" } else { "read" }, state.cur_loc);
                }
            }
        }
        Ok(retval)
    }

    /// Is the given watchpoint triggered on any address in the given interval (with both endpoints inclusive)?
    pub(crate) fn is_watchpoint_triggered<B: Backend>(
        &self,
        state: &State<B>,
        watchpoint: &Watchpoint,
        interval_lower: &B::BV,
        interval_upper: &B::BV,
    ) -> Result<bool> {
        let width = interval_lower.get_width();
        assert_eq!(width, interval_upper.get_width());

        let watchpoint_lower = state.bv_from_u64(watchpoint.low, width);
        let watchpoint_upper = state.bv_from_u64(watchpoint.high, width);

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

        state.sat_with_extra_constraints(std::iter::once(
            &interval_lower_contained.or(&interval_upper_contained).or(&contains_entire_watchpoint)
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::BtorBackend;
    use crate::config::Config;
    use crate::state::Location;
    use crate::project::Project;
    use llvm_ir::*;

    /// First some utilities copy-pasted from the unit tests for `State`

    /// utility to initialize a `State` out of a `Project` and a function name
    fn blank_state<'p>(project: &'p Project, funcname: &str) -> State<'p, BtorBackend> {
        let (func, module) = project.get_func_by_name(funcname).expect("Failed to find function");
        let start_loc = Location {
            module,
            func,
            bbname: "test_bb".to_owned().into(),
            instr: 0,
        };
        State::new(project, start_loc, Config::default())
    }

    /// Utility that creates a simple `Project` for testing.
    /// The `Project` will contain a single `Module` (with the given name) which contains
    /// a single function (given).
    fn blank_project(modname: impl Into<String>, func: Function) -> Project {
        Project::from_module(Module {
            name: modname.into(),
            source_file_name: String::new(),
            data_layout: String::new(),
            target_triple: None,
            functions: vec![func],
            global_vars: vec![],
            global_aliases: vec![],
            named_struct_types: HashMap::new(),
            inline_assembly: String::new(),
        })
    }

    /// utility that creates a technically valid (but functionally useless) `Function` for testing
    fn blank_function(name: impl Into<String>) -> Function {
        Function::new(name)
    }

    #[test]
    fn watchpoints() -> Result<()> {
        let func = blank_function("test_func");
        let project = blank_project("test_mod", func);
        let state = blank_state(&project, "test_func");

        let mut watchpoints = Watchpoints::new();
        watchpoints.add("w1", Watchpoint::new(0x1000, 8));
        watchpoints.add("w2", Watchpoint::new(0x2000, 32));

        // Experiments on the first watchpoint
        let addr = state.bv_from_u32(0x1000, 64);

        // check that we can trigger it with a 1-byte write to 0x1000
        assert!(watchpoints.process_watchpoint_triggers(&state, &addr, 8, true)?);

        // check that we can trigger it with an 8-byte write to 0x1000
        assert!(watchpoints.process_watchpoint_triggers(&state, &addr, 64, true)?);

        // check that we can trigger it with a 1-byte write to 0x1002
        let addr = state.bv_from_u32(0x1002, 64);
        assert!(watchpoints.process_watchpoint_triggers(&state, &addr, 8, true)?);

        // check that we can trigger it with a 8-byte write to 0x1002
        assert!(watchpoints.process_watchpoint_triggers(&state, &addr, 64, true)?);

        // check that we don't trigger it with a 1-byte write to 0x0fff
        let addr = state.bv_from_u32(0x0fff, 64);
        assert!(!watchpoints.process_watchpoint_triggers(&state, &addr, 8, true)?);

        // check that we can trigger it with an 8-byte write to 0x0fff
        assert!(watchpoints.process_watchpoint_triggers(&state, &addr, 64, true)?);

        // check that we don't trigger it with a 1-byte write to 0x1008
        let addr = state.bv_from_u32(0x1008, 64);
        assert!(!watchpoints.process_watchpoint_triggers(&state, &addr, 8, true)?);

        // check that we do trigger it with a 0x100-byte write to 0x0ff0
        let addr = state.bv_from_u32(0x0ff0, 64);
        assert!(watchpoints.process_watchpoint_triggers(&state, &addr, 0x100 * 8, true)?);

        // disable it and check that we no longer trigger it
        assert!(watchpoints.disable("w1"));
        let addr = state.bv_from_u32(0x1002, 64);
        assert!(!watchpoints.process_watchpoint_triggers(&state, &addr, 8, true)?);

        // re-enable it
        assert!(watchpoints.enable("w1"));
        // also check that trying to disable or enable a non-existent watchpoint returns `false`
        assert!(!watchpoints.disable("foo"));
        assert!(!watchpoints.enable("foo"));

        // Experiments on the second watchpoint
        let addr = state.bv_from_u32(0x2000, 64);

        // check that we can trigger it with a 1-byte write to 0x2000
        assert!(watchpoints.process_watchpoint_triggers(&state, &addr, 8, true)?);

        // check that we can trigger it with a 1-byte write to 0x2010
        let addr = state.bv_from_u32(0x2010, 64);
        assert!(watchpoints.process_watchpoint_triggers(&state, &addr, 8, true)?);

        // check that a write touching both watchpoints does trigger
        let addr = state.bv_from_u32(0x0ff0, 64);
        assert!(watchpoints.process_watchpoint_triggers(&state, &addr, 0x10000, true)?);

        // check that a write in between the two watchpoints doesn't trigger
        let addr = state.bv_from_u32(0x1f00, 64);
        assert!(!watchpoints.process_watchpoint_triggers(&state, &addr, 16, true)?);

        // fully remove the second watchpoint
        assert!(watchpoints.remove("w2"));

        // check that it is no longer triggered
        let addr = state.bv_from_u32(0x2000, 64);
        assert!(!watchpoints.process_watchpoint_triggers(&state, &addr, 8, true)?);

        // check that trying to re-enable it now returns false
        assert!(!watchpoints.enable("w2"));

        Ok(())
    }
}
