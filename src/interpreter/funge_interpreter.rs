use crate::interpreter::{FungeAddress, FungeDim2, FungeSpace, SpaceAccessorDim2, ThreadList, FungeThread};
use crate::io::{CodeBuffer, CodeSource, LineTerminator};
use crate::vector::Vector3;
use crate::interpreter::instruction::insts;
use std::io::{Write, Read};
use std::num::Wrapping;
use crate::FungeDialect;

/// Interpreter for funge*.
/// Instances directly contain the interpretation state.
pub struct FungeInterpreter<'s, 'io> {
	code_source: CodeSource,
	threads: ThreadList<'s>,
	funge_space: FungeSpace<'s, FungeDim2, i32, SpaceAccessorDim2<i32>>,
	dialect_mode: FungeDialect,
	
	charout: &'io mut dyn Write,
	charin: &'io mut dyn Read,
	
	programatically_quit: bool,
	quit_exit_code: i32,
}

impl<'s, 'io> FungeInterpreter<'s, 'io> {
	pub fn new(code_source: CodeSource, charout: &'io mut dyn Write, charin: &'io mut dyn Read) -> Self {
		// Instantiate
		let mut interpreter = FungeInterpreter {
			code_source,
			threads: ThreadList::new(),
			funge_space: FungeSpace::new(),
			dialect_mode: code_source.get_dialect(),
			charout,
			charin,
			
			programatically_quit: false,
			quit_exit_code: 0,
		};
		
		// Create initial thread
		interpreter.create_thread(Vector3::new(), Vector3::new_xyz(1, 0, 0));
		
		return interpreter;
	}
	
	pub fn get_programmatic_exit_code(&self) -> Option<i32> {
		if self.programatically_quit {
			return Some(self.quit_exit_code);
		}
		else {
			return None;
		}
	}
	
	/// Starts the execution of this interpreter by transferring
	/// control over to it's main loop.
	/// This method does not return aslong as the interpreter
	/// is running.
	pub fn start_execution(&mut self) {
		// Main execution loop
		'mainloop:
		while self.threads.num() > 0 {
			// Execute next tick with each thread
			let mut i = 0;
			while i < self.threads.num() {
//				let thread = self.threads.get_mut(i).unwrap();
				
				// Run thread tick
				self.execute_thread_tick(i);
				
				// Increment index
				i += 1;
			}
		}
		
		// Log
		print!("Main loop terminated");
	}
	
	#[inline]
	fn execute_thread_tick(&mut self, thread_index: u32) {
		let mut thread = self.threads.get_mut(thread_index).unwrap();
		
		let ip = thread.ip;
		
		// Read instruction cell
		let mut move_ip = true;
		let mut valid_instruction = false;
		let instruction = self.funge_space.read_cell(&ip);
		
		// Execute instruction
		if (32 < instruction) && (instruction <= 126) {
			valid_instruction = true;
			
			match instruction {
				/* ! */ 33 => insts::inst_logical_not(thread),
				/* # */ 35 => thread.ip.add_delta_wrapping(&thread.delta),
				/* $ */ 36 => insts::inst_pop(thread),
				/* % */ 37 => insts::inst_remainder(thread),
				/* ' */ 39 => insts::inst_fetch_character(thread, &mut self.funge_space),
				/* * */ 42 => insts::inst_multiply(thread),
				/* + */ 43 => insts::inst_add(thread),
				/* , */ 44 => insts::inst_output_char(thread, &mut self.charout),
				/* - */ 45 => insts::inst_subtract(thread),
				/* / */ 47 => insts::inst_divide(thread),
				/* . */ 46 => insts::inst_output_integer(thread, &mut self.charout),
				/* : */ 58 => insts::inst_duplicate(thread),
				/* ? */ 63 => insts::inst_go_away(thread, self.dialect_mode),
				/* \ */ 92 => insts::inst_swap(thread),
				/* ` */ 96 => insts::inst_greater_than(thread),
				/* g */ 103 => insts::inst_get(thread, &mut self.funge_space, self.dialect_mode),
				/* n */ 110 => insts::inst_clear_stack(thread),
				/* p */ 112 => insts::inst_put(thread, &mut self.funge_space, self.dialect_mode),
				/* q */ 113 => {
					self.programatically_quit = true;
					self.quit_exit_code = thread.stack_stack.pop();
				}
				/* r */ 114 => insts::inst_reflect(thread),
				/* x */ 120 => insts::inst_absolute_delta(thread, self.dialect_mode),
				/* z */ 122 => {/* No-op */}
				
				/* @ */ 64 => panic!("[[Stop instruction]]"),
				/* A...Z */ n @ 65..=90 => {
					// TODO: Implement properly
					// Reflect delta
					let d = &mut thread.delta;
					d.set_x(-d.x());
					d.set_y(-d.y());
					d.set_z(-d.z());
				}
				
				/* 0...9 */ n @ 48..=57 => insts::inst_push_number(thread, n - 48),
				/* a...f */ n @ 97..=102 => insts::inst_push_number(thread, n - 97),
				
				_ => {
					// Invalid (or implmented) instruction so set flag
					valid_instruction = false
				},
			}
		}
		// "Execute" space instruction
		else if instruction == 32 {
			// Set flag because we already set the ip to the next non-space instruction
			move_ip = false;
			
			// Search for next non-space instruction
			let mut pos = thread.ip; // Copy ip
			while {
				pos.add_delta_wrapping(&thread.delta);
				(self.funge_space.read_cell(&pos) == 32)
			} {}
			
			// Set thread ip to next non-space instruction
			thread.ip = pos;
		}
		
		// Move ip by delta
		if move_ip {
			thread.ip.add_delta_wrapping(&thread.delta);
		}
	}
	
