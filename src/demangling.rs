#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Demangling {
    /// Don't try to demangle
    None,
    /// Try to demangle using the C++ demangler (suitable for `Project`s containing C++ code).
    /// Names that fail to demangle will simply be printed as-is.
    CPP,
    /// Try to demangle using the Rust demangler (suitable for `Project`s containing Rust code).
    /// Names that fail to demangle will simply be printed as-is.
    Rust,
}

impl Demangling {
    /// Attempts to demangle the given function name, as appropriate based on the
    /// `Demangling` setting.
    pub fn maybe_demangle(&self, funcname: &str) -> String {
        match self {
            Demangling::None => funcname.to_owned(),
            Demangling::CPP => cpp_demangle_or_id(funcname),
            Demangling::Rust => rust_demangle_or_id(funcname),
        }
    }
}

/// Helper function to demangle function names with the C++ demangler.
///
/// Returns `Some` if successfully demangled, or `None` if any error occurs
/// (for instance, if `funcname` isn't a valid C++ mangled name)
pub(crate) fn try_cpp_demangle(funcname: &str) -> Option<String> {
    let opts = cpp_demangle::DemangleOptions {
        no_params: true,
    };
    cpp_demangle::Symbol::new(funcname).ok().and_then(|sym| sym.demangle(&opts).ok())
}

/// Like `try_cpp_demangle()`, but just returns the input string unmodified in
/// the case of any error, rather than returning `None`.
pub(crate) fn cpp_demangle_or_id(funcname: &str) -> String {
    try_cpp_demangle(funcname).unwrap_or_else(|| funcname.to_owned())
}

/// Helper function to demangle function names with the Rust demangler.
///
/// Returns `Some` if successfully demangled, or `None` if any error occurs
/// (for instance, if `funcname` isn't a valid Rust mangled name)
pub(crate) fn try_rust_demangle(funcname: &str) -> Option<String> {
    rustc_demangle::try_demangle(funcname).ok().map(|demangled|
        format!("{:#}", demangled)
    )
}

/// Like `try_rust_demangle()`, but just returns the input string unmodified in
/// the case of any error, rather than returning `None`.
pub(crate) fn rust_demangle_or_id(funcname: &str) -> String {
    format!("{:#}", rustc_demangle::demangle(funcname))
}
