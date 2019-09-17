use crate::interpreter::fingerprint::{Fingerprint, SimpleFingerprint, FingerprintInstFunction, FingerprintName};
use crate::interpreter::FungeThread;

pub fn create_null_fingerprint() -> Box<dyn Fingerprint<'static>> {
	// Create inst array
	let inst_array: [Option<&FingerprintInstFunction>; 26] = [Some(&(inst_null as FingerprintInstFunction)); 26];
	
	// Build fingerprint object
	let name = FingerprintName::from_fid(0x4e554c4c);
	return Box::new(SimpleFingerprint::new(name, inst_array));
}

//pub trait NullFingerprint<'f> : Fingerprint<'f> {
//	
//}
//
//impl<'f> NullFingerprint<'f> for SimpleFingerprint<'f> {
//	
//}

fn inst_null(thread: &mut FungeThread) {
	// Do nothing
}