	pub fn load_initial_code(&mut self, code: &CodeBuffer) {
		self.load_code(code, FungeAddress::new_xyz(0, 0, 0));
	}
	
	/// Loads the code from the given buffer into this interpreters
	/// funge space where the given position is the top left corner
	/// of the code's bounding box.
	/// 
	/// The cell values in the buffer are copied verbatim into the funge space,
	/// overwriting the previous values. Space characters (ASCII 32)
	/// in the code buffer are treated as essentially transparent.
	/// They do not overwrite cells in the funge space and are simply
	/// ignored, leaving the previous value intact.
	///
	/// This method traverses and processes the code buffer lines until either
	/// none are left or a line has an End terminator. Note that and End terminator
	/// should never occur until the very last code buffer line, although
	/// the existance of an End terminator is optional. In case an End
	/// terminator does occur before the last line, the loading procedure
	/// is simply stopped instead of panicing.
	/// 
	pub fn load_code(&mut self, code: &CodeBuffer, position: FungeAddress) {
		let mut offset = FungeAddress::new_value(0);
		
		// TODO: Handle dimensionality properly
		
		'line_loop:
		for line in code.lines.iter() {
			// Put line values into funge space
			for (x, raw_value) in line.data.iter().enumerate() {
				let cell_value = *raw_value as i32; // Reinterpret u32 codepoint as i32 cell value
				
				// Only overwrite cell if code cell is not a space (32)
				if cell_value != 32 {
					let mut address = FungeAddress::new_xyz(x as i32 + position.x(), position.y(), position.z());
					address.add_wrapping(&offset);
					
					// Write cell
					self.funge_space.write_cell(&address, cell_value);
				}
			}
			
			// Process terminator
			match line.terminator {
				LineTerminator::FeedY => offset.set_y(offset.y().wrapping_add(1i32)),
				LineTerminator::FeedZ => offset.set_z(offset.z().wrapping_add(1i32)),
				LineTerminator::End => break 'line_loop, // Stop iteration on End terminator
			};
		}
	}
	
	pub fn create_thread(&mut self, ip: InstructionPointer, delta: InstructionDelta) {
		let thread = FungeThread::new(ip, delta);
		self.threads.test_add(thread); // TODO: Implement this properly
	}
}

pub type InstructionPointer = FungeAddress;
impl InstructionPointer {
	pub fn add_delta_wrapping(&mut self, delta: &InstructionDelta) {
		self.elements[0] = (Wrapping(self.x()) + Wrapping(delta.x())).0;
		self.elements[1] = (Wrapping(self.y()) + Wrapping(delta.y())).0;
		self.elements[2] = (Wrapping(self.z()) + Wrapping(delta.z())).0;
	}
}

pub type InstructionDelta = Vector3<i32>;
