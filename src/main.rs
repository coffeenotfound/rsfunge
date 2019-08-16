#[macro_use]
extern crate lazy_static;

pub mod cli;
//pub mod error;
pub mod utils;
pub mod funge_dialect;
pub mod interpreter;

fn main() {
	// Start the program
	crate::cli::start();
}
