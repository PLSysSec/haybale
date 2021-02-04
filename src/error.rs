use std::fmt;

/// Error types used throughout this crate.
///
/// The `Display` impl for `Error` will provide information about the error
/// itself. For more detailed information about the error, including the program
/// context in which it occurred, see
/// [`State.full_error_message_with_context()`](struct.State.html#method.full_error_message_with_context).
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Error {
    /// While performing an operation, we discovered the current path is unsat.
    ///
    /// This error type is used internally, but (by default) isn't exposed to consumers of `ExecutionManager`;
    /// see [`Config.squash_unsats`](config/struct.Config.html#structfield.squash_unsats).
    Unsat,
    /// The current path has exceeded the configured `loop_bound` (see [`Config`](config/struct.Config.html)).
    /// (The `usize` here indicates the value of the configured `loop_bound`.)
    LoopBoundExceeded(usize),
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
    /// Reached an LLVM `Unreachable` instruction
    UnreachableInstruction,
    /// Failed to interpret some symbolic value (`BV`) as a function pointer,
    /// because it has a possible solution (the `u64` here) which points to
    /// something that's not a function
    FailedToResolveFunctionPointer(u64),
    /// The hook for some function returned a value which didn't match the
    /// function return type: for instance, a value of the wrong size.
    /// The `String` here just describes the error
    HookReturnValueMismatch(String),
    /// Some kind of error which doesn't fall into one of the above categories.
    /// The `String` here describes the error
    OtherError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Unsat =>
                write!(f, "`Unsat`: the current state or path is unsat"),
            Error::LoopBoundExceeded(bound) =>
                write!(f, "`LoopBoundExceeded`: the current path has exceeded the configured `loop_bound`, which was {}", bound),
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
            Error::UnreachableInstruction =>
                write!(f, "`UnreachableInstruction`: Reached an LLVM 'Unreachable' instruction"),
            Error::FailedToResolveFunctionPointer(solution) =>
                write!(f, "`FailedToResolveFunctionPointer`: Can't resolve a symbolically-valued function pointer, because one possible solution for it ({:#x}) points to something that's not a function", solution),
            Error::HookReturnValueMismatch(details) =>
                write!(f, "`HookReturnValueMismatch`: {}", details),
            Error::OtherError(details) =>
                write!(f, "`OtherError`: {}", details),
        }
    }
}

impl From<Error> for String {
    fn from(e: Error) -> String {
        e.to_string() // use the Display impl
    }
}

/// A type alias for convenience, similar to how `std::io::Result` is used for I/O operations
pub type Result<T> = std::result::Result<T, Error>;

/// This Result is used for some backend operations.
/// It can return (zero or more) nonfatal warnings along with the actual `Result` value.
#[derive(Clone, Debug, PartialEq)]
pub struct BackendResult<T> {
    /// The actual result
    pub res: Result<T>,
    /// (Optional) Nonfatal warnings. These do not prevent execution from
    /// continuing on this path.
    pub warnings: Vec<String>,
}

impl<T> From<Result<T>> for BackendResult<T> {
    fn from(res: Result<T>) -> BackendResult<T> {
        BackendResult {
            res,
            warnings: Vec::new(),
        }
    }
}

impl<T> BackendResult<T> {
    /// Construct a `BackendResult` from a `Result` and the given warning.
    /// (To make one without a warning, just use `.into()`.)
    pub fn with_warn(res: Result<T>, warning: String) -> Self {
        Self {
            res,
            warnings: vec![warning],
        }
    }

    /// Construct a `BackendResult` from a `Result` and the given warnings.
    /// (To make one with a single warning, use `with_warn()`; to make one
    /// without a warning, just use `.into()`.)
    pub fn with_warns(res: Result<T>, warnings: impl IntoIterator<Item = String>) -> Self {
        Self {
            res,
            warnings: warnings.into_iter().collect(),
        }
    }

    /// Add the given warning.
    pub fn add_warn(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn is_ok(&self) -> bool {
        self.res.is_ok()
    }

    pub fn is_err(&self) -> bool {
        self.res.is_err()
    }

    /// Returns `true` if `self` has one or more warnings attached.
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Converts `self` into `Option<T>`, discarding any errors or warnings.
    pub fn ok(self) -> Option<T> {
        self.res.ok()
    }

    /// Converts `self` into `Option<Error>`, discarding any success value or
    /// warnings.
    pub fn err(self) -> Option<Error> {
        self.res.err()
    }

    /// Maps the `Ok` value, leaving an `Err` untouched, and also leaving
    /// warnings untouched.
    pub fn map<U, F>(self, op: F) -> BackendResult<U> where F: FnOnce(T) -> U {
        BackendResult {
            res: self.res.map(op),
            warnings: self.warnings,
        }
    }

    /// If `self` is `Ok`, returns `res` with all the warnings on both `self` and
    /// `res`.
    /// If `self` is `Err`, returns `self` with only its own warnings.
    pub fn and<U>(self, res: BackendResult<U>) -> BackendResult<U> {
        match self.res {
            Ok(_) => {
                let mut warnings = self.warnings;
                warnings.extend(res.warnings.into_iter());
                BackendResult {
                    res: res.res,
                    warnings,
                }
            }
            Err(err) => BackendResult {
                res: Err(err),
                warnings: self.warnings,
            }
        }
    }

    /// If `self` is `Ok`, calls `op` on it and returns the result with all the
    /// warnings generated along the way.
    /// If `self` is `Err`, returns `self` with only its own warnings.
    pub fn and_then<U, F>(self, op: F) -> BackendResult<U> where F: FnOnce(T) -> BackendResult<U> {
        match self.res {
            Ok(t) => {
                let new_res = op(t);
                let mut warnings = self.warnings;
                warnings.extend(new_res.warnings.into_iter());
                BackendResult {
                    res: new_res.res,
                    warnings,
                }
            }
            Err(err) => BackendResult {
                res: Err(err),
                warnings: self.warnings,
            }
        }
    }

    /// If `self` is `Ok`, returns `self` with only its own warnings.
    /// If `self` is `Err`, returns `res` with all the warnings on both `self`
    /// and `res`.
    pub fn or(self, res: Self) -> Self {
        match &self.res {
            Ok(_) => self,
            Err(_) => {
                let mut warnings = self.warnings;
                warnings.extend(res.warnings.into_iter());
                BackendResult {
                    res: res.res,
                    warnings,
                }
            }
        }
    }

    /// If `self` is `Ok`, returns `self` with only its own warnings.
    /// If `self` is `Err`, calls `op` on it and returns the result with all the
    /// warnings generated along the way.
    pub fn or_else<F>(self, op: F) -> Self where F: FnOnce(Error) -> Self {
        match self.res {
            Ok(_) => self,
            Err(err) => {
                let new_res = op(err);
                let mut warnings = self.warnings;
                warnings.extend(new_res.warnings.into_iter());
                BackendResult {
                    res: new_res.res,
                    warnings,
                }
            }
        }
    }

    /// Unwrap a `BackendResult` into a `Result`.
    /// Panics if any warnings were present.
    pub fn unwrap_warn(self) -> Result<T> {
        if self.warnings.is_empty() {
            self.res
        } else {
            panic!("unwrap_warn: there was a warning: {}", self.warnings[0])
        }
    }

    /// Unwrap a `BackendResult` into a `Result`, discarding any warnings.
    pub fn discard_warnings(self) -> Result<T> {
        self.res
    }
}
