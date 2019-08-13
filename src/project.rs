use llvm_ir::{Function, Module};
use llvm_ir::module::{GlobalAlias, GlobalVariable};
use std::fs::DirEntry;
use std::io;
use std::path::Path;

/// A `Project` is a collection of LLVM code to be explored,
/// consisting of one or more LLVM modules
pub struct Project {
    modules: Vec<Module>,
}

impl Project {
    /// Construct a new `Project` from a path to an LLVM bitcode file
    pub fn from_bc_path(path: impl AsRef<Path>) -> Result<Self, String> {
        Ok(Self {
            modules: vec![Module::from_bc_path(path)?],
        })
    }

    /// Construct a new `Project` from multiple LLVM bitcode files
    pub fn from_bc_paths<P>(paths: impl IntoIterator<Item = P>) -> Result<Self, String> where P: AsRef<Path> {
        Ok(Self {
            modules: paths
                .into_iter()
                .map(|p| Module::from_bc_path(p.as_ref()))
                .collect::<Vec<Result<_,_>>>()
                .into_iter()
                .collect::<Result<Vec<_>,_>>()?,  // The final into_iter().collect() converts Vec<Result<T, E>> to Result<Vec<T>, E>, failing if any of the items were Err
        })
    }

    /// Construct a new `Project` from a path to a directory containing
    /// LLVM bitcode files.
    ///
    /// All files in the directory which have the extension `extn` will
    /// be parsed and added to the `Project`.
    pub fn from_bc_dir(path: impl AsRef<Path>, extn: &str) -> Result<Self, io::Error> {
        Ok(Self {
            modules: Self::modules_from_bc_dir(path, extn)?,
        })
    }

    /// Add the code in the given LLVM bitcode file to the `Project`
    pub fn add_bc_path(&mut self, path: impl AsRef<Path>) -> Result<(), String> {
        let module = Module::from_bc_path(path)?;
        self.modules.push(module);
        Ok(())
    }

    /// Add the code in the given directory to the `Project`.
    /// See [`Project::from_bc_dir()`](struct.Project.html#method.from_bc_dir).
    pub fn add_bc_dir(&mut self, path: impl AsRef<Path>, extn: &str) -> Result<(), io::Error> {
        let modules = Self::modules_from_bc_dir(path, extn)?;
        self.modules.extend(modules);
        Ok(())
    }

    /// Iterate over all `Function`s in the `Project`.
    /// Gives pairs which also indicate the `Module` the `Function` is defined in.
    pub fn all_functions(&self) -> impl Iterator<Item = (&Function, &Module)> {
        self.modules.iter().map(|m| m.functions.iter().zip(std::iter::repeat(m))).flatten()
    }

    /// Iterate over all `GlobalVariable`s in the `Project`.
    /// Gives pairs which also indicate the `Module` the `GlobalVariable` comes from.
    pub fn all_global_vars(&self) -> impl Iterator<Item = (&GlobalVariable, &Module)> {
        self.modules.iter().map(|m| m.global_vars.iter().zip(std::iter::repeat(m))).flatten()
    }

    /// Iterate over all `GlobalAlias`es in the `Project`.
    /// Gives pairs which also indicate the `Module` the `GlobalAlias` comes from.
    pub fn all_global_aliases(&self) -> impl Iterator<Item = (&GlobalAlias, &Module)> {
        self.modules.iter().map(|m| m.global_aliases.iter().zip(std::iter::repeat(m))).flatten()
    }

    /// Get the names of the LLVM modules which have been parsed and loaded into
    /// the `Project`
    pub fn active_module_names(&self) -> impl Iterator<Item = &String> {
        self.modules.iter().map(|m| &m.name)
    }

    /// Search the project for a function with the given name.
    /// If a matching function is found, return both it and the module it was
    /// found in.
    pub fn get_func_by_name<'p>(&'p self, name: &str) -> Option<(&'p Function, &'p Module)> {
        let mut retval = None;
        for module in &self.modules {
            if let Some(f) = module.get_func_by_name(name) {
                match retval {
                    None => retval = Some((f, module)),
                    Some(_) => panic!("Multiple functions found with name {:?}", name),
                };
            }
        }
        retval
    }

    fn modules_from_bc_dir(path: impl AsRef<Path>, extn: &str) -> Result<Vec<Module>, io::Error> {
        // warning, we use both `Iterator::map` and `Result::map` in here, and it's easy to get them confused
        path
            .as_ref()
            .read_dir()?
            .filter(|entry| match entry_is_dir(entry) {
                Some(true) => false,  // filter out if it is a directory
                Some(false) => true,  // leave in the ones that are non-directories
                None => true,  // also leave in errors, because we want to know about those
            })
            .map(|entry| entry.map(|entry| entry.path()))
            .filter(|path| match path {
                Ok(path) => match path.extension() {
                    Some(e) => e == extn,
                    None => false,  // filter out if it has no extension
                },
                Err(_) => true,  // leave in errors, because we want to know about those
            })
            .map(|path| path.and_then(|path| Module::from_bc_path(path)
                .map_err(|s| io::Error::new(io::ErrorKind::Other, s))))
            .collect::<Vec<Result<_,_>>>()  // Turns Iterator<Item = Result<_, _>> into Vec<Result<_, _>>
            .into_iter()
            .collect()  // Turns Vec<Result<T, E>> into Result<Vec<T>, E>, failing if any of the results were Err
    }

    /// For testing only: construct a `Project` directly from a `Module`
    #[cfg(test)]
    pub(crate) fn from_module(module: Module) -> Self {
        Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_file_project() {
        let proj = Project::from_bc_path(Path::new("tests/bcfiles/basic.bc"))
            .unwrap_or_else(|e| panic!("Failed to create project: {}", e));
        let (func, module) = proj.get_func_by_name("no_args_zero").expect("Failed to find function");
        assert_eq!(&func.name, "no_args_zero");
        assert_eq!(&module.name, "tests/bcfiles/basic.bc");
    }

    #[test]
    fn double_file_project() {
        let proj = Project::from_bc_paths(vec!["tests/bcfiles/basic.bc", "tests/bcfiles/loop.bc"].into_iter().map(Path::new))
            .unwrap_or_else(|e| panic!("Failed to create project: {}", e));
        let (func, module) = proj.get_func_by_name("no_args_zero").expect("Failed to find function");
        assert_eq!(&func.name, "no_args_zero");
        assert_eq!(&module.name, "tests/bcfiles/basic.bc");
        let (func, module) = proj.get_func_by_name("while_loop").expect("Failed to find function");
        assert_eq!(&func.name, "while_loop");
        assert_eq!(&module.name, "tests/bcfiles/loop.bc");
    }

    #[test]
    fn whole_directory_project() {
        let proj = Project::from_bc_dir("tests/bcfiles", "bc").unwrap_or_else(|e| panic!("Failed to create project: {}", e));
        let (func, module) = proj.get_func_by_name("no_args_zero").expect("Failed to find function");
        assert_eq!(&func.name, "no_args_zero");
        assert_eq!(&module.name, "tests/bcfiles/basic.bc");
        let (func, module) = proj.get_func_by_name("while_loop").expect("Failed to find function");
        assert_eq!(&func.name, "while_loop");
        assert_eq!(&module.name, "tests/bcfiles/loop.bc");
    }
}
