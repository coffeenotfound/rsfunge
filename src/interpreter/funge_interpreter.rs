use crate::vector::Vector3;
use crate::io::{CodeSource, CodeBuffer};
use crate::interpreter::{FungeAddress, FungeSpace, FungeDim2, SpaceAccessorDim2, ThreadList};
use std::marker::PhantomData;
use std::num::Wrapping;

/// Interpreter for funge*.
/// Instances directly contain the interpretation state.
pub struct FungeInterpreter<'s> {
	code_source: CodeSource,
	threads: ThreadList<'s>,
	
	funge_space: FungeSpace<'s, FungeDim2, i32, SpaceAccessorDim2<i32>>,
}
impl<'s> FungeInterpreter<'s> {
	pub fn new(code_source: CodeSource) -> Self {
		// Instantiate
		let mut interpreter = FungeInterpreter {
			code_source,
			threads: ThreadList::new(),
			funge_space: FungeSpace::new(),
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
				let mut thread = self.threads.get_mut(i).unwrap();
				
				// Run thread tick
				self.execute_thread_tick(thread);
				
				// Increment index
				i += 1;
			}
		}
	}
	
	#[inline]
	fn execute_thread_tick(&mut self, thread: &mut FungeThread) {
		let ip = thread.ip;
		
		// Read instruction cell
		let instruction = self.funge_space.read_cell(&ip);
		
		// Match instruction
		
		// Move ip
//		thread.ip
	}
	
	pub fn load_initial_code(&mut self, code: &CodeBuffer) {
		// 
	}
	
	pub fn create_thread(&mut self, ip: InstructionPointer, delta: InstructionDelta) {
//		let thread = FungeThread::new(ip, delta);
//		self.threads.push(thread);
	}
}

pub struct FungeThread<'s> {
	pub ip: InstructionPointer,
	pub delta: InstructionDelta,
	
	pub string_mode: bool,
	
	_phantom: PhantomData<(&'s u8)>,
	
//	pub page_key_cache: FungeSpacePage
}
impl<'s> FungeThread<'s> {
	pub fn new(ip: InstructionPointer, delta: InstructionDelta) -> Self {
		FungeThread {
			ip,
			delta,
			string_mode: false,
			_phantom: PhantomData,
		}
	}
}

pub type InstructionPointer = FungeAddress;
impl InstructionPointer {
	pub fn add_delta_wrapping(&mut self, delta: InstructionDelta) {
		self.elements[0] = (Wrapping(self.x()) + Wrapping(delta.x())).0;
		self.elements[1] = (Wrapping(self.y()) + Wrapping(delta.y())).0;
		self.elements[2] = (Wrapping(self.z()) + Wrapping(delta.z())).0;
	}
}

pub type InstructionDelta = Vector3<i32>;
