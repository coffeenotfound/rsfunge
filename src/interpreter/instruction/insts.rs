use crate::interpreter::{FungeThread, InstructionDelta, FungeSpace, FungeAddress, FungeDimension, FungeSpaceAccessor, FungeStack, InstructionPointer};
use crate::FungeDialect;
use rand::Rng;
use std::io::{Stdout, Stdin, Read, Write};

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

/// 39: Fetch character (')
#[inline(always)]
pub fn inst_fetch_character<N, A>(thread: &mut FungeThread, funge_space: &mut FungeSpace<N, i32, A>) where N: FungeDimension, A: FungeSpaceAccessor<N, i32> {
	// Get next pos
	let mut pos: FungeAddress = thread.ip;
	pos.add_wrapping(&thread.delta);
	
	// Load character value at pos
	let char = funge_space.read_cell(&pos);
	
	// Push onto stack
	thread.stack_stack.push(char);
	
	// Set ip to pos (so it gets moved again after this function and skips over the read character cell)
	thread.ip = pos;
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
pub fn inst_output_char(thread: &mut FungeThread, charout: &mut Stdout) {
	let cell = thread.stack_stack.pop();
	
	// Print the cell by converting it to a unicode scalar ("char") if possible
	let mut char = '?';
	if cell > 0 {
		if let Some(c) = std::char::from_u32(cell as u32) {
			char = c;
		}
	}
	
	// Act as `r` if the write failed
	if let Err(_) = write!(charout, "{}", char) {
		// Reflect delta
		_reflect_delta(&mut thread.delta);
	}
	
	// Try flushing charout explicitely
	let _ = charout.flush();
}

/// 46: Output integer (.)
#[inline(always)]
pub fn inst_output_integer(thread: &mut FungeThread, charout: &mut Stdout) {
	let cell = thread.stack_stack.pop();
	
	// Act as `r` if the write failed
	if let Err(_) = write!(charout, "{} ", cell as i32) { // As per spec write a space after the decimal number
		// Reflect delta
		_reflect_delta(&mut thread.delta);
	}
	
	// Try flushing charout explicitely
	let _ = charout.flush();
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

/// 60: Go west (<)
#[inline(always)]
pub fn inst_go_west(thread: &mut FungeThread) {
	// Set delta
	thread.delta = InstructionDelta::new_xyz(-1, 0, 0);
}

/// 62: Go east (>)
#[inline(always)]
pub fn inst_go_east(thread: &mut FungeThread) {
	// Set delta
	thread.delta = InstructionDelta::new_xyz(1, 0, 0);
}

/// 94: Go north (^)
#[inline(always)]
pub fn inst_go_north(thread: &mut FungeThread, dialect: FungeDialect) -> bool {
	return match dialect {
		FungeDialect::Unefunge98 => false,
		FungeDialect::Befunge93 | FungeDialect::Befunge98 | FungeDialect::Trefunge98 => {
			// Set delta
			thread.delta = InstructionDelta::new_xyz(0, -1, 0);
			true
		},
	};
}

/// 118: Go south (v)
#[inline(always)]
pub fn inst_go_south(thread: &mut FungeThread, dialect: FungeDialect) -> bool {
	return match dialect {
		FungeDialect::Unefunge98 => false,
		FungeDialect::Befunge93 | FungeDialect::Befunge98 | FungeDialect::Trefunge98 => {
			// Set delta
			thread.delta = InstructionDelta::new_xyz(0, 1, 0);
			true
		},
	};
}

/// 63: Go away (?)
#[inline(always)]
pub fn inst_go_away(thread: &mut FungeThread, dialect: FungeDialect) {
	let new_delta: InstructionDelta;
	let mut rng = rand::thread_rng();
	
	// Create new random delta
	match dialect {
		// 1D
		FungeDialect::Unefunge98 => {
			const DIRS: [i32; 2] = [-1, 1];
			
			let dir = DIRS[rng.gen_range(0usize, DIRS.len())];
			new_delta = InstructionDelta::new_xyz(dir, 0, 0);
		}
		// 2D
		FungeDialect::Befunge93 | FungeDialect::Befunge98 => {
			const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
			
			let dir = DIRS[rng.gen_range(0usize, DIRS.len())];
			new_delta = InstructionDelta::new_xyz(dir.0, dir.1, 0);
		}
		// 3D
		FungeDialect::Trefunge98 => {
			const DIRS: [(i32, i32, i32); 6] = [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];
			
			let dir = DIRS[rng.gen_range(0usize, DIRS.len())];
			new_delta = InstructionDelta::new_xyz(dir.0, dir.1, dir.2);
		}
	}
	
	// Assign delta to thread
	thread.delta = new_delta;
}

/// 96: Greater than (`)
#[inline(always)]
pub fn inst_greater_than(thread: &mut FungeThread) {
	let (first, second) = thread.stack_stack.pop_two();
	thread.stack_stack.push(if second > first { 1 } else { 0 });
}

/// 36: Pop ($)
#[inline(always)]
pub fn inst_pop(thread: &mut FungeThread) {
	// Pop value from cell and discard
	thread.stack_stack.pop();
}

/// 92: Swap (\)
#[inline(always)]
pub fn inst_swap(thread: &mut FungeThread) {
	let ss = &mut thread.stack_stack;
	let (a, b) = ss.pop_two();
	ss.push(a);
	ss.push(b);
}

#[inline(always)]
pub fn _pop_vector(toss: &mut FungeStack, dialect: FungeDialect) -> FungeAddress {
	let vector = match dialect {
		// 1D
		FungeDialect::Unefunge98 => {
			let x = toss.pop();
			FungeAddress::new_xyz(x, 0, 0)
		}
		// 2D
		FungeDialect::Befunge93 | FungeDialect::Befunge98 => {
			let y = toss.pop();
			let x = toss.pop();
			FungeAddress::new_xyz(x, y, 0)
		}
		// 3D
		FungeDialect::Trefunge98 => {
			let z = toss.pop();
			let y = toss.pop();
			let x = toss.pop();
			FungeAddress::new_xyz(x, y, z)
		}
	};
	return vector;
}

/// 103: Get (g)
#[inline(always)]
pub fn inst_get<N, A>(thread: &mut FungeThread, funge_space: &mut FungeSpace<N, i32, A>, dialect: FungeDialect) 
where N: FungeDimension, A: FungeSpaceAccessor<N, i32> {
	let storage_offset = thread.get_storage_offset();
	
	// Pop vector
	let toss = thread.stack_stack.top_stack();
	let mut position = _pop_vector(toss, dialect);
	
	// Add storage offset
	position.add_wrapping(&storage_offset);
	
	// Read and push cell
	let cell = funge_space.read_cell(&position);
	toss.push(cell);
}

/// 110: Clear stack (n)
#[inline(always)]
pub fn inst_clear_stack(thread: &mut FungeThread) {
	let toss = thread.stack_stack.top_stack();
	toss.clear();
}

/// 112: Put (p)
#[inline(always)]
pub fn inst_put<N, A>(thread: &mut FungeThread, funge_space: &mut FungeSpace<N, i32, A>, dialect: FungeDialect) 
where N: FungeDimension, A: FungeSpaceAccessor<N, i32> {
	let storage_offset = &thread.get_storage_offset();
	
	// Pop vector
	let toss = thread.stack_stack.top_stack();
	let mut position = _pop_vector(toss, dialect);
	
	// Add storage offset
	position.add_wrapping(storage_offset);
	
	// Pop and write value
	let value = toss.pop();
	funge_space.write_cell(&position, value);
}

pub fn _reflect_delta(delta: &mut InstructionPointer) {
	// Reflect delta
	delta.set_x(-delta.x());
	delta.set_y(-delta.y());
	delta.set_z(-delta.z());
}

/// 114: Reflect (r)
#[inline(always)]
pub fn inst_reflect(thread: &mut FungeThread) {
	_reflect_delta(&mut thread.delta);
}

/// 120: Absolute delta (x)
#[inline(always)]
pub fn inst_absolute_delta(thread: &mut FungeThread, dialect: FungeDialect) {
	// Pop delta vector
	let new_delta: InstructionDelta = _pop_vector(thread.stack_stack.top_stack(), dialect);
	
	// Assign new delta to thread
	thread.delta = new_delta;
}

