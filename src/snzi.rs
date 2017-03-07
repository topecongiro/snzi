/// Scalable NonZero Indicator or SNZI. SNZI provides a shared counter
/// abstraction with relaxed semantic.
pub trait SNZI {
    /// Announces the arrival of the calling thread.
    fn arrive(&mut self);

    /// Announces the department of the calling thread.
    fn depart(&mut self);

    /// Returns whether the number of `arrive()` operations exceeds the number of
    /// `depart()` operations.
    fn query(&self) -> bool;
}

/// An implementation of SNZI trait using a tree based structure.
///
/// It has two invariants:
/// 1. A parent's surplus due to child is never negative.
/// 2. A parent ahs a surplus due to a child iff the child has a surplus.
pub struct TreeSNZI {
    
}

struct SNZINode {
    
}
