use crate::interpreter::{InstructionPointer, InstructionDelta, FungeAddress, FungeStackStack};
use std::marker::PhantomData;

pub struct FungeThread<'s> {
	pub ip: InstructionPointer,
	pub delta: InstructionDelta,
	
	pub stack_stack: FungeStackStack,
	
//	pub stroage_offset: FungeAddress,
	pub string_mode: bool,
	
	_phantom: PhantomData<(&'s u8)>,
	
//	pub page_key_cache: FungeSpacePage
}

impl<'s> FungeThread<'s> {
	pub fn new(ip: InstructionPointer, delta: InstructionDelta) -> Self {
		FungeThread {
			ip,
			delta,
			stack_stack: FungeStackStack::new(),
//			stroage_offset: FungeAddress::new_value(0),
			string_mode: false,
			_phantom: PhantomData,
		}
	}
	
	pub fn get_storage_offset(&self) -> FungeAddress {
		// TODO: Implement storage offset stack or however it works
		return FungeAddress::new_value(0);
	}
}
