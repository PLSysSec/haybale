#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ParameterVal {
    /// The parameter can have any value whatsoever. (The analysis will
    /// effectively consider all possible values.)
    Unconstrained,
    /// The parameter will have this exact value.
    ExactValue(u64),
    /// The parameter can have any value in this range (inclusive).
    Range(u64, u64),
    /// The parameter will have a non-null value, but otherwise be completely
    /// unconstrained (could point anywhere or alias anything).
    /// This can only be used for pointer-type parameters.
    NonNullPointer,
    /// The parameter will point to allocated memory, with the given allocation
    /// size in bytes. It will not be NULL and will not alias any other allocated
    /// memory.
    /// This can only be used for pointer-type parameters.
    PointerToAllocated(u64),
}

impl Default for ParameterVal {
    fn default() -> Self {
        Self::Unconstrained
    }
}
