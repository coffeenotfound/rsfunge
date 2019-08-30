use std::num::Wrapping;
use crate::interpreter::{FungeAddress, FungeDim2, FungeSpace, SpaceAccessorDim2, ThreadList};
use crate::io::{CodeBuffer, CodeSource};
use crate::vector::Vector3;
use crate::interpreter::instruction::insts;
use std::io::{Write, Read};

/// Interpreter for funge*.
/// Instances directly contain the interpretation state.
pub struct FungeInterpreter<'s, 'io> {
	code_source: CodeSource,
	threads: ThreadList<'s>,
	funge_space: FungeSpace<'s, FungeDim2, i32, SpaceAccessorDim2<i32>>,
	
	charout: &'io mut dyn Write,
	charin: &'io mut dyn Read,
}

impl<'s, 'io> FungeInterpreter<'s, 'io> {
	pub fn new(code_source: CodeSource, charout: &'io mut dyn Write, charin: &'io mut dyn Read) -> Self {
		// Instantiate
		let mut interpreter = FungeInterpreter {
			code_source,
			threads: ThreadList::new(),
			funge_space: FungeSpace::new(),
			charout,
			charin,
		};
		
		// Create initial thread
		interpreter.create_thread(Vector3::new(), Vector3::new_xyz(1, 0, 0));
		
		return interpreter;
	}
	
	/// Starts the execution of this interpreter by transferring
	/// control over to it's main loop.
	/// This method does not return aslong as the interpreter
	/// is running.
	pub fn start_execution(&mut self) {
		// Main execution loop
		loop {
			// Execute next tick with each thread
			let mut i = 0;
			loop {
//				let thread = self.threads.get_mut(i).unwrap();
				
				// Run thread tick
				self.execute_thread_tick(i);
				
				// Increment index
				i += 1;
			}
		}
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
				/* 0...9 */ n @ 48..=57 => insts::inst_push_number(thread, n - 48),
				/* a...f */ n @ 97..=102 => insts::inst_push_number(thread, n - 97),
				/* ! */ 33 => insts::inst_logical_not(thread),
				/* # */ 35 => thread.ip.add_delta_wrapping(&thread.delta),
				/* . */ 46 => insts::inst_output_integer(self, thread),
				/* ` */ 96 => insts::inst_greater_than(thread),
				/* z */ 122 => {/* No-op */}
				
				/* @ */ 64 => panic!("[[Stop instruction]]"),
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
		// 
	}
	
	pub fn create_thread(&mut self, ip: InstructionPointer, delta: InstructionDelta) {
//		let thread = FungeThread::new(ip, delta);
//		self.threads.push(thread);
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
