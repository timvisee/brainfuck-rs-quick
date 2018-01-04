mod bf;

use std::fs::File;
use std::io::Read;

use bf::bf;



/// Application entrypoint.
fn main() {
    // Open the input file
    let mut file = File::open("program.bf")
        .expect("failed to open 'program.bf'");

    // Read the file
    let mut program = String::new();
    file.read_to_string(&mut program)
        .expect("failed to read 'program.bf'");

    // Run the program and output
    println!("{}", bf(&program));
}



#[test]
fn test_hello_world() {
    assert_eq!(
        bf("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."),
        "Hello World!\n",
    );
}
