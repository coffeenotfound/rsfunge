use std::error::Error;
use std::marker::PhantomData;
use std::path::Path;
use std::fs::{OpenOptions};
use std::io::Read;
use std::iter::Peekable;
use std::str::Chars;
use std::cmp;
use crate::io::{CodeBuffer, CodeSource, CodeBufferLine, LineTerminator};
use crate::vector::Vector3;

/// Loads funge source code from file into a code buffer.
pub struct CodeLoader {
	_phantom: PhantomData<()>,
}
impl CodeLoader {
	pub fn new() -> Self {
		return CodeLoader {
			_phantom: PhantomData,
		};
	}
	
	/// Tries to load funge* source code from the given file.
	/// Returns either a code buffer with the code if it could be loaded correctly or
	/// an Err if an error occured.
	/// 
	/// The files contents are assumed to be UTF-8 encoded and the resulting
	/// code buffer will not contain any line feed, carriage return, carriage return + line feed
	/// or form feed control codes.
	pub fn load_from_file(&mut self, source: CodeSource) -> Result<CodeBuffer, impl Error> {
		// Open file
		let path: &Path = source.get_path();
		
		let mut open_opts = OpenOptions::new();
		open_opts.read(true);
		
		let file = open_opts.open(path);
		
		// Handle error opening file
		if let Err(e) = file {
			return Err(e);
		}
		let mut file = file.unwrap();
		
		// Read file
		let mut string_contents = String::new();
		let num_read_bytes = file.read_to_string(&mut string_contents)?;
		
		// TODO: Support different encodings (maybe via the rust-encoding crate)
		
		// Allocate line buffer
		let mut line_buffer = Vec::<CodeBufferLine>::with_capacity(64);
		
		let mut bounding_box = Vector3::<u32>::new_value(1);
		
		// Consume data
		let mut iter: Peekable<Chars> = string_contents.chars().peekable();
		'lineloop:
		loop {
			// Allocate new line
			let mut current_line = CodeBufferLine::new(32);
			
			// Set terminator to End intially
			// If the line ends without a line terminator (carriage return, etc.)
			// then it's overriden with the approriate value, else it's
			// just gonna be End, indicating EOL.
			current_line.terminator = LineTerminator::End;
			
			// Fill line
			'sourceloop:
//			for char in iter {
			loop {
				let char = if let Some(c) = iter.next() {
					c
				}
				else {
					break 'sourceloop;
				};
				
				match char {
					'\u{10}' => { // Line feed
						// Set terminator
						current_line.terminator = LineTerminator::FeedY;
						break 'sourceloop;
					}
					'\u{13}' => { // Carriage return
						// Check if next char is line feed to make it a carriage return, line feed combo
						if let Some(n) = iter.peek() {
							if *n == '\u{10}' {
								// Consume line feed
								iter.next();
							}
						}
						
						// Set terminator
						current_line.terminator = LineTerminator::FeedY;
						break 'sourceloop;
					}
					'\u{12}' => { // Form feed, increment z coord
						// Set terminator
						current_line.terminator = LineTerminator::FeedZ;
						break 'sourceloop;
					}
					_ => {
						// Put char into lin
						current_line.data.push(char as u32);
					}
				}
			}
			
			// Update bounding box
			bounding_box.set_x(cmp::max(bounding_box.x(), current_line.data.len() as u32));
			
			// Put line into buffer
			let current_terminator = current_line.terminator;
			line_buffer.push(current_line);
			
			// Check if we still have something left
			if let None = iter.peek() {
				break 'lineloop;
			}
			else {
				// Increase bounding box (only if there's actually another line)
				match current_terminator {
					LineTerminator::FeedY => {
						bounding_box.set_y(bounding_box.y() + 1);
					},
					LineTerminator::FeedZ => {
						bounding_box.set_z(bounding_box.z() + 1);
					},
					_ => {},
				}
			}
		}
		
		// TODO: Actually handle dimensionality properly
		// Make code buffer instance
		let code_buffer = CodeBuffer::new(line_buffer, bounding_box, 0);
		return Ok(code_buffer);
	}
}

pub enum CodeReadError {
	InvalidDimensions,
}
