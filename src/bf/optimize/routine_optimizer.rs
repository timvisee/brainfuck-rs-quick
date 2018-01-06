use super::super::Op;

use super::routine::optimize_zero;



/// Optimize a routine.
///
/// This optimization is applied on routines.
/// True or false should be given to `cond` depending on whether the routine
/// is conditional or not.
/// The operations contained by the routine should be given to `ops`.
///
/// If `Some` is returned, the whole routine should be replaced by it's
/// content.
pub fn optimize_routine(cond: bool, ops: &Vec<Op>) -> Option<Op> {
    optimize_zero(cond, ops)
}
