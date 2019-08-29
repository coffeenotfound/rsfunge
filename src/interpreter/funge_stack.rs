use crate::interpreter::FungeValue;

pub struct FungeStack<V: FungeValue = i32> {
	data: Vec<V>,
}

impl<V: FungeValue> FungeStack<V> {
	#[inline]
	pub fn push(&mut self, value: V) {
		self.data.push(value);
	}
	
	#[inline]
	pub fn pop(&mut self) -> V {
		return self.data.pop().unwrap_or(V::from(0));
	}
	
	#[inline]
	pub fn pop_two(&mut self) -> (V, V) {
		return (self.pop(), self.pop());
	}
	
	#[inline]
	pub fn try_pop(&mut self) -> Option<V> {
		return self.data.pop();
	}
	
	pub fn new() -> Self {
		return FungeStack {
			data: Vec::new(),
		}
	}
}
