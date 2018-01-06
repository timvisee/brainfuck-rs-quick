//! # Zeroing routine optimization
//! This optimizes routines that zero the current memory cell,
//! and replaces the routine with a single instruction.
//!
//! Because memory cells should overflow, this optimization also replaces
//! routines that only use additions.
//!
//! If a memory cell is an odd number, and a routine constantly adds an even
//! number to the cell it would normally be possible to create an infinite
//! routine as zero would never be reached. This optimization ignores this
//! and zero's te cell.
//!
//! # Optimization requirements
//! - A routine that must only contain add and subtract operations.
//!
//! # Example routines
//! - `[-]'
//!     - Zero the current cell
//! - `[---]`
//!     - Zero the current cell
//! - `[+]`
//!     - Zero the current cell
//! - `[++-]`
//!     - Zero the current cell



use super::super::super::Op;



/// Optimize zeroing routines.
///
/// This optimization is applied on routines.
/// True or false should be given to `cond` depending on whether the routine
/// is conditional or not.
/// The operations contained by the routine should be given to `ops`.
///
/// If `Some` is returned, the whole routine should be replaced by it's
/// contents.
pub fn optimize_zero(cond: bool, ops: &Vec<Op>) -> Option<Op> {
    // Do not run if this isn't a conditional loop
    if !cond {
        return None;
    }

    // Check whether we can optimize
    if ops.iter().all(
        |op| match *op {
            Op::Inc { .. } => true,
            _ => false,
        }
    ) {
        Some(Op::Zero)
    } else {
        None
    }
}
