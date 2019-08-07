pub struct Config {
    /// Maximum number of times to execute any given line of LLVM IR.
    /// This bounds both the number of iterations of loops, and also the depth of recursion.
    /// For inner loops, this bounds the number of total iterations across all invocations of the loop.
    pub loop_bound: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            loop_bound: 10,
        }
    }
}
