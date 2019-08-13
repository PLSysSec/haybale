#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PossibleSolutions<V> {
    /// This is exactly the set of possible solutions; there are no others.
    /// Note that an empty vector here indicates there are no possible solutions.
    PossibleSolutions(Vec<V>),
    /// There are more than `n` possible solutions, where `n` is the value
    /// contained here.
    MoreThanNPossibleSolutions(usize),
}
