use crate::interpreter::{FungeThread, FungeInterpreter};

/// 33: Logical not (!)
#[inline(always)]
pub fn inst_logical_not(thread: &mut FungeThread) { 
	let cell = thread.stack_stack.pop();
	thread.stack_stack.push(if cell == 0 { 1 } else { 0 });
}


/// 48...57: Push Zero, .., Push Niner (0, .., 9)
#[inline(always)]
pub fn inst_push_number(thread: &mut FungeThread, number: i32) {
	thread.stack_stack.push(number);
}

/// 58: Duplicate (:)
#[inline(always)]
pub fn inst_duplicate(thread: &mut FungeThread) {
	let cell = thread.stack_stack.pop();
	thread.stack_stack.push(cell);
	thread.stack_stack.push(cell);
}

/// 63: Go away (?)
#[inline(always)]
pub fn inst_go_away(thread: &mut FungeThread) {
	// TODO: Implement
}

/// 96: Greater than (`)
#[inline(always)]
pub fn inst_greater_than(thread: &mut FungeThread) {
	let (first, second) = thread.stack_stack.pop_two();
	thread.stack_stack.push(if second > first { 1 } else { 0 });
}
