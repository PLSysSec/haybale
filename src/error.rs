use crate::function_hooks::cpp_demangle;
use std::fmt;

/// Error types used throughout this crate
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Error {
    /// While performing an operation, we discovered the current path is unsat
    Unsat,
    /// The current path has exceeded the configured `loop_bound` (see [`Config`](struct.Config.html))
    LoopBoundExceeded,
    /// The current path has attempted to dereference a null pointer (or
    /// more precisely, a pointer for which `NULL` is a possible value)
    NullPointerDereference,
    /// Processing a call of a function with the given name, but failed to find an LLVM definition, a function hook, or a built-in handler for it
    FunctionNotFound(String),
    /// An operation attempted to coerce a `BV` more than one bit long into a `Bool`. The `String` is a text description of the `BV`, and the `u32` is its size
    BoolCoercionError(String, u32),
    /// The solver returned this processing error while evaluating a query
    SolverError(String),
    /// Encountered an LLVM instruction which is not currently supported
    UnsupportedInstruction(String),
    /// Encountered an LLVM instruction which was malformed, or at least didn't conform to our expected invariants
    MalformedInstruction(String),
    /// Some kind of error which doesn't fall into one of the above categories
    OtherError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Unsat =>
                write!(f, "`Unsat`: the current state or path is unsat"),
            Error::LoopBoundExceeded =>
                write!(f, "`LoopBoundExceeded`: the current path has exceeded the configured `loop_bound`"),
            Error::NullPointerDereference =>
                write!(f, "`NullPointerDereference`: the current path has attempted to dereference a null pointer"),
            Error::FunctionNotFound(funcname) => {
                write!(f, "`FunctionNotFound`: encountered a call of a function named {:?}", funcname)?;
                match cpp_demangle(funcname) {
                    Some(demangled) => write!(f, " (C++ demangled: {:?})", demangled)?,
                    None => {},
                };
                if let Ok(demangled) = rustc_demangle::try_demangle(funcname) {
                    write!(f, " (Rust demangled: \"{:#}\")", demangled)?;
                }
                write!(f, ", but failed to find an LLVM definition, a function hook, or a built-in handler for it")?;
                Ok(())
            },
            Error::BoolCoercionError(bv, size) =>
                write!(f, "`BoolCoercionError`: can't coerce a BV {} bits long into a Bool; the BV was {}", size, bv),
            Error::SolverError(details) =>
                write!(f, "`SolverError`: the solver returned this processing error while evaluating a query: {}", details),
            Error::UnsupportedInstruction(details) =>
                write!(f, "`UnsupportedInstruction`: encountered an LLVM instruction which is not currently supported: {}", details),
            Error::MalformedInstruction(details) =>
                write!(f, "`MalformedInstruction`: encountered an LLVM instruction which was malformed, or at least didn't conform to our expected invariants: {}", details),
            Error::OtherError(details) =>
                write!(f, "`OtherError`: {}", details),
        }
    }
}

/// A type alias for convenience, similar to how `std::io::Result` is used for I/O operations
pub type Result<T> = std::result::Result<T, Error>;
