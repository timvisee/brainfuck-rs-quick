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
