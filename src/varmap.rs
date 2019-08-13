// we have some methods on `VarMap` and/or `BVorBool` that may not currently be
// used by callers, but they still make sense to be part of `VarMap`/`BVorBool`
#![allow(dead_code)]

use crate::double_keyed_map::DoubleKeyedMap;
use std::fmt;
use log::debug;
use crate::backend::*;

use llvm_ir::Name;

#[derive(Clone)]
pub struct VarMap<'ctx, V, B> where V: BV<'ctx, AssociatedBool = B>, B: Bool<'ctx, AssociatedBV = V> {
    ctx: &'ctx z3::Context,
    /// Maps a `Name` to the Z3 object corresponding to the active version of that `Name`.
    /// Different variables in different functions can have the same `Name` but different
    /// values, so we actually have `(String, Name)` as the key type where the `String` is the
    /// function name. We assume no two functions have the same name.
    active_version: DoubleKeyedMap<String, Name, BVorBool<'ctx, V, B>>,
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

/// Our `VarMap` stores both `BV`s and `Bool`s
#[derive(Clone, PartialEq, Eq)]
enum BVorBool<'ctx, V, B>
    where V: BV<'ctx, AssociatedBool = B>, B: Bool<'ctx, AssociatedBV = V>
{
    BV(V, std::marker::PhantomData<&'ctx ()>),
    Bool(B, std::marker::PhantomData<&'ctx ()>),
}

impl<'ctx, V, B> fmt::Debug for BVorBool<'ctx, V, B>
    where V: BV<'ctx, AssociatedBool = B>, B: Bool<'ctx, AssociatedBV = V>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BVorBool::BV(bv, _) => write!(f, "BV( {:?} )", bv),
            BVorBool::Bool(b, _) => write!(f, "Bool( {:?} )", b),
        }
    }
}

impl<'ctx, V, B> BVorBool<'ctx, V, B>
    where V: BV<'ctx, AssociatedBool = B>, B: Bool<'ctx, AssociatedBV = V>
{
    fn bv(bv: V) -> Self {
        BVorBool::BV(bv, std::marker::PhantomData)
    }

    fn bool(b: B) -> Self {
        BVorBool::Bool(b, std::marker::PhantomData)
    }
}

impl<'ctx, V, B> BVorBool<'ctx, V, B>
    where V: BV<'ctx, AssociatedBool = B>, B: Bool<'ctx, AssociatedBV = V>
{
    fn is_bv(&self) -> bool {
        match self {
            BVorBool::BV(_, _) => true,
            _ => false,
        }
    }

    fn is_bool(&self) -> bool {
        match self {
            BVorBool::Bool(_, _) => true,
            _ => false,
        }
    }
}

// these are basically From impls, but for converting ref to ref
impl<'ctx, V, B> BVorBool<'ctx, V, B>
    where V: BV<'ctx, AssociatedBool = B>, B: Bool<'ctx, AssociatedBV = V>
{
    fn as_bv(&self) -> &V {
        match self {
            BVorBool::BV(bv, _) => &bv,
            _ => panic!("Can't convert {:?} to BV", self),
        }
    }

    fn as_bool(&self) -> &B {
        match self {
            BVorBool::Bool(b, _) => &b,
            _ => panic!("Can't convert {:?} to Bool", self),
        }
    }
}

// these are like the From impls, but make more of an effort to convert between
// types if the wrong type is requested
impl<'ctx, V, B> BVorBool<'ctx, V, B>
    where V: BV<'ctx, AssociatedBool = B>, B: Bool<'ctx, AssociatedBV = V>
{
    fn into_bv(self, ctx: &'ctx z3::Context) -> V {
        match self {
            BVorBool::BV(bv, _) => bv,
            BVorBool::Bool(b, _) => b.bvite(&V::from_u64(ctx, 1, 1), &V::from_u64(ctx, 0, 1)),
        }
    }

    fn into_bool(self, ctx: &'ctx z3::Context) -> B {
        match self {
            BVorBool::Bool(b, _) => b,
            BVorBool::BV(bv, _) => {
                if bv.get_size() == 1 {
                    bv._eq(&BV::from_u64(ctx, 1, 1))
                } else {
                    panic!("Can't convert BV {:?} of size {} to Bool", bv, bv.get_size())
                }
            },
        }
    }
}

