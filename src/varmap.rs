use std::collections::HashMap;
use std::fmt;
use z3::ast::{Ast, BV, Bool};
use log::debug;

use llvm_ir::Name;

#[derive(Clone)]
pub struct VarMap<'ctx> {
    ctx: &'ctx z3::Context,
    /// Maps a `Name` to the Z3 object corresponding to the latest version of that `Name`
    latest_version: HashMap<Name, BVorBool<'ctx>>,
    /// Maps a `Name` to the version number of the latest version of that `Name`.
    /// E.g., for `Name`s that have been created once, we have 0 here.
    version_num: HashMap<Name, usize>,
    /// Maximum version number of any given `Name`.
    /// This bounds the maximum number of distinct versions of any given `Name`,
    /// and thus can be used to bound loops, really crudely
    max_version_num: usize,
}

/// Our `VarMap` stores both `BV`s and `Bool`s
#[derive(Clone, PartialEq, Eq)]
enum BVorBool<'ctx> {
    BV(BV<'ctx>),
    Bool(Bool<'ctx>),
}

impl<'ctx> fmt::Debug for BVorBool<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BVorBool::BV(bv) => write!(f, "BV( {} )", bv),
            BVorBool::Bool(b) => write!(f, "Bool( {} )", b),
        }
    }
}

impl<'ctx> From<BV<'ctx>> for BVorBool<'ctx> {
    fn from(bv: BV<'ctx>) -> BVorBool<'ctx> {
        BVorBool::BV(bv)
    }
}

impl<'ctx> From<Bool<'ctx>> for BVorBool<'ctx> {
    fn from(b: Bool<'ctx>) -> BVorBool<'ctx> {
        BVorBool::Bool(b)
    }
}

impl<'ctx> From<BVorBool<'ctx>> for BV<'ctx> {
    fn from(b: BVorBool<'ctx>) -> BV<'ctx> {
        match b {
            BVorBool::BV(bv) => bv,
            _ => panic!("Can't convert {:?} to BV", b),
        }
    }
}

impl<'ctx> From<BVorBool<'ctx>> for Bool<'ctx> {
    fn from(b: BVorBool<'ctx>) -> Bool<'ctx> {
        match b {
            BVorBool::Bool(b) => b,
            _ => panic!("Can't convert {:?} to Bool", b),
        }
    }
}

impl<'ctx> BVorBool<'ctx> {
    fn is_bv(&self) -> bool {
        match self {
            BVorBool::BV(_) => true,
            _ => false,
        }
    }

    fn is_bool(&self) -> bool {
        match self {
            BVorBool::Bool(_) => true,
            _ => false,
        }
    }
}

// these are basically From impls, but for converting ref to ref
impl<'ctx> BVorBool<'ctx> {
    fn as_bv(&self) -> &BV<'ctx> {
        match self {
            BVorBool::BV(bv) => &bv,
            _ => panic!("Can't convert {:?} to BV", self),
        }
    }

    fn as_bool(&self) -> &Bool<'ctx> {
        match self {
            BVorBool::Bool(b) => &b,
            _ => panic!("Can't convert {:?} to Bool", self),
        }
    }
}

// these are like the From impls, but make more of an effort to convert between
// types if the wrong type is requested
impl<'ctx> BVorBool<'ctx> {
    fn to_bv(self, ctx: &'ctx z3::Context) -> BV<'ctx> {
        match self {
            BVorBool::BV(bv) => bv,
            BVorBool::Bool(b) => b.ite(&BV::from_u64(ctx, 1, 1), &BV::from_u64(ctx, 0, 1)),
        }
    }

    fn to_bool(self, ctx: &'ctx z3::Context) -> Bool<'ctx> {
        match self {
            BVorBool::Bool(b) => b,
            BVorBool::BV(bv) => {
                if bv.get_size() == 1 {
                    bv._eq(&BV::from_u64(ctx, 1, 1))
                } else {
                    panic!("Can't convert BV {:?} of size {} to Bool", bv, bv.get_size())
                }
            },
        }
    }
}

impl<'ctx> VarMap<'ctx> {
    /// `max_versions_of_name`: Maximum number of distinct versions of any given `Name`.
    /// This can be used to bound loops (really crudely)
    pub fn new(ctx: &'ctx z3::Context, max_versions_of_name: usize) -> Self {
        Self {
            ctx,
            latest_version: HashMap::new(),
            version_num: HashMap::new(),
            max_version_num: max_versions_of_name - 1,  // because 0 is a version
        }
    }

    /// Create a new `BV` for the given `Name`.
    /// This function performs uniquing, so if you call it twice
    /// with the same `Name`, you will get two different `BV`s.
    /// Returns the new `BV`, or `Err` if it can't be created.
    /// (As of this writing, the only reason an `Err` might be returned is that
    /// creating the new `BV` would exceed `max_versions_of_name` -- see
    /// [`VarMap::new()`](struct.VarMap.html#method.new).)
    pub fn new_bv_with_name(&mut self, name: Name, bits: u32) -> Result<BV<'ctx>, &'static str> {
        let new_version = self.new_version_of_name(&name)?;
        let bv = BV::new_const(self.ctx, new_version, bits);
        debug!("Adding bv var {:?} = {}", name, bv);
        self.latest_version.insert(name, bv.clone().into());
        Ok(bv)
    }

    /// Create a new `Bool` for the given `Name`.
    /// This function performs uniquing, so if you call it twice
    /// with the same `Name`, you will get two different `Bool`s.
    /// Returns the new `Bool`, or `Err` if it can't be created.
    /// (As of this writing, the only reason an `Err` might be returned is that
    /// creating the new `Bool` would exceed `max_versions_of_name` -- see
    /// [`VarMap::new()`](struct.VarMap.html#method.new).)
    pub fn new_bool_with_name(&mut self, name: Name) -> Result<Bool<'ctx>, &'static str> {
        let new_version = self.new_version_of_name(&name)?;
        let b = Bool::new_const(self.ctx, new_version);
        debug!("Adding bool var {:?} = {}", name, b);
        self.latest_version.insert(name, b.clone().into());
        Ok(b)
    }

    /// Look up the most recent `BV` created for the given `Name`
    pub fn lookup_bv_var(&self, name: &Name) -> BV<'ctx> {
        debug!("Looking up var {:?}", name);
        self.latest_version.get(name).unwrap_or_else(|| {
            let keys: Vec<&Name> = self.latest_version.keys().collect();
            panic!("Failed to find var {:?} in map with keys {:?}", name, keys);
        }).clone().to_bv(self.ctx)
    }

    /// Look up the most recent `Bool` created for the given `Name`
    pub fn lookup_bool_var(&self, name: &Name) -> Bool<'ctx> {
        debug!("Looking up var {:?}", name);
        self.latest_version.get(name).unwrap_or_else(|| {
            let keys: Vec<&Name> = self.latest_version.keys().collect();
            panic!("Failed to find var {:?} in map with keys {:?}", name, keys);
        }).clone().to_bool(self.ctx)
    }

