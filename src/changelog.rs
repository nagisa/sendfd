//! Project changelog


/// Release 0.3.0
///
/// * Removed the `Receivable` trait, because it is difficult to write meaningful code with `<T as
/// Receivable>` for `T ≠ RawFd`.
pub mod r0_3_0 {
}

/// Release 0.2.1
///
/// Removed an accidentally publicly exported internal function.
///
/// 0.2.0 has been yanked.
pub mod r0_2_1 {
}

/// Release 0.2.0
///
/// Pure-Rust reimplementation of the crate.
pub mod r0_2_0 {
}
