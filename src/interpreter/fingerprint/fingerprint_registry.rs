use crate::interpreter::fingerprint::Fingerprint;
use std::collections::HashMap;

pub struct FingerprintRegistry {
	fingerprint_map: HashMap<u32, Fingerprint>,
}

impl FingerprintRegistry {
	pub fn register_fingerprint(&mut self, fingerprint: Fingerprint) {
		// Put into registry map
		self.fingerprint_map.insert(fingerprint.get_id(), fingerprint);
	}
	
	pub fn find_fingerprint(&mut self, id: u32) -> Option<&Fingerprint> {
		return self.fingerprint_map.get(&id);
	}
}
