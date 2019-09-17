use crate::interpreter::fingerprint::Fingerprint;
use std::collections::HashMap;
use std::rc::Rc;

pub struct FingerprintRegistry<'f> {
	fingerprint_map: HashMap<u32, Rc<dyn Fingerprint<'f>>>,
}

impl<'f> FingerprintRegistry<'f> {
	pub fn new() -> Self {
		FingerprintRegistry {
			fingerprint_map: HashMap::new(),
		}
	}
	
	pub fn register_fingerprint(&mut self, fingerprint: Rc<dyn Fingerprint<'f>>) {
		// Put into registry map
		self.fingerprint_map.insert(fingerprint.get_name().get_fid(), fingerprint);
	}
	
	pub fn find_fingerprint(&mut self, fid: u32) -> Option<&Rc<dyn Fingerprint<'f>>> {
		return self.fingerprint_map.get(&fid);
	}
}
