
pub struct Fingerprint {
	name: [u8; 4],
	id: u32,
}

impl Fingerprint {
	pub fn get_name(&self) -> &[u8; 4] {
		&self.name
	}
	
	pub fn get_id(&self) -> u32 {
		self.id
	}
}
