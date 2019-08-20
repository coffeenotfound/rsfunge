use crate::vector::Vector3;
use crate::io::CodeSource;
use crate::interpreter::{FungeAddress, FungeSpace, FungeDim2, SpaceAccessorDim2};
use std::marker::PhantomData;

/// Interpreter for funge*.
/// Instances directly contain the interpretation state.
pub struct FungeInterpreter<'s> {
	code_source: CodeSource,
	threads: Vec<FungeThread<'s>>,
	
	funge_space: FungeSpace<'s, FungeDim2, i32, SpaceAccessorDim2<i32>>,
}
impl<'s> FungeInterpreter<'s> {
	pub fn start_thread(&mut self, ip: InstructionPointer, delta: InstructionDelta) {
		let thread = FungeThread::new(ip, delta);
		self.threads.push(thread);
	}
}

pub struct FungeThread<'s> {
	pub ip: InstructionPointer,
	pub delta: InstructionDelta,
	_unused: PhantomData<(&'s u8)>,
	
//	pub page_key_cache: FungeSpacePage
}
impl<'s> FungeThread<'s> {
	pub fn new(ip: InstructionPointer, delta: InstructionDelta) -> Self {
		FungeThread {
			ip,
			delta,
			_unused: PhantomData,
		}
	}
}

pub type InstructionPointer = FungeAddress;
pub type InstructionDelta = Vector3<i32>;
