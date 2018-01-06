mod app;
mod arg_handler;
mod bf;
mod profiler;

use std::fs::File;
use std::io::Read;

use arg_handler::ArgHandler;
use bf::{bf, Options};
use profiler::Profiler;



/// Application entrypoint.
fn main() {
    // Parse CLI arguments, get interpreter options
    let arg_handler = ArgHandler::parse();
    let options = arg_handler.as_options();

    // Read the file
    let program = read_file(arg_handler.file(), &options);

    // Run the program
    let output = bf(&program, &options);

    // Print the output
    if arg_handler.buffer() {
        println!("{}", output);
    }
}

/// Read file contents.
fn read_file(path: &str, options: &Options) -> String {
    // Profile
    let mut profiler = Profiler::new(options.profile);

    // Open the input file
    let mut file = File::open(path)
        .expect("failed to open 'program.bf'");

    // Read the file
    let mut program = String::new();
    file.read_to_string(&mut program)
        .expect("failed to read 'program.bf'");

    // Report time
    if options.profile {
        profiler.report("Loading program");
    }

    program
}
