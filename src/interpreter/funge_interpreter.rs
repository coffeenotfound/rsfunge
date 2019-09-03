use crate::interpreter::{FungeAddress, FungeDim2, FungeSpace, SpaceAccessorDim2, ThreadList, FungeThread};
use crate::io::{CodeBuffer, CodeSource, LineTerminator};
use crate::vector::Vector3;
use crate::interpreter::instruction::insts;
use std::io::{Stdin, Stdout};
use std::num::Wrapping;
use crate::FungeDialect;

/// The handprint of rsfunge, "RSFN"
pub const RSFUNGE_HANDPRINT: u32 = 0x5253464e;
pub const RSFUNGE_VERSION: u32 = 100;

/// Interpreter for funge*.
/// Instances directly contain the interpretation state.
pub struct FungeInterpreter<'s> {
	threads: ThreadList<'s>,
	funge_space: FungeSpace<'s, FungeDim2, i32, SpaceAccessorDim2<i32>>,
	dialect_mode: FungeDialect,
	code_source: CodeSource,
	
	charout: Stdout,
	charin: Stdin,
	
	programatically_quit: bool,
	quit_exit_code: i32,
}

impl<'s> FungeInterpreter<'s> {
	pub fn new(code_source: CodeSource, charout: Stdout, charin: Stdin) -> Self { //charout: &'io mut dyn Write, charin: &'io mut dyn Read
		// Instantiate
		let mut interpreter = FungeInterpreter {
			threads: ThreadList::new(),
			funge_space: FungeSpace::new(),
			dialect_mode: code_source.get_dialect(),
			code_source,
			
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
				
				// Exit
				if self.programatically_quit {
					break 'mainloop;
				}
				
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
		
		if !thread.string_mode {
//			let mut move_ip = true;
			let mut valid_instruction = false;
			
			let mut instruction;
			while {
				// Read instruction cell
				instruction = self.funge_space.read_cell(&thread.ip);
				
				// Pseudo-execute space and semicolon instruction
				// Both take zero ticks. Do as long as there is still some left
				match instruction {
					/* space */ 32 => {{
						// Search for next non-space instruction
						let mut pos = thread.ip; // Copy ip
						while {
							pos.add_delta_wrapping(&thread.delta);
							(self.funge_space.read_cell(&pos) == 32)
						} {}
						
						// Set thread ip to next non-space instruction
						thread.ip = pos;
					}; true},
					
					/* ; */ 59 => {{
						// Search for next non-space instruction
						let mut pos = thread.ip; // Copy ip
						while {
							pos.add_delta_wrapping(&thread.delta);
							(self.funge_space.read_cell(&pos) != 59)
						} {}
						
						// Move ip to next actual instruction
						pos.add_delta_wrapping(&thread.delta);
						
						// Set thread ip to next non-space instruction
						thread.ip = pos;
					}; true},
					_ => false,
				}
			} {}
			
			// Execute instruction
			if (32 < instruction) && (instruction <= 126) {
				valid_instruction = true;
				
				match instruction {
					/* ! */ 33 => insts::inst_logical_not(thread),
					/* " */ 34 => {
						// Enable string mode
						thread.string_mode = true;
					}
					/* # */ 35 => thread.ip.add_delta_wrapping(&thread.delta),
					/* $ */ 36 => insts::inst_pop(thread),
					/* % */ 37 => insts::inst_remainder(thread),
					/* & */
					/* ' */ 39 => insts::inst_fetch_character(thread, &mut self.funge_space),
					/* ( */
					/* ) */
					/* * */ 42 => insts::inst_multiply(thread),
					/* + */ 43 => insts::inst_add(thread),
					/* , */ 44 => insts::inst_output_char(thread, &mut self.charout),
					/* - */ 45 => insts::inst_subtract(thread),
					/* . */ 46 => insts::inst_output_integer(thread, &mut self.charout),
					/* / */ 47 => insts::inst_divide(thread),
					/* -> (0...9) */
					/* : */ 58 => insts::inst_duplicate(thread),
					/* -> (;) */
					/* < */ 60 => insts::inst_go_west(thread),
					/* = */
					/* > */ 62 => insts::inst_go_east(thread),
					/* ? */ 63 => insts::inst_go_away(thread, self.dialect_mode),
					/* @ */ 64 => panic!("[[Stop instruction]]"),
					/* -> (A...Z) */
					/* [ */ 91 => valid_instruction = insts::inst_turn_left(thread, self.dialect_mode),
					/* \ */ 92 => insts::inst_swap(thread),
					/* ] */ 93 => valid_instruction = insts::inst_turn_right(thread, self.dialect_mode),
					/* ^ */ 94 => valid_instruction = insts::inst_go_north(thread, self.dialect_mode),
					/* _ */ 95 => insts::inst_east_west_if(thread),
					/* ` */ 96 => insts::inst_greater_than(thread),
					/* -> (a...f) */
					/* g */ 103 => insts::inst_get(thread, &mut self.funge_space, self.dialect_mode),
					/* h */ 104 => valid_instruction = insts::inst_go_high(thread, self.dialect_mode),
					/* i */
					/* j */
					/* k */
					/* l */ 108 => valid_instruction = insts::inst_go_low(thread, self.dialect_mode),
					/* m */ 109 => valid_instruction = insts::inst_high_low_if(thread, self.dialect_mode),
					/* n */ 110 => insts::inst_clear_stack(thread),
					/* o */
					/* p */ 112 => insts::inst_put(thread, &mut self.funge_space, self.dialect_mode),
					/* q */ 113 => {
						self.programatically_quit = true;
						self.quit_exit_code = thread.stack_stack.pop();
					}
					/* r */ 114 => insts::inst_reflect(thread),
					/* s */
					/* t */
					/* u */
					/* v */ 118 => valid_instruction = insts::inst_go_south(thread, self.dialect_mode),
					/* w */
					/* x */ 120 => insts::inst_absolute_delta(thread, self.dialect_mode),
					/* y */
					/* z */ 122 => {/* No-op */}
					/* { */
					/* | */ 124 => valid_instruction = insts::inst_north_south_if(thread, self.dialect_mode),
					/* } */
					/* ~ */
					
					/* A...Z */ n @ 65..=90 => {
						// TODO: Implement alphabet instructions properly
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
			
			// If an invalid instruction was encountered, act as reflect
			if !valid_instruction {
				insts::_reflect_delta(&mut thread.delta);
			}
			
			// Move ip by delta
			thread.ip.add_delta_wrapping(&thread.delta);
		}
		else { // If in string mode
			// Read instruction cell
			let instruction = self.funge_space.read_cell(&thread.ip);
			
			match instruction {
				/* space */ 32 => {
					// Search for next non-space instruction
					let mut pos = thread.ip; // Copy ip
					while {
						pos.add_delta_wrapping(&thread.delta);
						(self.funge_space.read_cell(&pos) == 32)
					} {}
					
					// Set thread ip to next non-space instruction
					thread.ip = pos;
				}
				/* " */ 34 => {
					// Disable string mode
					thread.string_mode = false;
					
					// Advance ip
					thread.ip.add_delta_wrapping(&thread.delta);
				}
				_ => {
					// Push char as value onto the stack
					thread.stack_stack.push(instruction);
					
					// Advance ip
					thread.ip.add_delta_wrapping(&thread.delta);
				}
			}
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
