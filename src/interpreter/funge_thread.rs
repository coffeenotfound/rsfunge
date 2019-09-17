use crate::interpreter::{InstructionPointer, InstructionDelta, FungeAddress, FungeStackStack};
use std::marker::PhantomData;
use crate::interpreter::instruction::AlphabetInstructionTable;

pub struct FungeThread<'s, 'f> {
	pub ip: InstructionPointer,
	pub delta: InstructionDelta,
	
	pub stack_stack: FungeStackStack,
	
	pub alphabet_inst_table: AlphabetInstructionTable<'f>,
	
	pub stroage_offset: FungeAddress,
	pub string_mode: bool,
	
	_phantom: PhantomData<(&'s u8)>,
	
//	pub page_key_cache: FungeSpacePage,
}

impl<'s, 'f> FungeThread<'s, 'f> {
	pub fn new(ip: InstructionPointer, delta: InstructionDelta) -> Self {
		FungeThread {
			ip,
			delta,
			stack_stack: FungeStackStack::new(),
			alphabet_inst_table: AlphabetInstructionTable::new(),
			stroage_offset: FungeAddress::new_value(0),
			string_mode: false,
			_phantom: PhantomData,
		}
	}
	
	pub fn get_storage_offset(&self) -> FungeAddress {
		return self.stroage_offset;
	}
}