    /// Given a `Name`, creates a new version of it and returns the corresponding `z3::Symbol`
    /// (or `Err` if it would exceed the `max_version_num`)
    fn new_version_of_name(&mut self, name: &Name) -> Result<z3::Symbol, &'static str> {
        let new_version_num = self.version_num.entry(name.clone())  // it doesn't make sense to me why entry() takes the key by value rather than just a reference to it?
            .and_modify(|v| *v += 1)  // increment if it already exists in map
            .or_insert(0);  // insert a 0 if it didn't exist in map
        if *new_version_num > self.max_version_num {
            return Err("Exceeded maximum number of versions of that `Name`");
        }
        let mut suffix = "_".to_string();
        suffix.push_str(&new_version_num.to_string());
        let (mut prefix, stem) = match name {
            Name::Name(s) => ("name_".to_string(), s.clone()),
            Name::Number(n) => ("%".to_string(), n.to_string()),
        };
        prefix.push_str(&stem);
        prefix.push_str(&suffix);
        Ok(z3::Symbol::String(prefix))
    }
}

mod tests {
    use super::*;

    #[test]
    fn lookup_vars() {
        let ctx = z3::Context::new(&z3::Config::new());
        let mut varmap = VarMap::new(&ctx, 20);

        // create llvm-ir names
        let valname = Name::Name("val".to_owned());
        let boolname = Name::Number(2);

        // create corresponding Z3 values
        let valvar = varmap.new_bv_with_name(valname.clone(), 64).unwrap();
        let boolvar = varmap.new_bool_with_name(boolname.clone()).unwrap();  // these clone()s wouldn't normally be necessary but we want to compare against the original values later

        // check that looking up the llvm-ir values gives the correct Z3 ones
        assert_eq!(varmap.lookup_bv_var(&valname), valvar);
        assert_eq!(varmap.lookup_bool_var(&boolname), boolvar);
    }

    #[test]
    fn vars_are_uniqued() {
        let ctx = z3::Context::new(&z3::Config::new());
        let mut varmap = VarMap::new(&ctx, 20);
        let mut solver = crate::solver::Solver::new(&ctx);

        // create two vars with the same name
        let name = Name::Name("x".to_owned());
        let x1 = varmap.new_bv_with_name(name.clone(), 64).unwrap();
        let x2 = varmap.new_bv_with_name(name, 64).unwrap();

        // constrain with incompatible constraints
        solver.assert(&x1.bvugt(&BV::from_u64(&ctx, 2, 64)));
        solver.assert(&x2.bvult(&BV::from_u64(&ctx, 1, 64)));

        // check that we're still sat
        assert!(solver.check());

        // now repeat with integer names
        let name = Name::Number(3);
        let x1 = varmap.new_bv_with_name(name.clone(), 64).unwrap();
        let x2 = varmap.new_bv_with_name(name, 64).unwrap();
        solver.assert(&x1.bvugt(&BV::from_u64(&ctx, 2, 64)));
        solver.assert(&x2.bvult(&BV::from_u64(&ctx, 1, 64)));
        assert!(solver.check());
    }

    #[test]
    fn enforces_max_version() {
        let ctx = z3::Context::new(&z3::Config::new());

        // Create a `VarMap` with `max_version_num = 10`
        let mut varmap = VarMap::new(&ctx, 10);

        // Check that we can create 10 versions of the same `Name`
        let name = Name::Number(7);
        for _ in 0 .. 10 {
            let bv = varmap.new_bv_with_name(name.clone(), 64);
            assert!(bv.is_ok());
        }

        // Check that we can't create an 11th version of that `Name`
        let bv = varmap.new_bv_with_name(name, 64);
        assert!(bv.is_err());
    }
}
