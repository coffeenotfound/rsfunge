
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vector3<T: Copy> {
	pub elements: [T; 3],
}

impl<T: Copy> Vector3<T> {
	#[inline(always)]
	pub fn x(&self) -> T {
		self.elements[0]
	}
	
	#[inline(always)]
	pub fn y(&self) -> T {
		self.elements[1]
	}
	
	#[inline(always)]
	pub fn z(&self) -> T {
		self.elements[2]
	}
	
	#[inline(always)]
	pub fn set_xyz(&mut self, x: T, y: T, z: T) -> &mut Self {
		self.elements = [x, y, z];
		return self;
	}
	
	#[inline(always)]
	pub fn set_x(&mut self, x: T) -> &mut Self {
		self.elements[0] = x;
		return self;
	}
	
	#[inline(always)]
	pub fn set_y(&mut self, y: T) -> &mut Self {
		self.elements[1] = y;
		return self;
	}
	
	#[inline(always)]
	pub fn set_z(&mut self, z: T) -> &mut Self {
		self.elements[2] = z;
		return self;
	}
}