impl<'ctx, V, B> VarMap<'ctx, V, B>
    where V: BV<'ctx, AssociatedBool = B>, B: Bool<'ctx, AssociatedBV = V>
{
    /// `max_versions_of_name`: the maximum number of distinct versions allowed
    /// of any given `Name`, that is, the maximum number of Z3 objects created
    /// for a given LLVM SSA value. Used to bound both loop iterations and
    /// recursion depth.
    ///
    /// Variables with the same `Name` in different functions do not share
    /// counters for this purpose - they can each have up to
    /// `max_versions_of_name` distinct versions.
    pub fn new(ctx: &'ctx z3::Context, max_versions_of_name: usize) -> Self {
        Self {
            ctx,
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
    /// (As of this writing, the only reason an `Err` might be returned is that
    /// creating the new `BV` would exceed `max_versions_of_name` -- see
    /// [`VarMap::new()`](struct.VarMap.html#method.new).)
    pub fn new_bv_with_name(&mut self, funcname: String, name: Name, bits: u32) -> Result<V, &'static str> {
        let new_version = self.new_version_of_name(&funcname, &name)?;
        let bv = V::new(self.ctx, new_version, bits);
        debug!("Adding bv var {:?} = {:?}", name, bv);
        self.active_version.insert(funcname, name, BVorBool::bv(bv.clone()));
        Ok(bv)
    }

    /// Create a new (unconstrained) `Bool` for the given `(String, Name)` pair.
    ///
    /// This function performs uniquing, so if you call it twice
    /// with the same `(String, Name)` pair, you will get two different `Bool`s.
    ///
    /// Returns the new `Bool`, or `Err` if it can't be created.
    ///
    /// (As of this writing, the only reason an `Err` might be returned is that
    /// creating the new `Bool` would exceed `max_versions_of_name` -- see
    /// [`VarMap::new()`](struct.VarMap.html#method.new).)
    pub fn new_bool_with_name(&mut self, funcname: String, name: Name) -> Result<B, &'static str>
    {
        let new_version = self.new_version_of_name(&funcname, &name)?;
        let b = B::new(self.ctx, new_version);
        debug!("Adding bool var {:?} = {:?}", name, b);
        self.active_version.insert(funcname, name, BVorBool::bool(b.clone()));
        Ok(b)
    }

    /// Assign the given `(String, Name)` pair to map to the given `BV`.
    ///
    /// This function performs uniquing, so it creates a new version of the
    /// variable represented by the `(String, Name)` pair rather than overwriting
    /// the current version.
    ///
    /// Returns `Err` if the assignment can't be performed.
    /// (As of this writing, the only reason an `Err` might be returned is that
    /// creating the new version of the `BV` would exceed `max_versions_of_name`
    /// -- see [`VarMap::new()`](struct.VarMap.html#method.new).)
    pub fn assign_bv_to_name(&mut self, funcname: String, name: Name, bv: V) -> Result<(), &'static str> {
        let new_version_num = self.version_num.entry(funcname.clone(), name.clone())
            .and_modify(|v| *v += 1)  // increment if it already exists in map
            .or_insert(0);  // insert a 0 if it didn't exist in map
        if *new_version_num > self.max_version_num {
            Err("Exceeded maximum number of versions of that `Name`")
        } else {
            // We don't actually use the new_version_num except for the above check,
            // since we aren't creating a new BV that needs a versioned name
            debug!("Assigning bv var {:?} = {:?}", name, bv);
            self.active_version.insert(funcname, name, BVorBool::bv(bv));
            Ok(())
        }
    }

    /// Assign the given `(String, Name)` pair to map to the given `Bool`.
    ///
    /// This function performs uniquing, so it creates a new version of the
    /// variable represented by the `(String, Name)` pair rather than overwriting
    /// the current version.
    ///
    /// Returns `Err` if the assignment can't be performed.
    /// (As of this writing, the only reason an `Err` might be returned is that
    /// creating the new version of the `Bool` would exceed `max_versions_of_name`
    /// -- see [`VarMap::new()`](struct.VarMap.html#method.new).)
    pub fn assign_bool_to_name(&mut self, funcname: String, name: Name, b: B) -> Result<(), &'static str> {
        let new_version_num = self.version_num.entry(funcname.clone(), name.clone())
            .and_modify(|v| *v += 1)  // increment if it already exists in map
            .or_insert(0);  // insert a 0 if it didn't exist in map
        if *new_version_num > self.max_version_num {
            Err("Exceeded maximum number of versions of that `Name`")
        } else {
            // We don't actually use the new_version_num except for the above check,
            // since we aren't creating a new BV that needs a versioned name
            debug!("Assigning bool var {:?} = {:?}", name, b);
            self.active_version.insert(funcname, name, BVorBool::bool(b));
            Ok(())
        }
    }

    /// Look up the most recent `BV` created for the given `(String, Name)` pair
    pub fn lookup_bv_var(&self, funcname: &String, name: &Name) -> V {
        self.active_version.get(funcname, name).unwrap_or_else(|| {
            let keys: Vec<(&String, &Name)> = self.active_version.keys().collect();
            panic!("Failed to find var {:?} from function {:?} in map with keys {:?}", name, funcname, keys);
        }).clone().into_bv(self.ctx)
    }

    /// Look up the most recent `Bool` created for the given `(String, Name)` pair
    pub fn lookup_bool_var(&self, funcname: &String, name: &Name) -> B {
        self.active_version.get(funcname, name).unwrap_or_else(|| {
            let keys: Vec<(&String, &Name)> = self.active_version.keys().collect();
            panic!("Failed to find var {:?} from function {:?} in map with keys {:?}", name, funcname, keys);
        }).clone().into_bool(self.ctx)
    }

    /// Overwrite the latest version of the given `(String, Name)` pair to instead be `bv`.
    /// The `(String, Name)` pair must have already been previously assigned a value.
    pub fn overwrite_latest_version_of_bv(&mut self, funcname: &String, name: &Name, bv: V) {
        let mapvalue: &mut BVorBool<V, B> = self.active_version
            .get_mut(funcname, name)
            .expect("failed to find current active version in map");
        *mapvalue = BVorBool::bv(bv);
    }

    /// Overwrite the latest version of the given `(String, Name)` pair to instead be `b`.
    /// The `(String, Name)` pair must have already been previously assigned a value.
    pub fn overwrite_latest_version_of_bool(&mut self, funcname: &String, name: &Name, b: B) {
        let mapvalue: &mut BVorBool<V, B> = self.active_version
            .get_mut(funcname, name)
            .expect("failed to find current active version in map");
        *mapvalue = BVorBool::bool(b);
    }

    /// Given a `Name` (from a particular function), creates a new version of it
    /// and returns the corresponding `z3::Symbol`
    /// (or `Err` if it would exceed the `max_version_num`)
    fn new_version_of_name(&mut self, funcname: &str, name: &Name) -> Result<z3::Symbol, &'static str> {
        let new_version_num = self.version_num.entry(funcname.to_owned(), name.clone())
            .and_modify(|v| *v += 1)  // increment if it already exists in map
            .or_insert(0);  // insert a 0 if it didn't exist in map
        if *new_version_num > self.max_version_num {
            Err("Exceeded maximum number of versions of that `Name`")
        } else {
            Ok(Self::build_versioned_name(funcname, name, *new_version_num))
        }
    }

    /// Given a `Name` (from a particular function) and a version number, build the corresponding
    /// `z3::Symbol`.
    ///
    /// This function does not modify (or even use) the current state of the `VarMap`.
    fn build_versioned_name(funcname: &str, name: &Name, version_num: usize) -> z3::Symbol {
        let (name_prefix, stem): (&str, String) = match name {
            Name::Name(s) => ("name_", s.clone()),
            Name::Number(n) => ("%", n.to_string()),
        };
        z3::Symbol::String("@".to_owned() + funcname + "_" + name_prefix + &stem + "_" + &version_num.to_string())
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
    pub fn get_restore_info_for_fn(&self, funcname: String) -> RestoreInfo<'ctx, V, B> {
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
    pub fn restore_fn_vars(&mut self, rinfo: RestoreInfo<'ctx, V, B>) {
        let funcname = rinfo.funcname.clone();
        for pair in rinfo.pairs_to_restore {
            let val = self.active_version
                .get_mut(&funcname, &pair.0)
                .unwrap_or_else(|| panic!("Malformed RestoreInfo: key {:?}", (&funcname, &pair.0)));
            *val = pair.1;
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RestoreInfo<'ctx, V, B> where V: BV<'ctx, AssociatedBool = B>, B: Bool<'ctx, AssociatedBV = V> {
    funcname: String,
    pairs_to_restore: Vec<(Name, BVorBool<'ctx, V, B>)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_vars() {
        let ctx = z3::Context::new(&z3::Config::new());
        let mut varmap: VarMap<z3::ast::BV, z3::ast::Bool> = VarMap::new(&ctx, 20);
        let funcname = "foo".to_owned();

        // create llvm-ir names
        let valname = Name::Name("val".to_owned());
        let boolname = Name::Number(2);

        // create corresponding Z3 values
        let valvar = varmap.new_bv_with_name(funcname.clone(), valname.clone(), 64).unwrap();
        let boolvar = varmap.new_bool_with_name(funcname.clone(), boolname.clone()).unwrap();  // these clone()s wouldn't normally be necessary but we want to compare against the original values later

        // check that looking up the llvm-ir values gives the correct Z3 ones
        assert_eq!(varmap.lookup_bv_var(&funcname, &valname), valvar);
        assert_eq!(varmap.lookup_bool_var(&funcname, &boolname), boolvar);
    }

    #[test]
    fn vars_are_uniqued() {
        let ctx = z3::Context::new(&z3::Config::new());
        let mut varmap: VarMap<z3::ast::BV, z3::ast::Bool> = VarMap::new(&ctx, 20);
        let mut solver = crate::solver::Solver::new(&ctx);
        let funcname = "foo".to_owned();

        // create two vars with the same name
        let name = Name::Name("x".to_owned());
        let x1 = varmap.new_bv_with_name(funcname.clone(), name.clone(), 64).unwrap();
        let x2 = varmap.new_bv_with_name(funcname.clone(), name, 64).unwrap();

        // constrain with incompatible constraints
        solver.assert(&x1.bvugt(&BV::from_u64(&ctx, 2, 64)));
        solver.assert(&x2.bvult(&BV::from_u64(&ctx, 1, 64)));

        // check that we're still sat
        assert_eq!(solver.check(), Ok(true));

        // now repeat with integer names
        let name = Name::Number(3);
        let x1 = varmap.new_bv_with_name(funcname.clone(), name.clone(), 64).unwrap();
        let x2 = varmap.new_bv_with_name(funcname.clone(), name, 64).unwrap();
        solver.assert(&x1.bvugt(&BV::from_u64(&ctx, 2, 64)));
        solver.assert(&x2.bvult(&BV::from_u64(&ctx, 1, 64)));
        assert_eq!(solver.check(), Ok(true));

        // now repeat with the same name but different functions
        let name = Name::Number(10);
        let otherfuncname = "bar".to_owned();
        let x1 = varmap.new_bv_with_name(funcname.clone(), name.clone(), 64).unwrap();
        let x2 = varmap.new_bv_with_name(otherfuncname.clone(), name.clone(), 64).unwrap();
        solver.assert(&x1.bvugt(&BV::from_u64(&ctx, 2, 64)));
        solver.assert(&x2.bvult(&BV::from_u64(&ctx, 1, 64)));
        assert_eq!(solver.check(), Ok(true));
    }

    #[test]
    fn enforces_max_version() {
        let ctx = z3::Context::new(&z3::Config::new());

        // Create a `VarMap` with `max_version_num = 10`
        let mut varmap: VarMap<z3::ast::BV, z3::ast::Bool> = VarMap::new(&ctx, 10);

        // Check that we can create 10 versions of the same `Name`
        let funcname = "foo".to_owned();
        let name = Name::Number(7);
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
        let ctx = z3::Context::new(&z3::Config::new());
        let mut varmap: VarMap<z3::ast::BV, z3::ast::Bool> = VarMap::new(&ctx, 10);

        // create a var named "foo" in function "func"
        let fooname = Name::Name("foo".to_owned());
        let foo1 = varmap.new_bv_with_name("func".to_owned(), fooname.clone(), 64).unwrap();

        // save restore info for "func"
        let rinfo = varmap.get_restore_info_for_fn("func".to_owned());

        // create another var named "foo" in function "func"
        let foo2 = varmap.new_bv_with_name("func".to_owned(), fooname.clone(), 64).unwrap();

        // check that a lookup gives the most recent var
        assert_eq!(varmap.lookup_bv_var(&"func".to_owned(), &fooname), foo2);

        // restore, and check that a lookup now gives the first var
        varmap.restore_fn_vars(rinfo);
        assert_eq!(varmap.lookup_bv_var(&"func".to_owned(), &fooname), foo1);
    }

    #[test]
    fn restore_different_function() {
        let ctx = z3::Context::new(&z3::Config::new());
        let mut varmap: VarMap<z3::ast::BV, z3::ast::Bool> = VarMap::new(&ctx, 10);

        // create a var named "foo" in function "func"
        let fooname = Name::Name("foo".to_owned());
        let _foo1 = varmap.new_bv_with_name("func".to_owned(), fooname.clone(), 64).unwrap();

        // save restore info for function "blah"
        let rinfo_blah = varmap.get_restore_info_for_fn("blah".to_owned());

        // create another var named "foo" in function "func"
        let foo2 = varmap.new_bv_with_name("func".to_owned(), fooname.clone(), 64).unwrap();

        // restore function "blah", and check that lookups in function "func" are unaffected
        assert_eq!(varmap.lookup_bv_var(&"func".to_owned(), &fooname), foo2);
        varmap.restore_fn_vars(rinfo_blah);
        assert_eq!(varmap.lookup_bv_var(&"func".to_owned(), &fooname), foo2);
    }
}
