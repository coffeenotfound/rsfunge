use crate::interpreter::{FungeThread};
use std::io::Write;

/// 33: Logical not (!)
#[inline(always)]
pub fn inst_logical_not(thread: &mut FungeThread) { 
	let cell = thread.stack_stack.pop();
	thread.stack_stack.push(if cell == 0 { 1 } else { 0 });
}

/// 37: Remainder (%)
#[inline(always)]
pub fn inst_remainder(thread: &mut FungeThread) {
	let (a, b) = thread.stack_stack.pop_two();
	
	// Calculate the remainder of the division of the values with explicit overflow wrapping
	let c = b.wrapping_rem(a); // As per spec: Remainder from dividing second by first
	thread.stack_stack.push(c);
}

/// 42: Multiply (*)
#[inline(always)]
pub fn inst_multiply(thread: &mut FungeThread) {
	let (a, b) = thread.stack_stack.pop_two();
	
	// Mutliply the values with explicit overflow wrapping
	let c = a.wrapping_mul(b);
	thread.stack_stack.push(c);
}

/// 43: Add (+)
#[inline(always)]
pub fn inst_add(thread: &mut FungeThread) {
	let (a, b) = thread.stack_stack.pop_two();
	
	// Add the values with explicit overflow wrapping
	let c = a.wrapping_add(b);
	thread.stack_stack.push(c);
}

/// 45: Subtract (-)
#[inline(always)]
pub fn inst_subtract(thread: &mut FungeThread) {
	let (a, b) = thread.stack_stack.pop_two();
	
	// Subtract the values with explicit overflow wrapping
	let c = b.wrapping_sub(a); // As per spec: Subtract first from second
	thread.stack_stack.push(c);
}

/// 47: Divide (/)
#[inline(always)]
pub fn inst_divide(thread: &mut FungeThread) {
	let (a, b) = thread.stack_stack.pop_two();
	
	// Divide the values with explicit overflow wrapping
	let c = b.wrapping_div(a); // As per spec: Divide second by first
	thread.stack_stack.push(c);
}

/// 44: Output char (,)
#[inline(always)]
pub fn inst_output_char(thread: &mut FungeThread, charout: &mut dyn Write) {
	let cell = thread.stack_stack.pop();
	
	// Print the cell by converting it to a unicode scalar ("char") if possible
	let mut char = '?';
	if cell > 0 {
		if let Some(c) = std::char::from_u32(cell as u32) {
			char = c;
		}
	}
	
	if let Err(e) = write!(charout, "{}", char) {
		// Do nothing on error
	}
}

/// 46: Output integer (.)
#[inline(always)]
pub fn inst_output_integer(thread: &mut FungeThread, charout: &mut dyn Write) {
	let cell = thread.stack_stack.pop();
	
	if let Err(e) = write!(charout, "{} ", cell as i32) { // As per spec write a space after the decimal number
		// Do nothing on error
	}
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
