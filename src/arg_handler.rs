extern crate clap;

use self::clap::{Arg, ArgMatches, App};

use app::*;
use bf::Options;



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
			.arg(Arg::with_name("buffer")
                .short("b")
                .long("buffer")
                .help("Buffer output until the program is finished"))
			.arg(Arg::with_name("profiler")
                .short("p")
                .long("profiler")
                .alias("profile")
                .help("Enable the profiler to interpreter stages"))
			.arg(Arg::with_name("describe")
                .short("d")
                .long("describe")
                .alias("desc")
                .help("Describe interpreted and optimized program logic"))
			.arg(Arg::with_name("pretty")
                .long("pretty")
                .alias("prettify")
                .help("Pretify described program logic"))
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

	/// Check whether we should buffer output until the application completes.
	pub fn buffer(&self) -> bool {
		self.matches.is_present("buffer")
	}

	/// Check whether we should profile interpreter stages.
	pub fn profile(&self) -> bool {
		self.matches.is_present("profiler")
	}

	/// Check whether to describe program logic.
	pub fn describe(&self) -> bool {
		self.matches.is_present("describe")
	}

	/// Check whether to pretify described program logic.
	pub fn pretty(&self) -> bool {
		self.matches.is_present("pretty")
	}

    /// Create an interpreter options object, based on the CLI arguments.
    pub fn as_options(&self) -> Options {
        Options::default(
            self.buffer(),
            self.profile(),
            self.describe(),
            self.pretty(),
        )
    }
}
