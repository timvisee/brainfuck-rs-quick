mod app;
mod arg_handler;
mod bf;

use std::fs::File;
use std::io::Read;

use arg_handler::ArgHandler;
use bf::bf;



/// Application entrypoint.
fn main() {
    // Parse CLI arguments, get interpreter options
    let arg_handler = ArgHandler::parse();
    let options = arg_handler.as_options();

    // Open the input file
    let mut file = File::open(arg_handler.file())
        .expect("failed to open 'program.bf'");

    // Read the file
    let mut program = String::new();
    file.read_to_string(&mut program)
        .expect("failed to read 'program.bf'");

    // Run the program
    let output = bf(&program, &options);

    // Print the output
    if arg_handler.buffer() {
        println!("{}", output);
    }
}
