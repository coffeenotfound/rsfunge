
#[derive(Copy, Clone)]
pub struct FingerprintName {
	fid: u32,
}

impl FingerprintName {
	pub fn from_fid(fid: u32) -> Self {
		FingerprintName {
			fid
		}
	}
	
	pub fn get_ascii_name(&self) -> [u8; 4] {
		let bytes: [u8; 4] = self.fid.to_le_bytes();
		return bytes;
	}
	
	pub fn get_fid(&self) -> u32 {
		self.fid
	}
}
