use std::fmt;

/// Error types used throughout this crate.
///
/// The `Display` impl for `Error` will provide information about the error
/// itself. For more detailed information about the error, including the program
/// context in which it occurred, see
/// [`State.full_error_message_with_context()`](struct.State.html#method.full_error_message_with_context).
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Error {
    /// While performing an operation, we discovered the current path is unsat
    Unsat,
    /// The current path has exceeded the configured `loop_bound` (see [`Config`](config/struct.Config.html))
    LoopBoundExceeded,
    /// The current path has attempted to dereference a null pointer (or
    /// more precisely, a pointer for which `NULL` is a possible value)
    NullPointerDereference,
    /// Processing a call of a function with the given name, but failed to find an LLVM definition, a function hook, or a built-in handler for it
    FunctionNotFound(String),
    /// The solver returned this processing error while evaluating a query.
    /// Often, this is a timeout; see [`Config.solver_query_timeout`](config/struct.Config.html#structfield.solver_query_timeout)
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
            Error::FunctionNotFound(funcname) =>
                write!(f, "`FunctionNotFound`: encountered a call of a function named {:?}, but failed to find an LLVM definition, a function hook, or a built-in handler for it", funcname),
            Error::SolverError(details) =>
                write!(f, "`SolverError`: the solver returned this error while evaluating a query: {}", details),
            Error::UnsupportedInstruction(details) =>
                write!(f, "`UnsupportedInstruction`: encountered an LLVM instruction which is not currently supported: {}", details),
            Error::MalformedInstruction(details) =>
                write!(f, "`MalformedInstruction`: encountered an LLVM instruction which was malformed, or at least didn't conform to our expected invariants: {}", details),
            Error::OtherError(details) =>
                write!(f, "`OtherError`: {}", details),
        }
    }
}

impl From<Error> for String {
    fn from(e: Error) -> String {
        e.to_string()  // use the Display impl
    }
}

/// A type alias for convenience, similar to how `std::io::Result` is used for I/O operations
pub type Result<T> = std::result::Result<T, Error>;
