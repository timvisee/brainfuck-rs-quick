/// The size of the memory.
const MEM_SIZE: usize = 30_000;



/// The memory bank of a brainfuck program.
///
/// This struct defines the state of such a program,
/// and provides helper functions to easily manage it.
pub struct Memory {
    /// The memory data set
    data: [u8; MEM_SIZE],

    /// Index of the current memory cell pointer
    pointer: usize,
}

impl Memory {
    /// Create new application memory.
    ///
    /// This allocates all memory the program might use,
    /// and returns the initial memory state.
    pub fn new() -> Memory {
        Memory {
            data: [0; MEM_SIZE],
            pointer: 0,
        }
    }

    /// Seek the memory cell pointer for the given relative `amount`.
    pub fn seek(&mut self, amount: isize) {
        // // TODO: Is this correct
        // // Seek if the value won't underflow
        // if amount >= 0 || self.pointer as isize >= -amount {
        //     self.pointer += amount as usize;
        // } else {
        //     self.pointer = 0;
        // }
        self.pointer += amount as usize;
    }

    /// Increase the value of the current memory cell by the given relative
    /// `amount`.
    pub fn inc(&mut self, amount: isize) {
        // TODO: Don't go out of bound
        // TODO: Don't cast
        self.data[self.pointer] += amount as u8;
    }

    /// Read and return the value of the current memory cell.
    pub fn read(&self) -> u8 {
        self.data[self.pointer]
    }

    /// Write the given value to the current memory cell.
    pub fn write(&mut self, value: u8) {
        self.data[self.pointer] = value;
    }

    /// Check whether the current memory cell is zero.
    pub fn zero(&self) -> bool {
        self.data[self.pointer] == 0
    }

    /// Set the current memory cell value to zero.
    pub fn set_zero(&mut self) {
        self.data[self.pointer] = 0;
    }

    /// Move the current cell value to the given relative targets,
    /// zeroing the current cell.
    pub fn copy_zero(&mut self, targets: &Vec<isize>) {
        // Read the cell value, return if it is zero
        let value = self.data[self.pointer];
        if value == 0 {
            return;
        }

        // Write the values
        for target in targets {
            // TODO: is this cast correct
            self.data[(self.pointer as isize + target) as usize] += value;
        }

        // Zero the current cell
        self.set_zero();
    }
}
