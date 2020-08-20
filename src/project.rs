use crate::demangling::try_cpp_demangle;
use llvm_ir::module::{GlobalAlias, GlobalVariable};
use llvm_ir::{Function, Module, Type};
use log::{info, warn};
use rustc_demangle::demangle;
use std::fs::DirEntry;
use std::io;
use std::path::Path;
use std::sync::{Arc, RwLock};

/// A `Project` is a collection of LLVM code to be explored,
/// consisting of one or more LLVM modules.
pub struct Project {
    modules: Vec<Module>,
    pointer_size_bits: u32,
}

impl Project {
    /// Construct a new `Project` from a path to an LLVM bitcode file
    pub fn from_bc_path(path: impl AsRef<Path>) -> Result<Self, String> {
        info!("Parsing bitcode in file {}", path.as_ref().display());
        let module = Module::from_bc_path(path)?;
        Ok(Self {
            pointer_size_bits: get_ptr_size(&module),
            modules: vec![module],
        })
    }

    /// Construct a new `Project` from multiple LLVM bitcode files
    pub fn from_bc_paths<P>(paths: impl IntoIterator<Item = P>) -> Result<Self, String>
    where
        P: AsRef<Path>,
    {
        info!("Parsing bitcode from specified files");
        let (modules, ptr_sizes): (Vec<Module>, Vec<u32>) = paths
            .into_iter()
            .map(|p| Module::from_bc_path(p.as_ref()))
            .map(|r| {
                r.map(|m| {
                    let ptr_size = get_ptr_size(&m);
                    (m, ptr_size)
                })
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .unzip();
        let mut ptr_sizes = ptr_sizes.into_iter();
        let pointer_size_bits = ptr_sizes
            .next()
            .expect("Project::from_bc_paths: at least one path is required");
        assert!(
            ptr_sizes.all(|size| size == pointer_size_bits),
            "Project::from_bc_paths: modules have conflicting pointer sizes"
        );
        Ok(Self {
            modules,
            pointer_size_bits,
        })
    }

    /// Construct a new `Project` from a path to a directory containing
    /// LLVM bitcode files.
    ///
    /// All files in the directory which have the extension `extn` will
    /// be parsed and added to the `Project`.
    pub fn from_bc_dir(path: impl AsRef<Path>, extn: &str) -> Result<Self, io::Error> {
        info!("Parsing bitcode from directory {}", path.as_ref().display());
        let (modules, pointer_size_bits) = Self::modules_from_bc_dir(path, extn, |_| false)?;
        Ok(Self {
            modules,
            pointer_size_bits,
        })
    }

    /// Construct a new `Project` from a path to a directory containing LLVM
    /// bitcode files.
    ///
    /// All files in the directory which have the extension `extn`, except those
    /// for which the provided `exclude` closure returns `true`, will be parsed
    /// and added to the `Project`.
    pub fn from_bc_dir_with_blacklist(
        path: impl AsRef<Path>,
        extn: &str,
        exclude: impl Fn(&Path) -> bool,
    ) -> Result<Self, io::Error> {
        info!(
            "Parsing bitcode from directory {} with blacklist",
            path.as_ref().display()
        );
        let (modules, pointer_size_bits) = Self::modules_from_bc_dir(path, extn, exclude)?;
        Ok(Self {
            modules,
            pointer_size_bits,
        })
    }

    /// Add the code in the given LLVM bitcode file to the `Project`
    pub fn add_bc_path(&mut self, path: impl AsRef<Path>) -> Result<(), String> {
        info!("Parsing bitcode in file {}", path.as_ref().display());
        let module = Module::from_bc_path(path)?;
        assert_eq!(
            get_ptr_size(&module),
            self.pointer_size_bits,
            "Modules have conflicting pointer sizes"
        );
        self.modules.push(module);
        Ok(())
    }

    /// Add the code in the given directory to the `Project`.
    /// See [`Project::from_bc_dir()`](struct.Project.html#method.from_bc_dir).
    pub fn add_bc_dir(&mut self, path: impl AsRef<Path>, extn: &str) -> Result<(), io::Error> {
        info!("Parsing bitcode from directory {}", path.as_ref().display());
        let (modules, pointer_size_bits) = Self::modules_from_bc_dir(path, extn, |_| false)?;
        assert_eq!(
            pointer_size_bits, self.pointer_size_bits,
            "Modules have conflicting pointer sizes"
        );
        self.modules.extend(modules);
        Ok(())
    }

    /// Add the code in the given directory, except for blacklisted files, to the `Project`.
    /// See [`Project::from_bc_dir_with_blacklist()`](struct.Project.html#method.from_bc_dir_with_blacklist).
    pub fn add_bc_dir_with_blacklist(
        &mut self,
        path: impl AsRef<Path>,
        extn: &str,
        exclude: impl Fn(&Path) -> bool,
    ) -> Result<(), io::Error> {
        info!(
            "Parsing bitcode from directory {} with blacklist",
            path.as_ref().display()
        );
        let (modules, pointer_size_bits) = Self::modules_from_bc_dir(path, extn, exclude)?;
        assert_eq!(
            pointer_size_bits, self.pointer_size_bits,
            "Modules have conflicting pointer sizes"
        );
        self.modules.extend(modules);
        Ok(())
    }

    /// Get the pointer size used by the `Project`, in bits.
    /// E.g., this will be `64` if the LLVM bitcode was compiled for a 64-bit platform.
    pub fn pointer_size_bits(&self) -> u32 {
        self.pointer_size_bits
    }

    /// Iterate over all `Function`s in the `Project`.
    /// Gives pairs which also indicate the `Module` the `Function` is defined in.
    pub fn all_functions(&self) -> impl Iterator<Item = (&Function, &Module)> {
        self.modules
            .iter()
            .map(|m| m.functions.iter().zip(std::iter::repeat(m)))
            .flatten()
    }

    /// Iterate over all `GlobalVariable`s in the `Project`.
    /// Gives pairs which also indicate the `Module` the `GlobalVariable` comes from.
    pub fn all_global_vars(&self) -> impl Iterator<Item = (&GlobalVariable, &Module)> {
        self.modules
            .iter()
            .map(|m| m.global_vars.iter().zip(std::iter::repeat(m)))
            .flatten()
    }

    /// Iterate over all `GlobalAlias`es in the `Project`.
    /// Gives pairs which also indicate the `Module` the `GlobalAlias` comes from.
    pub fn all_global_aliases(&self) -> impl Iterator<Item = (&GlobalAlias, &Module)> {
        self.modules
            .iter()
            .map(|m| m.global_aliases.iter().zip(std::iter::repeat(m)))
            .flatten()
    }

    /// Iterate over all named struct types in the `Project`.
    /// Gives triplets `(name, Type, Module)` which indicate the struct's name,
    /// type, and which module it comes from.
    ///
    /// If the `Type` in the triplet is `None`, that means the struct type is
    /// opaque; see
    /// [LLVM 9 docs on Opaque Structure Types](https://releases.llvm.org/9.0.0/docs/LangRef.html#t-opaque).
    pub fn all_named_struct_types(&self) -> impl Iterator<Item = (&String, Option<Type>, &Module)> {
        self.modules
            .iter()
            .map(|m| {
                m.named_struct_types
                    .iter()
                    .map(|(name, opt)| (name, opt.as_ref().map(|arc| arc.read().unwrap().clone())))
                    .zip(std::iter::repeat(m))
                    .map(|((name, opt), m)| (name, opt, m))
            })
            .flatten()
    }

    /// Get the names of the LLVM modules which have been parsed and loaded into
    /// the `Project`
    pub fn active_module_names(&self) -> impl Iterator<Item = &String> {
        self.modules.iter().map(|m| &m.name)
    }

    pub(crate) fn module_source_file_names(&self) -> impl Iterator<Item = &String> {
        self.modules.iter().map(|m| &m.source_file_name)
    }

    /// Search the project for a function with the given name.
    /// If a matching function is found, return both it and the module it was
    /// found in.
    ///
    /// For projects containing C++ or Rust code, you can pass either the mangled
    /// or demangled function name.
    ///
    /// If you have a `State` handy, you may want to use
    /// `state.get_func_by_name()` instead, which will get the appropriate
    /// (potentially module-private) definition based on the current LLVM module.
    pub fn get_func_by_name<'p>(&'p self, name: &str) -> Option<(&'p Function, &'p Module)> {
        let mut retval = None;
        for module in &self.modules {
            if let Some(f) = module.get_func_by_name(name) {
                match retval {
                    None => retval = Some((f, module)),
                    Some((_, retmod)) => panic!("Multiple functions found with name {:?}: one in module {:?}, another in module {:?}", name, retmod.name, module.name),
                };
            }
        }
        if retval.is_some() {
            return retval;
        }
        // if we get to this point, we haven't found the function normally; maybe we were
        // given a Rust demangled name
        for module in &self.modules {
            if let Some(f) = module
                .functions
                .iter()
                .find(|func| demangle(&func.name).to_string() == name)
            {
                match retval {
                    None => retval = Some((f, module)),
                    Some((_, retmod)) => panic!("Multiple functions found with demangled name {:?}: one in module {:?}, another in module {:?}", name, retmod.name, module.name),
                };
            }
        }
        if retval.is_some() {
            return retval;
        }
        // if we get to this point, we still haven't found the function; try
        // stripping the trailing hash value from the Rust mangled name
        for module in &self.modules {
            if let Some(f) = module
                .functions
                .iter()
                .find(|func| format!("{:#}", demangle(&func.name)) == name)
            {
                match retval {
                    None => retval = Some((f, module)),
                    Some((_, retmod)) => panic!("Multiple functions found with demangled name {:?}: one in module {:?}, another in module {:?}", name, retmod.name, module.name),
                };
            }
        }
        if retval.is_some() {
            return retval;
        }
        // if we get to this point, we still haven't found the function;
        // maybe we were given a C++ demangled name
        for module in &self.modules {
            if let Some(f) = module
                .functions
                .iter()
                .find(|func| try_cpp_demangle(&func.name).as_deref() == Some(name))
            {
                match retval {
                    None => retval = Some((f, module)),
                    Some((_, retmod)) => panic!("Multiple functions found with demangled name {:?}: one in module {:?}, another in module {:?}", name, retmod.name, module.name),
                };
            }
        }
        retval
    }

    /// Search the project for a named struct type with the given name.
    /// If a matching named struct type is found, return both it and the module
    /// it was found in.
    ///
    /// If `None` is returned, then no named struct type with the given name was
    /// found in the project.
    ///
    /// If `Some(None, <module>)` is returned, that means the struct type is
    /// opaque; see
    /// [LLVM 9 docs on Opaque Structure Types](https://releases.llvm.org/9.0.0/docs/LangRef.html#t-opaque).
    ///
    /// If the named struct type is defined in multiple modules in the `Project`,
    /// this returns one of them arbitrarily. However, it will only return
    /// `Some(None, <module>)` if _all_ definitions are opaque; that is, it will
    /// attempt to return some non-opaque definition if one exists, before
    /// returning an opaque definition.
    pub fn get_named_struct_type_by_name<'p>(
        &'p self,
        name: &str,
    ) -> Option<(&'p Option<Arc<RwLock<Type>>>, &'p Module)> {
        let mut retval: Option<(&'p Option<Arc<RwLock<Type>>>, &'p Module)> = None;
        for module in &self.modules {
            if let Some(t) = module
                .named_struct_types
                .iter()
                .find(|&(n, _)| n == name)
                .map(|(_, t)| t)
            {
                match (retval, t) {
                    (None, t) => retval = Some((t, module)), // first definition we've found: this is the new candidate to return
                    (Some(_), None) => {}, // this is an opaque definition, and we previously found some other definition (opaque or not); do nothing
                    (Some((None, _)), t @ Some(_)) => retval = Some((t, module)), // found an actual definition, replace the previous opaque definition
                    (Some((Some(arc1), retmod)), Some(arc2)) => {
                        // duplicate non-opaque definitions: ensure they completely agree
                        let def1: &Type = &arc1.read().unwrap();
                        let def2: &Type = &arc2.read().unwrap();
                        if def1 != def2 {
                            // if they don't agree, we merely warn rather than panicking.
                            // For instance, if the struct contains an anonymous union as one of its members,
                            // duplicate definitions of the struct will appear to conflict due to the
                            // anonymous union being numbered differently in the two modules, even if the
                            // union has the same contents in both modules.
                            warn!("Multiple named struct types found with name {:?}: the first was from module {:?}, the other was from module {:?}.\n  First definition: {:?}\n  Second definition: {:?}\n  We will (arbitrarily) use the first one.", name, retmod.name, module.name, def1, def2);
                        // then we'll do nothing, leaving (arbitrarily) the first definition we found
                        } else {
                            // do nothing, leaving (arbitrarily) the first definition we found
                        }
                    },
                };
            }
        }
        retval
    }

