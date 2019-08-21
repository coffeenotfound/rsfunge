use std::error::Error;
use crate::funge_dialect::FungeDialect;
use crate::io::{CodeBuffer, CodeSource, CodeBufferLine};
use std::marker::PhantomData;
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::thread::current;

/// Loads code from file into a code buffer with valid data.
pub struct CodeLoader {
	_phantom: PhantomData<()>,
}
impl CodeLoader {
	pub fn new() -> Self {
		return CodeLoader {
			_phantom,
		};
	}
	
	/// Tries to load funge* source code from the given file.
	/// Returns a code buffer with the code if it could be loaded
	/// correctly.
	pub fn load_from_file(&mut self, source: CodeSource) -> Result<CodeBuffer, impl Error> {
		// Open file
		let path: &Path = source.get_path();
		
		let open_opts = OpenOptions::new().read(true);
		let file = open_opts.open(path);
		
		// Handle error opening file
		if let Err(e) = file {
			return Err(e);
		}
		let mut file = file.unwrap();
		
		// Read file
		let mut string_contents = String::new();
		let num_read_bytes = file.read_to_string(&mut string_contents)?;
		
		// TODO: Optimize this using a however big read buffer and manual deconding with rust-encoding
		
		// Allocate line buffer
		let mut line_buffer = Vec::<CodeBufferLine>::with_capacity(64);
		let mut current_line = CodeBufferLine::new(32);
		
		// TODO: Actually do this properly instead of copying to a new decoded array which is stupid
		let decoded_string = string_contents.chars().collect();
		
		let mut i = 0;
		loop {
			
		}
//		for char in string_contents.chars() {
//			match char {
//				'\u{0065}' => {
//
//				}
//				_ => {
//
//				}
//			}
//
//			// Push char
//			current_line.data.push(char as u32);
//		}
	}
}

pub enum CodeReadError {
	InvalidDimensions,
}
