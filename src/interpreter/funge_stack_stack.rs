use crate::interpreter::{FungeValue, FungeStack};

/// A stack of funge stacks, making it the stack stack.
/// A stack stack always contains atleast one stack.
/// NOTE: The above is important!
pub struct FungeStackStack<V: FungeValue = i32> {
	data: Vec<FungeStack<V>>,
}

impl<V: FungeValue> FungeStackStack<V> {
	#[inline]
	pub fn top_stack(&mut self) -> &mut FungeStack<V> {
		// Because we must always have atleast one stack we can just unwrap the Option
		return self.data.last_mut().unwrap();
	}
	
	#[inline]
	pub fn second_stack(&mut self) -> Option<&mut FungeStack<V>> {
		if self.data.len() >= 2 {
			unsafe {
				let len = self.data.len();
				return Some(self.data.get_unchecked_mut(len - 2));
			}
		}
		else {
			return None;
		}
	}
	
	#[inline]
	pub fn push(&mut self, value: V) {
		let top = self.top_stack();
		top.push(value);
	}
	
	#[inline]
	pub fn pop(&mut self) -> V {
		let top = self.top_stack();
		return top.pop();
		
//		return match top {
//			Some(top) => top.pop(),
//			None => V::from(0),
//		};
	}
	
	#[inline]
	pub fn pop_two(&mut self) -> (V, V) {
		let top = self.top_stack();
		return top.pop_two();
		
//		return match top {
//			Some(top) => top.pop(),
//			None => (V::from(0), V::from(0)),
//		};
	}
	
	#[inline]
	pub fn push_stack(&mut self, stack: FungeStack<V>) {
		self.data.push(stack);
	}
	
	#[inline]
	pub fn pop_stack(&mut self) -> Option<FungeStack<V>> {
		if self.data.len() > 1 {
			self.data.pop()
		}
		else {
			None
		}
	}
	
	/// Returns the nth stack from the top in this stack stack or None
	/// if the index is greater than or equal the number of stacks.
	/// 0 is the TOSS, 1 the SOSS, etc.
	#[inline]
	pub fn nth_stack(&mut self, index: u32) -> Option<&mut FungeStack<V>> {
		let last = self.data.len() - 1;
		return self.data.get_mut(last - index as usize);
	}
	
	#[inline]
	pub fn num_stacks(&self) -> u32 {
		self.data.len() as u32
	}
	
	pub fn new() -> Self {
		let mut data = Vec::with_capacity(8);
		data.push(FungeStack::new());
		
		return FungeStackStack {
			data,
		};
	}
}
