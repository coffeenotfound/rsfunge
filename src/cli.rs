use clap::{self, Arg};
use std::env;
use crate::utils;

//pub type ArgumentError = GenericError<S>;

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
	let evaluation_res = (|| {
		let cli_result = cli.get_matches_from_safe(env::args_os());
		
		let matches;
		if let Ok(m) = cli_result {
			matches = m;
		}
		else {
			// TODO: Throw error
			return;
		}
		
//		FungeDialect::
		
		let source_file = matches.value_of("source-file");
	})();
	
	// Setup interpreter
}
