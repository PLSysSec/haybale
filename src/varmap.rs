// we have some methods on `VarMap` that may not currently be used by callers,
// but they still make sense to be part of `VarMap`
#![allow(dead_code)]

use crate::backend::{BV, SolverRef};
use crate::double_keyed_map::DoubleKeyedMap;
use crate::error::*;
use log::debug;

use llvm_ir::Name;

#[derive(Clone)]
pub struct VarMap<V: BV> {
    solver: V::SolverRef,
    /// Maps a `Name` to the `BV` corresponding to the active version of that `Name`.
    /// Different variables in different functions can have the same `Name` but
    /// different values, so we actually have `(String, Name)` as the key type
    /// where the `String` is the function name. We assume no two functions have
    /// the same name.
    active_version: DoubleKeyedMap<String, Name, V>,
    /// Maps a `Name` to the version number of the latest version of that `Name`.
    /// E.g., for `Name`s that have been created once, we have 0 here.
    /// Like with the `active_version` map, the key type here includes the function name.
    ///
    /// The version number here may not correspond to the active version in the
    /// presence of recursion: when we return from a recursive call, the caller's
    /// versions of the variables are active, even though the callee's versions
    /// are the most recently created.
    version_num: DoubleKeyedMap<String, Name, usize>,
    /// Maximum version number of any given `Name`.
    /// This bounds the maximum number of distinct versions of any given `Name`,
    /// and thus can be used to bound both loop iterations and recursion depth.
    ///
    /// Variables with the same `Name` in different functions do not share
    /// counters for this purpose - they can each have versions up to the
    /// `max_version_num`.
    max_version_num: usize,
}

impl<V: BV> VarMap<V> {
    /// `max_versions_of_name`: the maximum number of distinct versions allowed
    /// of any given `Name`, that is, the maximum number of `BV`s created for a
    /// given LLVM SSA value. Used to bound both loop iterations and recursion
    /// depth.
    ///
    /// Variables with the same `Name` in different functions do not share
    /// counters for this purpose - they can each have up to
    /// `max_versions_of_name` distinct versions.
    pub fn new(solver: V::SolverRef, max_versions_of_name: usize) -> Self {
        Self {
            solver,
            active_version: DoubleKeyedMap::new(),
            version_num: DoubleKeyedMap::new(),
            max_version_num: max_versions_of_name - 1,  // because 0 is a version
        }
    }

    /// Create a new (unconstrained) `BV` for the given `(String, Name)` pair.
    ///
    /// This function performs uniquing, so if you call it twice
    /// with the same `(String, Name)` pair, you will get two different `BV`s.
    ///
    /// Returns the new `BV`, or `Err` if it can't be created.
    ///
    /// (As of this writing, the only `Err` that might be returned is
    /// `Error::LoopBoundExceeded`, which is returned if creating the new `BV`
    /// would exceed `max_versions_of_name` -- see
    /// [`VarMap::new()`](struct.VarMap.html#method.new).)
    pub fn new_bv_with_name(&mut self, funcname: String, name: Name, bits: u32) -> Result<V> {
        let new_version = self.new_version_of_name(&funcname, &name)?;
        let bv = V::new(self.solver.clone(), bits, Some(&new_version));
        debug!("Adding var {:?} = {:?}", name, bv);
        self.active_version.insert(funcname, name, bv.clone());
        Ok(bv)
    }

    /// Assign the given `(String, Name)` pair to map to the given `BV`.
    ///
    /// This function performs uniquing, so it creates a new version of the
    /// variable represented by the `(String, Name)` pair rather than overwriting
    /// the current version.
    ///
    /// Returns `Err` if the assignment can't be performed.
    ///
    /// (As of this writing, the only `Err` that might be returned is
    /// `Error::LoopBoundExceeded`, which is returned if creating the new version
    /// of the `BV` would exceed `max_versions_of_name` -- see
    /// [`VarMap::new()`](struct.VarMap.html#method.new).)
    pub fn assign_bv_to_name(&mut self, funcname: String, name: Name, bv: V) -> Result<()> {
        let new_version_num = self.version_num.entry(funcname.clone(), name.clone())
            .and_modify(|v| *v += 1)  // increment if it already exists in map
            .or_insert(0);  // insert a 0 if it didn't exist in map
        if *new_version_num > self.max_version_num {
            Err(Error::LoopBoundExceeded)
        } else {
            // We don't actually use the new_version_num except for the above check,
            // since we aren't creating a new BV that needs a versioned name
            debug!("Assigning var {:?} = {:?}", name, bv);
            self.active_version.insert(funcname, name, bv);
            Ok(())
        }
    }

    /// Look up the most recent `BV` created for the given `(String, Name)` pair.
    #[allow(clippy::ptr_arg)]  // as of this writing, clippy warns that the &String argument should be &str; but it actually needs to be &String here
    pub fn lookup_var(&self, funcname: &String, name: &Name) -> &V {
        self.active_version.get(funcname, name).unwrap_or_else(|| {
            let keys: Vec<(&String, &Name)> = self.active_version.keys().collect();
            panic!("Failed to find var {:?} from function {:?} in map with keys {:?}", name, funcname, keys);
        })
    }

