use std::fmt::Debug;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PossibleSolutions<V> where V: Clone + PartialEq + Eq + Debug {
    ExactlyOnePossibleSolution(V),
    MultiplePossibleSolutions,
    NoSolutions,
}
