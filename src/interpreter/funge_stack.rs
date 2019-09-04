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
	
	/// Peeks the nth value from the top of the stack,
	/// where 0 is the top most value.
	#[inline]
	pub fn peek_nth(&self, n: u32) -> Option<V> {
		let last = self.data.len() - 1;
		return self.data.get(last - n as usize).cloned();
	}
	
	#[inline]
	pub fn clear(&mut self) {
		self.data.clear();
	}
	
	#[inline]
	pub fn depth(&self) -> u32 {
		self.data.len() as u32
	}
	
	/// Transfers |count| cells from this stacks top in non-reverse order
	/// to the given other stack. If this stack has less elements that |count|
	/// then the transferred cells will be at the top of the other stack and
	/// the bottom filled with zeroes until the other stack has received
	/// |count| cells.
	pub fn transfer_to_stack(&mut self, other: &mut FungeStack<V>, count: u32) {
		let real_transfer_count = std::cmp::min(self.depth(), count);
		let zeroes_count = count - real_transfer_count;
		
		// Reserve capacity in other stack
		other.data.reserve(count as usize);
		
		// Push zeroes if needed
		for _ in 0..zeroes_count {
			other.push(V::default());
		}
		
		// Copy cells from this stack to other
		let transfer_start_pos = self.data.len() - real_transfer_count as usize;
		other.data.extend_from_slice(&self.data[transfer_start_pos..]);
		
		// Truncate own data to simulate cell transfer
		self.data.truncate(self.data.len() - real_transfer_count as usize)
	}
	
	pub fn new() -> Self {
		return FungeStack {
			data: Vec::new(),
		}
	}
}