    /// Overwrite the latest version of the given `(String, Name)` pair to instead be `bv`.
    /// The `(String, Name)` pair must have already been previously assigned a value.
    #[allow(clippy::ptr_arg)]  // as of this writing, clippy warns that the &String argument should be &str; but it actually needs to be &String here
    pub fn overwrite_latest_version_of_bv(&mut self, funcname: &String, name: &Name, bv: V) {
        let mapvalue: &mut V = self.active_version
            .get_mut(funcname, name)
            .unwrap_or_else(|| panic!("failed to find current active version of {:?} (function {:?}) in map", name, funcname));
        *mapvalue = bv;
    }

    /// Given a `Name` (from a particular function), creates a new version of it
    /// and returns the corresponding versioned name
    /// (or `Error::LoopBoundExceeded` if it would exceed the `max_version_num`)
    fn new_version_of_name(&mut self, funcname: &str, name: &Name) -> Result<String> {
        let new_version_num = self.version_num.entry(funcname.to_owned(), name.clone())
            .and_modify(|v| *v += 1)  // increment if it already exists in map
            .or_insert(0);  // insert a 0 if it didn't exist in map
        if *new_version_num > self.max_version_num {
            Err(Error::LoopBoundExceeded)
        } else {
            Ok(Self::build_versioned_name(funcname, name, *new_version_num))
        }
    }

    /// Given a `Name` (from a particular function) and a version number, build
    /// the corresponding versioned name.
    ///
    /// This function does not modify (or even use) the current state of the
    /// `VarMap`.
    fn build_versioned_name(funcname: &str, name: &Name, version_num: usize) -> String {
        let (name_prefix, stem): (&str, String) = match name {
            Name::Name(s) => ("name_", s.clone()),
            Name::Number(n) => ("%", n.to_string()),
        };
        "@".to_owned() + funcname + "_" + name_prefix + &stem + "_" + &version_num.to_string()
    }

    /// Get a `RestoreInfo` which can later be used with `restore_fn_vars()` to
    /// restore all of the given function's variables (in their current active
    /// versions) back to active.
    ///
    /// This is intended to support recursion. A `RestoreInfo` can be generated
    /// before a recursive call, and then once the call returns, the restore
    /// operation ensures the caller still has access to the correct versions of
    /// its local variables (not the callee's versions, which are technically
    /// more recent).
    pub fn get_restore_info_for_fn(&self, funcname: String) -> RestoreInfo<V> {
        let pairs_to_restore: Vec<_> = self.active_version.iter()
            .filter(|(f,_,_)| f == &&funcname)
            .map(|(_,n,v)| (n.clone(), v.clone()))
            .collect();
        RestoreInfo {
            funcname,
            pairs_to_restore,
        }
    }

    /// Restore all of the variables in a `RestoreInfo` to their versions which
    /// were active at the time the `RestoreInfo` was generated
    pub fn restore_fn_vars(&mut self, rinfo: RestoreInfo<V>) {
        let funcname = rinfo.funcname.clone();
        for pair in rinfo.pairs_to_restore {
            let val = self.active_version
                .get_mut(&funcname, &pair.0)
                .unwrap_or_else(|| panic!("Malformed RestoreInfo: key {:?}", (&funcname, &pair.0)));
            *val = pair.1;
        }
    }