    /// Given a `NamedStructType`, get the `StructType` corresponding to the
    /// actual definition of that `NamedStructType`. This may be as simple as
    /// upgrading the weak reference, but in the case of opaque struct types may
    /// involve searching the `Project` for a definition of the relevant struct.
    ///
    /// Returns `None` if the struct is fully opaque, meaning it has no
    /// definition in the `Project`.
    pub fn get_inner_struct_type_from_named(&self, ty: &Type) -> Option<Arc<RwLock<Type>>> {
        match ty {
            Type::NamedStructType { name, ty } => match &ty.as_ref() {
                Some(ty) => Some(ty.upgrade().expect("Failed to upgrade weak reference")),
                None => {
                    // This is an opaque struct definition. Try to find a non-opaque definition of the same struct.
                    match self.get_named_struct_type_by_name(name).unwrap_or_else(|| panic!("Have a struct with name {:?}, but no struct of that name found in the Project", name)) {
                        (Some(arc), _) => Some(arc.clone()),
                        (None, _) => None,
                    }
                }
            },
            _ => panic!("Project::get_inner_struct_type_from_named: called with a Type which is not a NamedStructType: {:?}", ty),
        }
    }

    /// Returns the modules and the pointer size
    fn modules_from_bc_dir(
        path: impl AsRef<Path>,
        extn: &str,
        exclude: impl Fn(&Path) -> bool,
    ) -> Result<(Vec<Module>, u32), io::Error> {
        // warning, we use both `Iterator::map` and `Result::map` in here, and it's easy to get them confused
        let (modules, ptr_sizes): (Vec<Module>, Vec<u32>) = path
            .as_ref()
            .read_dir()?
            .filter(|entry| match entry_is_dir(entry) {
                Some(true) => false, // filter out if it is a directory
                Some(false) => true, // leave in the ones that are non-directories
                None => true,        // also leave in errors, because we want to know about those
            })
            .map(|entry| entry.map(|entry| entry.path()))
            .filter(|path| match path {
                Ok(path) => match path.extension() {
                    Some(e) => e == extn && !exclude(path),
                    None => false, // filter out if it has no extension
                },
                Err(_) => true, // leave in errors, because we want to know about those
            })
            .map(|path| {
                path.and_then(|path| {
                    Module::from_bc_path(path).map_err(|s| io::Error::new(io::ErrorKind::Other, s))
                })
            })
            .map(|r| {
                r.map(|m| {
                    let ptr_size = get_ptr_size(&m);
                    (m, ptr_size)
                })
            })
            .collect::<Result<Vec<(Module, u32)>, _>>()?
            .into_iter()
            .unzip();
        let mut ptr_sizes = ptr_sizes.into_iter();
        let pointer_size_bits = ptr_sizes.next().expect("at least one path is required");
        assert!(
            ptr_sizes.all(|size| size == pointer_size_bits),
            "modules have conflicting pointer sizes"
        );
        Ok((modules, pointer_size_bits))
    }

