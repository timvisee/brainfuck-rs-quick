use super::super::super::Op;



/// Optimize addition and zero routines.
///
/// This optimization is applied on routines.
/// True or false should be given to `cond` depending on whether the routine
/// is conditional or not.
/// The operations contained by the routine should be given to `ops`.
///
/// If `Some` is returned, the whole routine should be replaced by it's
/// contents.
pub fn optimize_add_and_zero(cond: bool, ops: &Vec<Op>) -> Option<Op> {
    // Do not run if this isn't a conditional loop,
    // there must be at least six operations
    if !cond || ops.len() < 4 {
        return None;
    }

    // Create a operations iterator
    let mut iter = ops.iter().enumerate();

    // The first cell must subtract
    // TODO: support other amounts here
    let step = 1;
    match iter.next() {
        Some((_, &Op::Inc { amount })) if amount == -step => {},
        _ => return None,
    }

    // Create a vector for relative memory positions to copy to,
    // and a variable that remembers the current offset
    let mut targets = Vec::with_capacity((ops.len() - 2) / 2);
    let mut offset = 0;

    // Find cells this data is moved into
    loop {
        // Get the seek and subtract operators
        let (_, seek_op) = iter.next()?;
        let (sub_i, sub_op) = iter.next()?;

        // This must be a seek, modify the offste
        match seek_op {
            &Op::Seek { amount } => offset += amount,
            _ => return None,
        }

        // This must be addition
        match sub_op {
            &Op::Inc { amount } if amount == step => {},
            _ => return None,
        }

        // Add the current offset to the target list
        targets.push(offset);

        // If the subtraction was the second last operator,
        // the last must set the offset back to zero
        if sub_i == ops.len() - 2 {
            // Get the reset operator
            let (_, reset_op) = iter.next()?;

            // This must be addition
            match reset_op {
                &Op::Seek { amount } if amount == -offset=> {},
                _ => return None,
            }

            // This optimization is succesful, return the resulting operator
            return Some(
                Op::AddAndZero {
                    targets,
                }
            );
        }
    }
}
