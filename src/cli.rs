use std::env;
use std::error;
use std::fmt::{self, Display};
use std::path::PathBuf;
use clap::{self, Arg};
use crate::{FungeDialect};
use crate::interpreter::{FungeInterpreter, FungeSpaceAccessor, FungeDimension, FungeDim2, FungeDim3, SpaceAccessorDim2, SpaceAccessorDim3};
use crate::io::{CodeLoader, CodeSource};
use std::io::{stdin, stdout};
use std::rc::Rc;
use crate::interpreter::fingerprint::{FingerprintRegistry};
use crate::interpreter::fingerprint::standard::{create_null_fingerprint};
use std::cell::RefCell;
use crate::utils::humanize;

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

pub fn start() -> i32 {
	// Handle cli
	let run_options: RunOptions;
	if let Ok(options) = parse_cli() {
		run_options = options;
	}
	else {
		return -1;
	}
	
	// Run interpreter
	let dialect_mode: FungeDialect = FungeDialect::Befunge98;
	let res = match dialect_mode {
		FungeDialect::Befunge93 => run_interpreter::<FungeDim2, SpaceAccessorDim2<i32>>(run_options),
//		FungeDialect::Unefunge98 => run_interpreter::<FungeDim1, SpaceAccessorDim1<i32>>(run_options),
		FungeDialect::Befunge98 => run_interpreter::<FungeDim2, SpaceAccessorDim2<i32>>(run_options),
		FungeDialect::Trefunge98 => run_interpreter::<FungeDim3, SpaceAccessorDim3<i32>>(run_options),
		_ => unimplemented!(),
	};
	
	// Exit with exit code
	return res.0;
}

fn parse_cli() -> Result<RunOptions, impl error::Error> {
	// Construct cli
	let dialect_list = humanize::format_humaized_list(vec!["TEST", "WORLD"].as_slice());
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
		
		// TODO: Parse rest of options
		
		// Parse dialect options
		let dialect_mode: DialectOption = if let Some(dialect_name) = matches.value_of("dialect") {
			match dialect_name {
				"b93" | "befunge93" => DialectOption::Specific(FungeDialect::Befunge93),
				"u98" | "unefunge98" => DialectOption::Specific(FungeDialect::Unefunge98),
				"b98" | "befunge98" => DialectOption::Specific(FungeDialect::Befunge98),
				"t98" | "trefunge98" => DialectOption::Specific(FungeDialect::Trefunge98),
				other => {
					return Err(ArgError::new(String::from(format!("Unknown funge dialect '{}'", other))));
				}
			}
		}
		else {
			DialectOption::Unknown
		};
		
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
			dialect_mode,
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

fn run_interpreter<N, A>(run_options: RunOptions) -> (i32,) where N: FungeDimension, A: FungeSpaceAccessor<N, i32> {
	// Load inital code
	let code_source = CodeSource::new(run_options.source_file, match run_options.dialect_mode {DialectOption::Specific(d) => Some(d), _ => None});
	
	let mut loader = CodeLoader::new();
	let code_buffer = loader.load_from_file(code_source.clone());
	
	if let Err(e) = code_buffer {
		panic!("Failed to load code from file: \"{}\" ({})", code_source.get_path().display(), e);
	}
	
	// Get actual dialect
	let actual_dialect: FungeDialect = if let DialectOption::Specific(d) = run_options.dialect_mode {d}
	else {
		// TODO: Implement dialect probing
		unimplemented!();
	};
	
	// Make fingerprint registry
	let fingerprint_registry_ref = Rc::new(RefCell::new(FingerprintRegistry::new()));
	
	// Register standard fingerprints
	fingerprint_registry_ref.borrow_mut().register_fingerprint(Rc::from(create_null_fingerprint()));
	
	// Create interpreter
	let charout = stdout();
	let charin = stdin();
	let mut interpreter: FungeInterpreter<N, A> = FungeInterpreter::new(actual_dialect, code_source, fingerprint_registry_ref, charout, charin);
	
	// Load inital code into interpreter
	interpreter.load_initial_code(&code_buffer.unwrap());
	
	// Transfer control to interpreter and start execution
	interpreter.start_execution();
	
	// Exit with exit code
	let exit_code = if let Some(code) = interpreter.get_programmatic_exit_code() {code}
	else {0};
	
	return (exit_code,);
}

pub struct RunOptions {
	source_file: PathBuf,
	dialect_mode: DialectOption,
}

pub enum DialectOption {
	Specific(FungeDialect),
	Unknown,
}
