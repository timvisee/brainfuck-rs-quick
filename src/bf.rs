use std::u8;

use interpreter::Interpreter;
use memory::Memory;



/// Interpret a Brainfuck program from a string.
/// Return the result string.
pub fn bf(prog: &str) -> String {
    // Create application memory, and define an output vector
    let mut memory = Memory::new();
    let mut output: Vec<u8> = vec![];

    // Interpret the program, and execute it
    Interpreter::interpret(&mut prog.bytes())
        .execute(&mut memory, &mut output);

    // Parse and output the string
    String::from_utf8(output).unwrap()
}
