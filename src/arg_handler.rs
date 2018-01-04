extern crate clap;

use self::clap::{Arg, ArgMatches, App};

use app::*;



/// CLI argument handler.
pub struct ArgHandler<'a> {
    matches: ArgMatches<'a>,
}

impl<'a: 'b, 'b> ArgHandler<'a> {

    /// Parse CLI arguments.
    pub fn parse() -> ArgHandler<'a> {
        // Handle/parse arguments
        let matches = App::new(APP_NAME)
            .version(APP_VERSION)
            .author(APP_AUTHOR)
            .about(APP_ABOUT)
            .arg(Arg::with_name("FILE")
                .help("Brainfuck file to interpret")
                .required(true)
                .index(1))
            .get_matches();

        // Instantiate
        ArgHandler {
            matches,
        }
    }

    /// Get the file property.
    pub fn file(&'a self) -> &'b str {
        self.matches.value_of("FILE")
            .expect("Please specify a brainfuck file to parse")
    }
}
