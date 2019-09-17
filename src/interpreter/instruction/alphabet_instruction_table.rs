use crate::interpreter::fingerprint::{Fingerprint, FingerprintInstFunction};
use std::rc::Rc;

pub struct AlphabetInstructionTable<'f> {
	entries: Vec<AlphabetInstructionTableEntry<'f>>,
	
	// TODO: Change this whole inst table cache logic to make it actually good
	cached_inst_table: [Option<&'f FingerprintInstFunction>; 26],
}

impl<'f> AlphabetInstructionTable<'f> {
	pub fn new() -> Self {
		AlphabetInstructionTable {
			entries: Vec::with_capacity(16),
			cached_inst_table: [None; 26],
		}
	}
	
	pub fn push_fingerprint(&mut self, fingerprint: &'_ Rc<dyn Fingerprint<'f>>) {
		// Get instructions
		let mut inst_array: [Option<&FingerprintInstFunction>; 26] = [None; 26];
		fingerprint.get_alphabet_instructions(&mut inst_array);
		
		// Make entry
		let entry = AlphabetInstructionTableEntry {
			fingerprint: Rc::clone(fingerprint),
			inst_array,
		};
		
		// Update inst table cache
		for (i, f) in inst_array.iter().enumerate() {
			match f {
				Some(inst) => {
					self.cached_inst_table[i] = Some(*inst);
				}
				None => {}
			}
		}
		
		// Push fingerprint onto fingerprint stack
		self.entries.push(entry);
	}
	
	pub fn pop_fingerprint(&mut self) -> Option<Rc<dyn Fingerprint<'f>>> {
		// Pop fingerprint // TODO: Improve this
		if let Some(e) = self.entries.pop() {
			// Unoverload fingerprint instructions
			for i in 0..26 {
				if e.inst_array[i].is_some() {
					'search_loop:
					for j in (0..(self.entries.len() - 1)).rev() {
						if self.entries[j].inst_array[i].is_some() {
							self.cached_inst_table[i] = self.entries[j].inst_array[i];
							break 'search_loop;
						}
					}
				}
			}
			
			return Some(e.fingerprint);
		}
		else {
			return None;
		}
	}
	
	pub fn find_inst(&self, index: u32) -> Option<&'f FingerprintInstFunction> {
		*self.cached_inst_table.get(index as usize).unwrap()
	}
}

pub struct AlphabetInstructionTableEntry<'f> {
	pub fingerprint: Rc<dyn Fingerprint<'f>>,
	pub inst_array: [Option<&'f FingerprintInstFunction>; 26],
	
}
