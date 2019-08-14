#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Error {
    /// While performing an operation, we discovered the current path is unsat
    Unsat,
    /// The current path has exceeded the configured `loop_bound` (see [`Config`](struct.Config.html))
    LoopBoundExceeded,
    /// The solver returned this processing error while evaluating a query
    SolverError(String),
    /// Some kind of error which doesn't fall into one of the above categories
    OtherError(String),
}

/// A type alias for convenience, similar to how `std::io::Result` is used for I/O operations
pub type Result<T> = std::result::Result<T, Error>;
