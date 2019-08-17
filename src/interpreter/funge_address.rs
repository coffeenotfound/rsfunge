use std::hash;
use crate::vector::Vector3;

pub type FungeAddress = Vector3<u32>;

/// Hash impl for InstructionPointer
impl hash::Hash for FungeAddress {
	fn hash<H: hash::Hasher>(&self, state: &mut H) {
		state.write_u32(self.elements[0]);
		state.write_u32(self.elements[1]);
		state.write_u32(self.elements[2]);
	}
}
