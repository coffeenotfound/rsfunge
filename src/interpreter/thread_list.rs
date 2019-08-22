use crate::interpreter::FungeThread;

/// A list of funge threads kept in the right order.
pub struct ThreadList<'s> {
	threads: Vec<FungeThread<'s>>,
}
impl<'s> ThreadList<'s> {
	pub fn new() -> Self {
		ThreadList {
			threads: Vec::with_capacity(8),
		}
	}
	
	pub fn get_mut(&mut self, index: u32) -> Option<&mut FungeThread<'s>> {
		return self.threads.get_mut(index as usize);
	}
	
	pub fn get(&self, index: u32) -> Option<&FungeThread<'s>> {
		return self.threads.get(index as usize);
	}
}

//impl<'s> std::ops::Index<usize> for ThreadList<'s> {
//	type Output = ThreadList<'s>;
//	
//	fn index(&self, index: usize) -> &Self::Output {
//		
//	}
//}
//
//impl<'s> std::ops::IndexMut<usize> for ThreadList<'s> {
//	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//		
//	}
//}
