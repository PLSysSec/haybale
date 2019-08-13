#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PossibleSolutions<V> {
    ExactlyOnePossibleSolution(V),
    MultiplePossibleSolutions,
    NoSolutions,
}
