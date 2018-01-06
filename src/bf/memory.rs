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
    ///
    /// The pointer won't underflow as specified by the brainfuck
    /// specification. Instead the pointer would be set to zero.
    pub fn seek(&mut self, amount: isize) {
        self.pointer = Memory::seek_virtual(self.pointer, amount);
    }

    /// Seek a virtual memory cell pointer by the given relative `amount`.
    /// The new pointer position is returned.
    ///
    /// This method follows the brainfuck specificaiton, not allowing the
    /// index to underflow. If the number would underflow, zero is returned.
    fn seek_virtual(pointer: usize, amount: isize) -> usize {
        if amount > 0 {
            pointer + (amount as usize)
        } else {
            pointer.checked_sub(-amount as usize).unwrap_or(0)
            // pointer - (-amount as usize)
        }
    }

    /// Increase the value of the current memory cell by the given relative
    /// `amount`.
    ///
    /// The pointer won't underflow as specified by the brainfuck
    /// specification. Instead the memory cell would be set to zero.
    /// Overflowing is allowed.
    pub fn inc(&mut self, amount: isize) {
        self.data[self.pointer] = Memory::inc_virtual(
            self.data[self.pointer],
            amount,
        );
    }

    /// Increate a virutal memory cell by the given relative `amount`.
    /// The new memory cell value is returned.
    ///
    /// This method follows the brainfuck specification, not allowing the
    /// index to underflow. If the number would underflow, zero is returned.
    /// Overflowing is allowed.
    fn inc_virtual(value: u8, amount: isize) -> u8 {
        if amount > 0 {
            value + (amount as u8)
        } else {
            value.checked_sub(-amount as u8).unwrap_or(0)
            // value - (-amount as u8)
        }
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
    ///
    /// The targets 
    pub fn copy_zero(&mut self, targets: &Vec<(isize, f32)>) {
        // Read the cell value, return if it is zero
        let value = self.data[self.pointer];
        if value == 0 {
            return;
        }

        // Write the values
        for &(target, factor) in targets {
            // Determine the pointer position
            let pointer = Memory::seek_virtual(self.pointer, target);

            // Increase the data in the cell
            self.data[pointer] = Memory::inc_virtual(
                self.data[pointer],
                (value as f32 * factor) as isize,
            );
        }

        // Zero the current cell
        self.set_zero();
    }
}
