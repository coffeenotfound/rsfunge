use crate::vector::Vector3;
use crate::io::code_source::CodeSource;
use crate::interpreter::FungeAddress;

/// Interpreter for funge*.
/// Instances directly contain the interpretation state.
pub struct FungeInterpreter {
	code_source: CodeSource,
	threads: Vec<FungeThread>,
}
impl FungeInterpreter {
	pub fn start_thread(&mut self, ip: InstructionPointer, delta: InstructionDelta) {
		let mut thread = FungeThread::new(ip, delta);
		self.threads.push(thread);
	}
}

pub struct FungeThread {
	pub ip: InstructionPointer,
	pub delta: InstructionDelta,
}
impl FungeThread {
	pub fn new(ip: InstructionPointer, delta: InstructionDelta) -> Self {
		FungeThread {
			ip,
			delta,
		}
	}
}

pub type InstructionPointer = FungeAddress;
pub type InstructionDelta = Vector3<i32>;
