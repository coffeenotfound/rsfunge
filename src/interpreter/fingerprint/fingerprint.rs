use crate::interpreter::fingerprint::{FingerprintName, FingerprintInstFunction};

pub trait Fingerprint<'f> {
	fn get_name(&self) -> FingerprintName
	;
	
	fn get_alphabet_instructions(&self, dest: &mut [Option<&'f FingerprintInstFunction>; 26])
	;
}
