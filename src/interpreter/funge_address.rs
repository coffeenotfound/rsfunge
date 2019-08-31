use std::hash;
use crate::vector::Vector3;
use std::num::Wrapping;

pub type FungeAddress = Vector3<i32>;

impl FungeAddress {
	pub fn add_wrapping(&mut self, other: &FungeAddress) {
		self.elements[0] = (Wrapping(self.x()) + Wrapping(other.x())).0;
		self.elements[1] = (Wrapping(self.y()) + Wrapping(other.y())).0;
		self.elements[2] = (Wrapping(self.z()) + Wrapping(other.z())).0;
	}
}

/// Hash impl for InstructionPointer
impl hash::Hash for FungeAddress {
	fn hash<H: hash::Hasher>(&self, state: &mut H) {
		state.write_i32(self.elements[0]);
		state.write_i32(self.elements[1]);
		state.write_i32(self.elements[2]);
	}
}

pub type FungePageAddress = Vector3<i32>;
