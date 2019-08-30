use std::env;
use std::error;
use std::fmt::{self, Display};
use std::path::PathBuf;
use clap::{self, Arg};
use crate::{utils, FungeDialect};
use crate::interpreter::FungeInterpreter;
use crate::io::{CodeLoader, CodeSource};
use std::io::{stdin, stdout};

//pub type ArgumentError = GenericError<S>;#
#[derive(Debug)]
pub struct ArgError {
	message: String,
}
impl ArgError {
	pub fn new(message: String) -> Self {
		ArgError {
			message,
		}
	}
}
impl error::Error for ArgError {
	
}
impl Display for ArgError {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}", self.message)
	}
}

pub fn start() {
	// Handle cli
	let run_options: RunOptions;
	if let Ok(options) = parse_cli() {
		run_options = options;
	}
	else {
		return;
	}
	
	// Load inital code
	let code_source = CodeSource::new(run_options.source_file, FungeDialect::Befunge98); // TODO: Use proper dialect
	
	let mut loader = CodeLoader::new();
	let code_buffer = loader.load_from_file(code_source.clone());
	
	if let Err(e) = code_buffer {
		panic!("Failed to load code from file: \"{}\" ({})", code_source.get_path().display(), e);
	}
	
	// Create interpreter
	let mut charout = stdout();
	let mut charin = stdin();
	let mut interpreter: FungeInterpreter = FungeInterpreter::new(code_source, &mut charout, &mut charin);
	
	// Load inital code into interpreter
	interpreter.load_initial_code(&code_buffer.unwrap());
	
	// Transfer control to interpreter and start execution
	interpreter.start_execution();
}

fn parse_cli() -> Result<RunOptions, impl error::Error> {
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
	let eval_result = (|| -> Result<RunOptions, ArgError> {
		let cli_result = cli.get_matches_from_safe(env::args_os());
		
		let matches;
		if let Ok(m) = cli_result {
			matches = m;
		}
		else {
			return Err(ArgError::new(String::from("Failed to parse")));
		}
		
		// TODO: Parse dialect
		// TODO: Parse rest of options
		
		// Get source file path
		let source_file = {
			let file = matches.value_of("source-file");
			
			if file.is_none() {
				return Err(ArgError::new(String::from("Source file must be specified")));
			}
			PathBuf::from(file.unwrap())
		};
		
		// Make options object
		let options = RunOptions {
			source_file,
		};
		return Ok(options);
	})();
	
	match eval_result {
		// Handle validation error
		Err(err) => {
			// Print cli help
			eprint!("{}", err.message);
			
			return Err(err); // TODO: Handle properly, i.e. wrap in own error with message
		}
		// Return valid RunOptions object
		Ok(options) => {
			return Ok(options);
		}
	}
}

pub struct RunOptions {
	source_file: PathBuf,
}

pub enum DialectOption {
	Specific(FungeDialect),
	Unknown,
}
