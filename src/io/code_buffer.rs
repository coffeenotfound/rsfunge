use crate::vector::Vector3;
use std::marker::PhantomData;

///// An unsigned 8-bit ASCII character.
//pub type Ascii = u8;

/// An unsigned 32-bit Unicode codepoint.
pub type Codepoint = u32;

/// CodeBuffer contains a loaded funge program (one source file)
pub struct CodeBuffer {
	pub lines: Vec<CodeBufferLine>,
	pub bounding_box: Vector3<u32>,
	pub dimensionality: u32,
	
	_phantom: PhantomData<()>,
}
impl CodeBuffer {
	pub fn new(lines: Vec<CodeBufferLine>, bounding_box: Vector3<u32>, dimensionality: u32) -> Self {
		return CodeBuffer {
			lines,
			bounding_box,
			dimensionality,
			_phantom: PhantomData,
		};
	}
}

pub struct CodeBufferLine {
	pub data: Vec<Codepoint>,
	pub terminator: LineTerminator,
}
impl CodeBufferLine {
	pub fn new(capacity: u32) -> Self {
		return CodeBufferLine {
			data: Vec::with_capacity(capacity as usize),
			terminator: LineTerminator::End,
		}
	}
}

#[derive(Copy, Clone)]
pub enum LineTerminator {
	FeedY,
	FeedZ,
	End,
}

//pub enum CodeBufferData {
//	Ascii(Vec<Ascii>),
//	Unicode(Vec<Codepoint>),
//}
