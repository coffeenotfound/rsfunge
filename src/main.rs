#![allow(unused_parens)]
#![feature(specialization)]
#![feature(trait_alias)]

#[macro_use]
extern crate lazy_static;

pub mod cli;
//pub mod error;
pub mod utils;
pub mod funge_dialect;
pub mod interpreter;
pub mod io;
pub mod vector;
pub mod buffer;

pub mod final_ref;

// DEBUG:
//pub mod stringth;

fn main() {
	// Start the program
	crate::cli::start();
}
