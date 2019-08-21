#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Error {
    /// While performing an operation, we discovered the current path is unsat
    Unsat,
    /// The current path has exceeded the configured `loop_bound` (see [`Config`](struct.Config.html))
    LoopBoundExceeded,
    /// Processing a call of a function with the given name, but failed to find an LLVM definition, a function hook, or a built-in handler for it
    FunctionNotFound(String),
    /// An operation attempted to coerce a `BV` more than one bit long into a `Bool`
    BoolCoercionError(String),
    /// The solver returned this processing error while evaluating a query
    SolverError(String),
    /// Encountered an LLVM instruction which is not currently supported
    UnsupportedInstruction(String),
    /// Encountered an LLVM instruction which was malformed, or at least didn't conform to our expected invariants
    MalformedInstruction(String),
    /// Some kind of error which doesn't fall into one of the above categories
    OtherError(String),
}

/// A type alias for convenience, similar to how `std::io::Result` is used for I/O operations
pub type Result<T> = std::result::Result<T, Error>;
