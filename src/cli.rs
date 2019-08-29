use clap::{self, Arg};
use std::env;
use std::error;
use crate::{utils, FungeDialect};
use crate::interpreter::FungeInterpreter;
use std::fmt::{self, Display};
use crate::io::{CodeLoader, CodeSource};
use std::path::PathBuf;

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
			.empty_values(false)
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
	
	// Check for cli (validation) errors
	if let Err(err) = evaluation_res {
		panic!("Cli error"); // TODO: Handle properly
	}
	
	// Get params
	let code_source = CodeSource::new(PathBuf::new(), FungeDialect::Befunge98);
	
	// Load code
	let mut loader = CodeLoader::new();
	let code_buffer = loader.load_from_file(code_source.clone());
	
	if let Err(e) = code_buffer {
		panic!("Failed to load code from file: \"{}\" ({})", code_source.get_path().display(), e);
	}
	
	// Create interpreter
	let mut interpreter: FungeInterpreter = FungeInterpreter::new(code_source);
	
	// Load inital code into interpreter
	interpreter.load_initial_code(&code_buffer.unwrap());
	
	// Transfer control to interpreter and start execution
	interpreter.start_execution();
}
