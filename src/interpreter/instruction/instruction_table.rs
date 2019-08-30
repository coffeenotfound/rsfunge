use crate::interpreter::instruction::{InstructionAlphabet};

#[deprecated]
pub struct InstructionTable<'h> {
	alphabet_stack: Vec<&'h InstructionAlphabet>,
}

impl<'h> InstructionTable<'h> {
	pub fn push_alphabet(&mut self, alphabet: &'h InstructionAlphabet) {
		self.alphabet_stack.push(alphabet);
	}
	
	pub fn pop_alphabet(&mut self) -> Option<&'h InstructionAlphabet> {
		return self.alphabet_stack.pop();
	}
	
	pub fn top_alphabet(&self) -> Option<&'h InstructionAlphabet> {
		// Return top of stack and dereference pointer to alphabet pointer
		return self.alphabet_stack.last().copied();
	}
}
