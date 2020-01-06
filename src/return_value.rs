/// A simple enum describing the value returned from a function
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum ReturnValue<V> {
    /// The function or call returns this value
    Return(V),
    /// The function or call returns void
    ReturnVoid,
    /// The function or call throws this value (using the LLVM `invoke`/`resume`
    /// mechanism, which is used for e.g. C++ exceptions)
    ///
    /// (note that, unless other comments say otherwise, this is a pointer to the
    /// actual value or object thrown, not the value itself)
    Throw(V),
    /// The function or call aborts without ever returning (e.g., with a Rust
    /// panic, or by calling the C `exit()` function)
    Abort,
}
