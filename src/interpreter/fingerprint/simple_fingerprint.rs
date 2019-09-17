use crate::interpreter::fingerprint::{Fingerprint, FingerprintName, FingerprintInstFunction};

pub struct SimpleFingerprint<'f> {
	name: FingerprintName,
	inst_functions: [Option<&'f FingerprintInstFunction>; 26],
}

impl<'f> SimpleFingerprint<'f> {
	pub fn new(name: FingerprintName, insts: [Option<&'f FingerprintInstFunction>; 26]) -> SimpleFingerprint<'f> {
		SimpleFingerprint {
			name,
			inst_functions: insts,
		}
	}
}

impl<'f> Fingerprint<'f> for SimpleFingerprint<'f> {
	fn get_name(&self) -> FingerprintName {
		self.name
	}
	
	fn get_alphabet_instructions(&self, dest: &mut [Option<&'f FingerprintInstFunction>; 26]) {
		dest.copy_from_slice(&self.inst_functions);
	}
}
