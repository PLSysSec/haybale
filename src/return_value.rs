/// A simple enum describing the value returned from a function
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ReturnValue<V> {
    /// The function or call returns this value
    Return(V),
    /// The function or call returns void
    ReturnVoid,
}
