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

/// Loads code from file into a code buffer with valid data.
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
	/// Returns a code buffer with the code if it could be loaded
	/// correctly.
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
		
		// TODO: Optimize this using a however big read buffer and manual deconding with rust-encoding
		
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
						current_line.terminator = LineTerminator::NextY;
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
						current_line.terminator = LineTerminator::NextY;
						break 'sourceloop;
					}
					'\u{12}' => { // Form feed, increment z coord
						// Set terminator
						current_line.terminator = LineTerminator::NextZ;
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
					LineTerminator::NextY => {
						bounding_box.set_y(bounding_box.y() + 1);
					},
					LineTerminator::NextZ => {
						bounding_box.set_z(bounding_box.z() + 1);
					},
					_ => {},
				}
			}
		}
		
		// Make code buffer instance
		let code_buffer = CodeBuffer {
			lines: line_buffer,
			dimensions: 0, // TODO: Actually handle this properly
			bounding_size: bounding_box,
		};
		return Ok(code_buffer);
	}
}

pub enum CodeReadError {
	InvalidDimensions,
}
