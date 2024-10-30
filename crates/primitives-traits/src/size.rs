/// Trait for calculating a heuristic for the in-memory size of a struct.
pub trait HeuristicSize {
    /// Returns a heuristic for the in-memory size of a struct.
    fn size(&self) -> usize;
}
