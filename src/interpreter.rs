use std::mem;
use std::str::Bytes;

use op::Op;



/// Brainfuck interpreter.
///
/// This interpreter translates a stream of brainfuck program bytes into
/// operations.
pub struct Interpreter;

impl Interpreter {
    /// Interpret a brainfuck program from the given byte stream.
    /// Output a routine containing the whole state.
    pub fn interpret(program: &mut Bytes) -> Op {
        Interpreter::interpret_routine(program, false)
    }

    /// Interpret the given stream of bytes into a routine.
    /// This routine may be simple, or it may be conditional with makes the routine
    /// loopable.
    ///
    /// The byte stream should be given to `bytes`.
    ///
    /// If `cond` is `true`, this routine is loopable, `false` if it isn't.
    fn interpret_routine(bytes: &mut Bytes, cond: bool) -> Op {
        // Interpret the contained routine operations
        let ops = Interpreter::interpret_vec(bytes);

        // Optimize a zeroing loop
        if cond && ops.iter().all(|op| match *op {
                Op::Inc { .. } => true,
                _ => false,
            }) {
            return Op::Zero;
        }

        // Wrap the oprations in a routine
        Op::Routine {
            ops,
            cond,
        }
    }

    /// Interpret the given stream of bytes into a vector of operations.
    ///
    /// This function returns the vector if the input stream is empty,
    /// or if a loop-end operator has been reached.
    ///
    /// The byte stream should be given to `bytes`.
    fn interpret_vec(bytes: &mut Bytes) -> Vec<Op> {
        // Create an operations vector, and a workspace for the last operation
        // being worked on
        let mut ops = vec![];
        let mut workspace = None;

        // Interpret all bytes until we break
        loop {
            // Find the next byte to process, or break if the stream is emtpy
            let byte = if let Some(byte) = bytes.next() {
                byte
            } else {
                break;
            };

            // Process the byte
            match byte {
                // Seek up
                b'>' => Interpreter::process_workspace_seek(
                    &mut workspace,
                    &mut ops,
                    1,
                ),

                // Seek down
                b'<' => Interpreter::process_workspace_seek(
                    &mut workspace,
                    &mut ops,
                    -1,
                ),

                // Increase memory cell value
                b'+' => Interpreter::process_workspace_inc(
                    &mut workspace,
                    &mut ops,
                    1,
                ),

                // Decrease memory cell value
                b'-' => Interpreter::process_workspace_inc(
                    &mut workspace,
                    &mut ops,
                    -1,
                ),

                // Output the value of the current memory cell
                b'.' => {
                    // Commit and add a new operator
                    Interpreter::commit(&mut workspace, &mut ops, None);
                    ops.push(Op::Output);
                },

                // Read user input
                b',' => {
                    // Commit and add a new operator
                    Interpreter::commit(&mut workspace, &mut ops, None);
                    ops.push(Op::Input);
                },

                // Start a conditional loop
                b'[' => {
                    // Commit and add a new conditional routine
                    Interpreter::commit(&mut workspace, &mut ops, None);
                    ops.push(
                        Interpreter::interpret_routine(bytes, true),
                    );
                },

                // End a conditional loop, finish this operation vector
                b']' => break,

                // Unrecognized operation, skip
                _ => continue,
            }
        }

        // Commit the last workspace operation
        if let Some(op) = workspace {
            ops.push(op);
        }

        ops
    }

    /// Commit the given workspace in the given.
    /// And reinitialize the workspace with the given `fresh` operator.
    /// This is quicker than first setting it to zero.
    ///
    /// This method is intended to be used internally.
    ///
    /// The `workspace` is committed to `ops` if set.
    /// This leaves `workspace` with `fresh`.
    ///
    /// You may want to consider using `None` as `fresh` option,
    /// to reset the workspace.
    fn commit(workspace: &mut Option<Op>, ops: &mut Vec<Op>, fresh: Option<Op>) {
        // Take the workspace item, put it in the list
        if let Some(op) = mem::replace(workspace, fresh) {
            ops.push(op);
        }
    }

    /// Process a seek instruction, in the context of the given workspace.
    ///
    /// The workspace may be used to combine this new instruction with,
    /// as optimization.
    ///
    /// If an incompatible instruction was in the workspace, the workspace is
    /// committed, and a new workspace is created with the preferred
    /// instruction.
    ///
    /// If the workspace was compatible, the workspace will be left uncommitted
    /// for possible further optimizations in upcomming instructions.
    ///
    /// The `workspace` is committed to `ops`.
    fn process_workspace_seek(
        workspace: &mut Option<Op>,
        ops: &mut Vec<Op>,
        amount: isize,
    ) {
        // Determine whether to combine to an existing workspace,
        // or to commit and define a new operator workspace
        match *workspace {
            // Combine with the workspace operation
            Some(
                Op::Seek {
                    amount: ref mut current,
                }
            ) => *current += amount,

            // Commit the workspace, start working on a new seek operator
            _ => Interpreter::commit(
                workspace,
                ops,
                Some(
                    Op::Seek { amount }
                ),
            ),
        }
    }

    /// Process a increment instruction, in the context of the given workspace.
    ///
    /// The workspace may be used to combine this new instruction with,
    /// as optimization.
    ///
    /// If an incompatible instruction was in the workspace, the workspace is
    /// committed, and a new workspace is created with the preferred
    /// instruction.
    ///
    /// If the workspace was compatible, the workspace will be left uncommitted
    /// for possible further optimizations in upcomming instructions.
    ///
    /// The `workspace` is committed to `ops`.
    fn process_workspace_inc(
        workspace: &mut Option<Op>,
        ops: &mut Vec<Op>,
        amount: isize,
    ) {
        // Determine whether to combine to an existing workspace,
        // or to commit and define a new operator workspace
        match *workspace {
            // Combine with the workspace operation
            Some(
                Op::Inc {
                    amount: ref mut current,
                }
            ) => *current += amount,

            // Commit the workspace, start working on a new increment operator
            _ => Interpreter::commit(
                workspace,
                ops,
                Some(
                    Op::Inc { amount }
                ),
            ),
        }
    }
}
