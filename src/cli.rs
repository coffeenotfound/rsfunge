use clap::{self, Arg};
use std::env;
use std::error;
use crate::utils;
use crate::interpreter::FungeInterpreter;
use std::fmt::{self, Display};

//pub type ArgumentError = GenericError<S>;#
#[derive(Debug)]
pub struct ArgError;
impl error::Error for ArgError {
	
}
impl Display for ArgError {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "Arg error")
	}
}

pub fn start() {
	// Construct cli
	let dialect_list = utils::format_humaized_list(vec!["TEST", "WORLD"].as_slice());
	let dialect_help: String = format!("Which dialect of Funge to use ({})", dialect_list);
	
	let cli = clap::App::new("rsfunge")
		.arg(Arg::with_name("dialect")
			.short("d").long("dialect")
			.help(&dialect_help)
			.takes_value(true))
		.arg(Arg::with_name("source-file")
			.index(1)
			.required(true));
	
	// Evalutate cli invocation
	let evaluation_res = (|| -> Result<(), ArgError> {
		let cli_result = cli.get_matches_from_safe(env::args_os());
		
		let matches;
		if let Ok(m) = cli_result {
			matches = m;
		}
		else {
			return Err(ArgError);
		}
		
		// TODO: Parse dialect
		
		let source_file = matches.value_of("source-file");
		
		return Ok(());
	})();
	
	// Load code
	
	
	// Create interpreter
	let interpreter: FungeInterpreter; // = FungeInterpreter::new();
}
