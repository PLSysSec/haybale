use std::collections::HashMap;
use std::fmt;
use z3::ast::{Ast, BV, Bool};
use log::debug;

use llvm_ir::Name;

pub struct VarMap<'ctx> {
    vmap: HashMap<Name, BVorBool<'ctx>>,
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

impl<'ctx> VarMap<'ctx> {
    pub fn new() -> Self {
        Self {
            vmap: HashMap::new(),
        }
    }

    /// Associate the given name with the given `BV`
    pub fn add_bv_var(&mut self, name: Name, bv: BV<'ctx>) {
        debug!("Adding var {:?} = {}", name, bv);
        self.vmap.insert(name, bv.into());
    }

    /// Associate the given name with the given `Bool`
    pub fn add_bool_var(&mut self, name: Name, b: Bool<'ctx>) {
        debug!("Adding var {:?} = {}", name, b);
        self.vmap.insert(name, b.into());
    }

    /// Look up the `BV` previously created for the given value
    pub fn lookup_bv_var(&self, name: &Name) -> &BV<'ctx> {
        debug!("Looking up var {:?}", name);
        self.vmap.get(name).unwrap_or_else(|| {
            let keys: Vec<&Name> = self.vmap.keys().collect();
            panic!("Failed to find var {:?} in map with keys {:?}", name, keys);
        }).as_bv()
    }

    /// Look up the `Bool` previously created for the given value
    pub fn lookup_bool_var(&self, name: &Name) -> &Bool<'ctx> {
        debug!("Looking up var {:?}", name);
        self.vmap.get(name).unwrap_or_else(|| {
            let keys: Vec<&Name> = self.vmap.keys().collect();
            panic!("Failed to find var {:?} in map with keys {:?}", name, keys);
        }).as_bool()
    }
}

mod tests {
    use super::*;

    #[test]
    fn lookup_vars() {
        let ctx = z3::Context::new(&z3::Config::new());
        let mut varmap = VarMap::new();

        // create llvm-ir values
        let val = Name::Name("val".to_owned());
        let boolval = Name::Number(2);

        // create Z3 values
        let x = BV::new_const(&ctx, "x", 64);
        let boolvar = Bool::new_const(&ctx, "bool");

        // associate llvm-ir values with Z3 values
        varmap.add_bv_var(val.clone(), x.clone());  // these clone()s wouldn't normally be necessary but we want to compare against the original values later
        varmap.add_bool_var(boolval.clone(), boolvar.clone());

        // check that looking up the llvm-ir values gives the correct Z3 ones
        assert_eq!(varmap.lookup_bv_var(&val), &x);
        assert_eq!(varmap.lookup_bool_var(&boolval), &boolvar);
    }
}
