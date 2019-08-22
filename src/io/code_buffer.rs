use crate::vector::Vector3;

///// An unsigned 8-bit ASCII character.
//pub type Ascii = u8;

/// An unsigned 32-bit Unicode codepoint.
pub type Codepoint = u32;

/// CodeBuffer contains a loaded funge program (one source file)
pub struct CodeBuffer {
	pub lines: Vec<CodeBufferLine>,
	pub bounding_size: Vector3<u32>,
	pub dimensions: u32,
}
impl CodeBuffer {
	
}

pub struct CodeBufferLine {
	pub data: Vec<Codepoint>,
	pub terminator: LineTerminator,
}
impl CodeBufferLine {
	pub fn new(capacity: u32) -> Self {
		return CodeBufferLine {
			data: vec![32; (capacity as usize)],
			terminator: LineTerminator::End,
		}
	}
}

pub enum LineTerminator {
	NextX,
	NextY,
	NextZ,
	End,
}

//pub enum CodeBufferData {
//	Ascii(Vec<Ascii>),
//	Unicode(Vec<Codepoint>),
//}