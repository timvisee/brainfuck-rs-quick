use std::u8;

use super::Interpreter;
use super::Memory;
use super::Options;



/// Interpret a Brainfuck program from a string.
/// Return the result string.
pub fn bf(prog: &str, options: &Options) -> String {
    // Create application memory, and define an output vector
    let mut memory = Memory::new();
    let mut output: Vec<u8> = vec![];

    // Interpret the program, and execute it
    Interpreter::interpret(&mut prog.bytes())
        .execute(&mut memory, &options, &mut output);

    // Parse and output the string
    String::from_utf8(output).unwrap()
}



/// Test whether a basic Hello World implementation of brainfuck outputs the
/// correct result.
#[test]
fn test_hello_world() {
    assert_eq!(
        bf("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."),
        "Hello World!\n",
    );
}
