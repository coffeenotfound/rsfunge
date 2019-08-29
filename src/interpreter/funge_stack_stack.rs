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
	
	pub fn new() -> Self {
		let mut data = Vec::with_capacity(8);
		data.push(FungeStack::new());
		
		return FungeStackStack {
			data,
		};
	}
}