    /// Adapt the `VarMap` to a new solver instance.
    ///
    /// The new solver instance should have been created (possibly transitively)
    /// via `SolverRef::duplicate()` from the `SolverRef` this `VarMap` was
    /// originally created with (or most recently changed to). Further, no new
    /// variables should have been added since the call to
    /// `SolverRef::duplicate()`.
    pub fn change_solver(&mut self, new_solver: V::SolverRef) {
        for v in self.active_version.values_mut() {
            *v = new_solver.match_bv(v).unwrap();
        }
        self.solver = new_solver;
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RestoreInfo<V: BV> {
    funcname: String,
    pairs_to_restore: Vec<(Name, V)>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use boolector::Btor;
    use crate::solver_utils;
    use std::rc::Rc;

    type BV = boolector::BV<Rc<Btor>>;

    #[test]
    fn lookup_vars() {
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut varmap: VarMap<BV> = VarMap::new(btor, 20);
        let funcname = "foo".to_owned();

        // create llvm-ir names
        let name1 = Name::from("val");
        let name2 = Name::from(2);

        // create corresponding BV values
        let var1 = varmap.new_bv_with_name(funcname.clone(), name1.clone(), 64).unwrap();
        let var2 = varmap.new_bv_with_name(funcname.clone(), name2.clone(), 1).unwrap();  // these clone()s wouldn't normally be necessary but we want to compare against the original values later

        // check that looking up the llvm-ir values gives the correct BV ones
        assert_eq!(varmap.lookup_var(&funcname, &name1), &var1);
        assert_eq!(varmap.lookup_var(&funcname, &name2), &var2);
    }

    #[test]
    fn vars_are_uniqued() {
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut varmap: VarMap<BV> = VarMap::new(btor.clone(), 20);
        let funcname = "foo".to_owned();

        // create two vars with the same name
        let name = Name::from("x");
        let x1 = varmap.new_bv_with_name(funcname.clone(), name.clone(), 64).unwrap();
        let x2 = varmap.new_bv_with_name(funcname.clone(), name, 64).unwrap();

        // constrain with incompatible constraints
        x1.ugt(&BV::from_u64(btor.clone().into(), 2, 64)).assert();
        x2.ult(&BV::from_u64(btor.clone().into(), 1, 64)).assert();

        // check that we're still sat
        assert_eq!(solver_utils::sat(&btor), Ok(true));

        // now repeat with integer names
        let name = Name::from(3);
        let x1 = varmap.new_bv_with_name(funcname.clone(), name.clone(), 64).unwrap();
        let x2 = varmap.new_bv_with_name(funcname.clone(), name, 64).unwrap();
        x1.ugt(&BV::from_u64(btor.clone().into(), 2, 64)).assert();
        x2.ult(&BV::from_u64(btor.clone().into(), 1, 64)).assert();
        assert_eq!(solver_utils::sat(&btor), Ok(true));

        // now repeat with the same name but different functions
        let name = Name::from(10);
        let otherfuncname = "bar".to_owned();
        let x1 = varmap.new_bv_with_name(funcname.clone(), name.clone(), 64).unwrap();
        let x2 = varmap.new_bv_with_name(otherfuncname.clone(), name.clone(), 64).unwrap();
        x1.ugt(&BV::from_u64(btor.clone().into(), 2, 64)).assert();
        x2.ult(&BV::from_u64(btor.clone().into(), 1, 64)).assert();
        assert_eq!(solver_utils::sat(&btor), Ok(true));
    }

    #[test]
    fn enforces_max_version() {
        let btor = <Rc<Btor> as SolverRef>::new();

        // Create a `VarMap` with `max_version_num = 10`
        let mut varmap: VarMap<BV> = VarMap::new(btor, 10);

        // Check that we can create 10 versions of the same `Name`
        let funcname = "foo".to_owned();
        let name = Name::from(7);
        for _ in 0 .. 10 {
            let bv = varmap.new_bv_with_name(funcname.clone(), name.clone(), 64);
            assert!(bv.is_ok());
        }

        // Check that we can create another 10 versions of that `Name` in a different function
        let funcname2 = "bar".to_owned();
        for _ in 0 .. 10 {
            let bv = varmap.new_bv_with_name(funcname2.clone(), name.clone(), 64);
            assert!(bv.is_ok());
        }

        // Check that we can't create an 11th version of that `Name`
        let bv = varmap.new_bv_with_name(funcname, name, 64);
        assert!(bv.is_err());
    }

    #[test]
    fn restore_info() {
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut varmap: VarMap<BV> = VarMap::new(btor, 10);

        // create a var named "foo" in function "func"
        let fooname = Name::from("foo");
        let foo1 = varmap.new_bv_with_name("func".to_owned(), fooname.clone(), 64).unwrap();

        // save restore info for "func"
        let rinfo = varmap.get_restore_info_for_fn("func".to_owned());

        // create another var named "foo" in function "func"
        let foo2 = varmap.new_bv_with_name("func".to_owned(), fooname.clone(), 64).unwrap();

        // check that a lookup gives the most recent var
        assert_eq!(varmap.lookup_var(&"func".to_owned(), &fooname), &foo2);

        // restore, and check that a lookup now gives the first var
        varmap.restore_fn_vars(rinfo);
        assert_eq!(varmap.lookup_var(&"func".to_owned(), &fooname), &foo1);
    }

    #[test]
    fn restore_different_function() {
        let btor = <Rc<Btor> as SolverRef>::new();
        let mut varmap: VarMap<BV> = VarMap::new(btor, 10);

        // create a var named "foo" in function "func"
        let fooname = Name::from("foo");
        let _foo1 = varmap.new_bv_with_name("func".to_owned(), fooname.clone(), 64).unwrap();

        // save restore info for function "blah"
        let rinfo_blah = varmap.get_restore_info_for_fn("blah".to_owned());

        // create another var named "foo" in function "func"
        let foo2 = varmap.new_bv_with_name("func".to_owned(), fooname.clone(), 64).unwrap();

        // restore function "blah", and check that lookups in function "func" are unaffected
        assert_eq!(varmap.lookup_var(&"func".to_owned(), &fooname), &foo2);
        varmap.restore_fn_vars(rinfo_blah);
        assert_eq!(varmap.lookup_var(&"func".to_owned(), &fooname), &foo2);
    }
}