    /// For testing only: construct a `Project` directly from a `Module`
    #[cfg(test)]
    pub(crate) fn from_module(module: Module) -> Self {
        Self {
            pointer_size_bits: get_ptr_size(&module),
            modules: vec![module],
        }
    }
}

/// Returns `Some(true)` if the entry is a directory, `Some(false)` if the entry
/// is not a directory, and `None` if there was an I/O error in trying to make
/// the determination, or if the original `entry` was an `Err`.
fn entry_is_dir(entry: &io::Result<DirEntry>) -> Option<bool> {
    match entry {
        Ok(entry) => entry.file_type().map(|ft| ft.is_dir()).ok(),
        Err(_) => None,
    }
    // one-liner for this function:
    // entry.as_ref().ok().and_then(|entry| entry.file_type().map(|ft| ft.is_dir()).ok())
}

/// Extracts the pointer width from an LLVM module, or returns a default of 64 bits
/// if the width is not specified.
///
/// See [LLVM 9 docs on Data Layout](https://releases.llvm.org/10.0.0/docs/LangRef.html#data-layout)
//
// This function originally thanks to Hudson Ayers (github.com/hudson-ayers)
fn get_ptr_size(module: &Module) -> u32 {
    for spec in module.data_layout.split('-') {
        if spec.chars().nth(0) != Some('p') {
            continue;
        }
        let colon_idx = spec
            .find(':')
            .expect("datalayout 'p' specification has no colon");
        let addr_space_num = spec[1 .. colon_idx].parse::<u32>().unwrap_or(0); // if not specified, the address space defaults to 0
        if addr_space_num == 0 { // we are only looking for a specification for the default address space
            return spec[colon_idx + 1 .. colon_idx + 3]
                .parse::<u32>()
                .expect("Failed to parse pointer size");
        }
    }
    64 // no pointer size spec explicitly specified, so LLVM defaults to 64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_file_project() {
        let proj = Project::from_bc_path(Path::new("tests/bcfiles/basic.bc"))
            .unwrap_or_else(|e| panic!("Failed to create project: {}", e));
        assert_eq!(proj.pointer_size_bits(), 64);
        let (func, module) = proj
            .get_func_by_name("no_args_zero")
            .expect("Failed to find function");
        assert_eq!(&func.name, "no_args_zero");
        assert_eq!(&module.name, "tests/bcfiles/basic.bc");
    }

    #[test]
    fn double_file_project() {
        let proj = Project::from_bc_paths(
            vec!["tests/bcfiles/basic.bc", "tests/bcfiles/loop.bc"]
                .into_iter()
                .map(Path::new),
        )
        .unwrap_or_else(|e| panic!("Failed to create project: {}", e));
        assert_eq!(proj.pointer_size_bits(), 64);
        let (func, module) = proj
            .get_func_by_name("no_args_zero")
            .expect("Failed to find function");
        assert_eq!(&func.name, "no_args_zero");
        assert_eq!(&module.name, "tests/bcfiles/basic.bc");
        let (func, module) = proj
            .get_func_by_name("while_loop")
            .expect("Failed to find function");
        assert_eq!(&func.name, "while_loop");
        assert_eq!(&module.name, "tests/bcfiles/loop.bc");
    }

    #[test]
    fn whole_directory_project() {
        let proj = Project::from_bc_dir("tests/bcfiles", "bc")
            .unwrap_or_else(|e| panic!("Failed to create project: {}", e));
        assert_eq!(proj.pointer_size_bits(), 64);
        let (func, module) = proj
            .get_func_by_name("no_args_zero")
            .expect("Failed to find function");
        assert_eq!(&func.name, "no_args_zero");
        assert_eq!(&module.name, "tests/bcfiles/basic.bc");
        let (func, module) = proj
            .get_func_by_name("while_loop")
            .expect("Failed to find function");
        assert_eq!(&func.name, "while_loop");
        assert_eq!(&module.name, "tests/bcfiles/loop.bc");
    }

    #[test]
    fn whole_directory_project_with_blacklist() {
        let proj = Project::from_bc_dir_with_blacklist("tests/bcfiles", "bc", |path| {
            path.file_stem().unwrap() == "basic"
        })
        .unwrap_or_else(|e| panic!("Failed to create project: {}", e));
        assert_eq!(proj.pointer_size_bits(), 64);
        proj.get_func_by_name("while_loop")
            .expect("Failed to find function while_loop, which should be present");
        assert!(proj.get_func_by_name("no_args_zero").is_none(), "Found function no_args_zero, which is from a file that should have been blacklisted out");
    }

    #[test]
    fn project_for_32bit_target() {
        let proj = Project::from_bc_path("tests/bcfiles/32bit/basic_rust.bc")
            .unwrap_or_else(|e| panic!("Failed to create project: {}", e));
        assert_eq!(proj.pointer_size_bits(), 32);
        let (_, module) = proj
            .get_func_by_name("basic_rust::ez")
            .expect("Failed to find function");
        assert_eq!(&module.name, "tests/bcfiles/32bit/basic_rust.bc");
    }
}
