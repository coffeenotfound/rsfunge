use crate::interpreter::{FungeThread, InstructionDelta, FungeSpace, FungeAddress, FungeDimension, FungeSpaceAccessor, FungeStack, InstructionPointer, RSFUNGE_HANDPRINT, RSFUNGE_VERSION};
use rand::Rng;
use std::io::{Stdout, Stdin, Read, Write};
use chrono::{DateTime, Local, Datelike, Timelike};

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

/// 115: Store character (s)
#[inline(always)]
pub fn inst_store_character<N, A>(thread: &mut FungeThread, funge_space: &mut FungeSpace<N, i32, A>) where N: FungeDimension, A: FungeSpaceAccessor<N, i32> {
	// Get next pos
	let mut pos: FungeAddress = thread.ip;
	pos.add_wrapping(&thread.delta);
	
	// Pop char value
	let value = thread.stack_stack.pop();
	
	// Store in funge space
	funge_space.write_cell(&pos, value);
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
pub fn inst_go_north(thread: &mut FungeThread, dims: u32) -> bool {
	match dims {
		1 => return false,
		2 | 3 => {
			// Set delta
			thread.delta = InstructionDelta::new_xyz(0, -1, 0);
			return true;
		},
		_ => return false,
	}
}

/// 118: Go south (v)
#[inline(always)]
pub fn inst_go_south(thread: &mut FungeThread, dims: u32) -> bool {
	match dims {
		1 => return false,
		2 | 3 => {
			// Set delta
			thread.delta = InstructionDelta::new_xyz(0, 1, 0);
			return true;
		},
		_ => return false,
	}
}

/// 95: East west if (_)
#[inline(always)]
pub fn inst_east_west_if(thread: &mut FungeThread) {
	let value = thread.stack_stack.pop();
	
	// If zero, go east (>)
	if value == 0 {
		thread.delta = InstructionDelta::new_xyz(1, 0, 0);
	}
	// If non-zero, go west (<)
	else {
		thread.delta = InstructionDelta::new_xyz(-1, 0, 0);
	}
}

/// 124: North south if (|)
#[inline(always)]
pub fn inst_north_south_if(thread: &mut FungeThread, dims: u32) -> bool {
	match dims {
		1 => return false,
		2 | 3 => {
			let value = thread.stack_stack.pop();
			
			// If zero, go south (v)
			if value == 0 {
				thread.delta = InstructionDelta::new_xyz(0, 1, 0);
			}
			// If non-zero, go north (^)
			else {
				thread.delta = InstructionDelta::new_xyz(0, -1, 0);
			}
			return true;
		}
		_ => return false,
	}
}

/// 63: Go away (?)
#[inline(always)]
pub fn inst_go_away(thread: &mut FungeThread, dims: u32) {
	let new_delta: InstructionDelta;
	let mut rng = rand::thread_rng();
	
	// Create new random delta
	match dims {
		1 => {
			const DIRS: [i32; 2] = [-1, 1];
			
			let dir = DIRS[rng.gen_range(0usize, DIRS.len())];
			new_delta = InstructionDelta::new_xyz(dir, 0, 0);
		}
		2 => {
			const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
			
			let dir = DIRS[rng.gen_range(0usize, DIRS.len())];
			new_delta = InstructionDelta::new_xyz(dir.0, dir.1, 0);
		}
		3 => {
			const DIRS: [(i32, i32, i32); 6] = [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];
			
			let dir = DIRS[rng.gen_range(0usize, DIRS.len())];
			new_delta = InstructionDelta::new_xyz(dir.0, dir.1, dir.2);
		}
		_ => unimplemented!(),
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
pub fn _pop_vector(toss: &mut FungeStack, dims: u32) -> FungeAddress {
	let vector = match dims {
		// 1D
		1 => {
			let x = toss.pop();
			FungeAddress::new_xyz(x, 0, 0)
		}
		// 2D
		2 => {
			let y = toss.pop();
			let x = toss.pop();
			FungeAddress::new_xyz(x, y, 0)
		}
		// 3D
		3 => {
			let z = toss.pop();
			let y = toss.pop();
			let x = toss.pop();
			FungeAddress::new_xyz(x, y, z)
		}
		_ => unimplemented!(),
	};
	return vector;
}

/// 103: Get (g)
#[inline(always)]
pub fn inst_get<N, A>(thread: &mut FungeThread, funge_space: &mut FungeSpace<N, i32, A>, dims: u32) 
where N: FungeDimension, A: FungeSpaceAccessor<N, i32> {
	let storage_offset = thread.get_storage_offset();
	
	// Pop vector
	let toss = thread.stack_stack.top_stack();
	let mut position = _pop_vector(toss, dims);
	
	// Add storage offset
	position.add_wrapping(&storage_offset);
	
	// Read and push cell
	let cell = funge_space.read_cell(&position);
	toss.push(cell);
}

/// 106: Jump forward (j)
#[inline(always)]
pub fn inst_jump_forward(thread: &mut FungeThread) {
	// Note that the ip is moved by this instruction but it is also incremented
	// normally after the instruction is executed, this is per spec!
	
	let count = thread.stack_stack.pop();
	
	// Move ip
	let d = thread.delta;
	let ip = &mut thread.delta;
	ip.set_x(ip.x().wrapping_add(d.x().wrapping_mul(count)));
	ip.set_y(ip.y().wrapping_add(d.y().wrapping_mul(count)));
	ip.set_z(ip.z().wrapping_add(d.z().wrapping_mul(count)));
}

/// 104: Go high (h)
#[inline(always)]
pub fn inst_go_high(thread: &mut FungeThread, dims: u32) -> bool {
	return match dims {
		3 => {
			// Set delta
			thread.delta = InstructionDelta::new_xyz(0, 0, 1);
			true
		}
		_ => false,
	};
}

/// 108: Go low (l)
#[inline(always)]
pub fn inst_go_low(thread: &mut FungeThread, dims: u32) -> bool {
	match dims {
		3 => {
			// Set delta
			thread.delta = InstructionDelta::new_xyz(0, 0, -1);
			return true;
		}
		_ => return false,
	}
}

/// 109: High low if (m)
#[inline(always)]
pub fn inst_high_low_if(thread: &mut FungeThread, dims: u32) -> bool {
	match dims {
		3 => {
			let value = thread.stack_stack.pop();
			
			// If zero, act like go low (l)
			if value == 0 {
				thread.delta = InstructionDelta::new_xyz(0, 0, -1);
			}
			// If non-zero, act like go high (h)
			else {
				thread.delta = InstructionDelta::new_xyz(0, 0, 1);
			}
			return true;
		}
		_ => return false,
	}
}

/// 110: Clear stack (n)
#[inline(always)]
pub fn inst_clear_stack(thread: &mut FungeThread) {
	let toss = thread.stack_stack.top_stack();
	toss.clear();
}

/// 112: Put (p)
#[inline(always)]
pub fn inst_put<N, A>(thread: &mut FungeThread, funge_space: &mut FungeSpace<N, i32, A>, dims: u32) 
where N: FungeDimension, A: FungeSpaceAccessor<N, i32> {
	let storage_offset = &thread.get_storage_offset();
	
	// Pop vector
	let toss = thread.stack_stack.top_stack();
	let mut position = _pop_vector(toss, dims);
	
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

pub fn _rotate_delta_clockwise_90(delta: &mut InstructionDelta) {
	let d = delta;
	*d = InstructionDelta::new_xyz(-d.y(), d.x(), d.z());
}

pub fn _rotate_delta_counterclockwise_90(delta: &mut InstructionDelta) {
	let d = delta;
	*d = InstructionDelta::new_xyz(d.y(), -d.x(), d.z());
}

/// 91: Turn left ([)
#[inline(always)]
pub fn inst_turn_left(thread: &mut FungeThread, dims: u32) -> bool {
	match dims {
		2 | 3 => {
			// Rotate delta 90° counter-clockwise
			_rotate_delta_counterclockwise_90(&mut thread.delta);
			return true;
		}
		_ => return false,
	}
}

/// 93: Turn left (])
#[inline(always)]
pub fn inst_turn_right(thread: &mut FungeThread, dims: u32) -> bool {
	match dims {
		2 | 3 => {
			// Rotate delta 90° clockwise
			_rotate_delta_clockwise_90(&mut thread.delta);
			return true;
		}
		_ => return false,
	}
}

/// 119: Compare (w)
#[inline(always)]
pub fn inst_compare(thread: &mut FungeThread, dims: u32) -> bool {
	match dims {
		2 | 3 => {
			let (b, a) = thread.stack_stack.pop_two();
			
			// If a < b, act like turn left ([)
			if a < b {
				// Rotate delta 90° counter-clockwise
				_rotate_delta_counterclockwise_90(&mut thread.delta);
			}
			// If a > b, act like turn right (])
			else if a > b {
				// Rotate delta 90° clockwise
				_rotate_delta_clockwise_90(&mut thread.delta);
			}
			// Else don't change the delta
			
			return true;
		}
		_ => return false,
	}
}

/// 120: Absolute delta (x)
#[inline(always)]
pub fn inst_absolute_delta(thread: &mut FungeThread, dims: u32) {
	// Pop delta vector
	let new_delta: InstructionDelta = _pop_vector(thread.stack_stack.top_stack(), dims);
	
	// Assign new delta to thread
	thread.delta = new_delta;
}

#[inline(always)]
pub fn _get_sysinfo_cell_num(thread: &mut FungeThread, dims: u32, stack_num: u32, env_var_string: &[u8], cli_arg_string: &[u8]) -> u32 {
	let base_size = [17, 22, 25][dims as usize];
	let size = base_size + stack_num + env_var_string.len() as u32 + cli_arg_string.len() as u32;
	return size;
}

#[inline(always)]
pub fn _get_sysinfo_cell<N: FungeDimension, A: FungeSpaceAccessor<N, i32>>(thread: &mut FungeThread, index: u32, dims: u32, original_toss_depth: u32, stack_num: u32, env_var_string: &[u8], cli_arg_string: &[u8]) -> i32 {
	return match (index, dims) {
		(0, _) => 0, /* flags (env) */
		(1, _) => 4, /* num bytes per cell (global env) */
		(2, _) => RSFUNGE_HANDPRINT as i32, /* implementation handprint (env) */
		(3, _) => RSFUNGE_VERSION as i32, /* implementation version number (env) */
		(4, _) => 0, /* operating paradigm (for = instruction) (global env) */ // TODO: Return the right code, according to the execute_call_mode
		(5, _) => '/' as i32, /* path seperator char (global env) */
		(6, _) => A::dimensionality() as i32, /* dimensionality or number of cells per vector (global env) */
		(7, _) => 0, /* locally unique id for the current thread (ip) */ // TODO: Implement properly
		(8, _) => 0, /* unique team number for the current thread (not applicable to rsfunge) (ip) */
		
		(i @ 9, 1) |
		(i @ 9..=10, 2) |
		(i @ 9..=11, 3) => thread.ip.elements[i as usize], /* ip of the current thread (ip) */
		
		(i @ 10, 1) |
		(i @ 11..=12, 2) |
		(i @ 12..=14, 3) => thread.delta.elements[i as usize], /* delta of the curren thread (ip) */
		
		(i @ 11, 1) |
		(i @ 13..=14, 2) |
		(i @ 15..=17, 3) => thread.get_storage_offset().elements[i as usize], /* storage offset of the current thread (ip) */
		
		(_i @ 12, 1) |
		(_i @ 15..=16, 2) |
		(_i @ 18..=20, 3) => 0, /* least point which contains a non-space cell, relative to origin (env) */
		
		(_i @ 13, 1) |
		(_i @ 17..=18, 2) |
		(_i @ 19..=21, 3) => 0, /* greatest point which contains a non-space cell, relative to the least point (env) */ // TODO: Implement calc routine (go through all funge space pages and search locally in them the least, greatest point)s
		
		(14, 1) |
		(19, 2) |
		(22, 3) => { /* current ((year - 1900) * 256 * 256) + (month * 256) + (day of month) (env) */
			// Calculate timestamp
			let time: DateTime<Local> = chrono::Local::now();
			let timestamp: u32 = ((time.year() as u32 - 1900) * 256 * 256) + (time.month() as u32 * 256) + (time.day() as u32); // Use 1-based indexing for month and day
			timestamp as i32
		}
		
		(15, 1) |
		(20, 2) |
		(23, 3) => { /* current (hour * 256 * 256) + (minute * 256) + (second) (env) */
			// Get time
			let time: DateTime<Local> = chrono::Local::now();
			let result: u32 = (time.hour() as u32 * 256 * 256) + (time.minute() as u32 * 256) + (time.second() as u32); // 0-based "indexing" for hours, minutes, seconds. Makes sense but it somehow feels wrong considering the date uses 1-based indexing
			result as i32
		}
		
		(16, 1) |
		(21, 2) |
		(24, 3) => {
			stack_num as i32 /* number of stacks on the stack stack (ip) */
		}
		
		_ => (|| -> i32 {
			// Stack size cells
			let local_index = index - [17, 22, 25][dims as usize];
			if local_index < stack_num {
				let stack = thread.stack_stack.nth_stack(local_index).unwrap();
				return stack.depth() as i32;
			}
			
			// Cli arg string
			let local_index = local_index - stack_num;
			if local_index < cli_arg_string.len() as u32 {
				return cli_arg_string[local_index as usize] as i32;
			}
			
			// Env var string
			let local_index = local_index - cli_arg_string.len() as u32;
			if local_index < env_var_string.len() as u32 {
				return env_var_string[local_index as usize] as i32;
			}
			
			// Else, shouldn't be reached
			unimplemented!("Should never be reached");
		})(),
	};
}

/// 121: Get sysinfo (y)
#[inline(always)]
pub fn inst_get_sysinfo<N: FungeDimension, A: FungeSpaceAccessor<N, i32>>(thread: &mut FungeThread, env_var_string: &[u8], cli_arg_string: &[u8]) {
	// The given index: Zero or negative for everything, else the 1-based cell number
	let nth_cell = thread.stack_stack.pop();
	
	let dims = A::dimensionality();
	let toss_depth = thread.stack_stack.top_stack().depth();
	let stack_num = thread.stack_stack.num_stacks();
	
	// Push only specific (one-indexed) cell
	if nth_cell > 0 {
		let syscell_num = _get_sysinfo_cell_num(thread, dims, stack_num, env_var_string, cli_arg_string);
		
		// Use specific sysinfo cell
		if nth_cell <= syscell_num as i32 {
			let cell = _get_sysinfo_cell::<N, A>(thread, nth_cell as u32 - 1, dims, toss_depth, stack_num, env_var_string, cli_arg_string);
			thread.stack_stack.push(cell);
		}
		// If index larger than sysinfo cell num, pick from toss
		else {
			let cell = thread.stack_stack.top_stack().peek_nth((nth_cell as u32 - 1) - syscell_num);
			thread.stack_stack.push(cell.unwrap_or(0));
		}
	}
	// Push all sysinfo cells
	else {
		let syscell_num = _get_sysinfo_cell_num(thread, dims, stack_num, env_var_string, cli_arg_string);
		
		// Go through all sysinfo cells in reverse so that they are on the stack in the right order
		for i in 0..syscell_num {
			let cell = _get_sysinfo_cell::<N, A>(thread, (syscell_num - 1) - i, dims, toss_depth, stack_num, env_var_string, cli_arg_string);
			thread.stack_stack.push(cell);
		}
	}
}

/// 126: Input character (~)
#[inline(always)]
pub fn inst_input_character(thread: &mut FungeThread, charin: &mut Stdin) {
	// TODO: Implement charset support, for now just use ascii
	
	// Read one byte (one ascii char)
	let mut read_buffer: [u8; 1] = [0; 1];
	let read_res = charin.read_exact(&mut read_buffer);
	
	if let Ok(_) = read_res {
		// Push read char onto toss
		let char: i32 = read_buffer[0] as i32;
		thread.stack_stack.push(char);
	}
	// On read error (including end of file/pipe) act as reflect
	else {
		// Reflect delta
		_reflect_delta(&mut thread.delta);
	}
}

/// 117: Stack under stack (u)
#[inline(always)]
pub fn inst_stack_under_stack(thread: &mut FungeThread) {
	// Check if we have a soss
	if let Some(_) = thread.stack_stack.second_stack() {
		// Pop count from toss
		let count = thread.stack_stack.pop();
		
		// Transfer cells in reverse order via pop-push loop
		if count >= 0 {
			for _ in 0..count {
				let val = thread.stack_stack.second_stack().unwrap().pop();
				thread.stack_stack.top_stack().push(val);
			}
		}
		else {
			for _ in 0..count {
				let val = thread.stack_stack.top_stack().pop();
				thread.stack_stack.second_stack().unwrap().push(val);
			}
		}
	}
	// If we don't have a soss (only one stack is on the stack stack), act like reflect (r)
	else {
		_reflect_delta(&mut thread.delta);
	}
}

/// 123: Begin block ({)
#[inline(always)]
pub fn inst_begin_block(thread: &mut FungeThread, dims: u32) {
	let n = thread.stack_stack.pop();
	
	// Get storage offset
	let current_storage_offset: FungeAddress = thread.get_storage_offset();
	
	// Allocate new stack
	let mut new_toss = FungeStack::<i32>::new();
	
	// If n > 0, transfer |n| elements from soss to new toss in non-reversed order
	if n > 0 {
		let fsoss = thread.stack_stack.top_stack();
		fsoss.transfer_to_stack(&mut new_toss, n as u32);
	}
	// If n < 0, push |n| zeros onto soss
	else if n < 0 {
		let fsoss = thread.stack_stack.top_stack(); // Current toss is future soss
		
		for _ in 0..(-n) {
			fsoss.push(0);
		}
	}
	// If n == 0, don't transfer any elements
	else {}
	
	// Push storage offset onto soss
	let fsoss = thread.stack_stack.top_stack();
	match dims {
		1 => {
			fsoss.push(current_storage_offset.x()); // x
		}
		2 => {
			fsoss.push(current_storage_offset.y()); // y
			fsoss.push(current_storage_offset.x()); // x
		}
		3 => {
			fsoss.push(current_storage_offset.z()); // z
			fsoss.push(current_storage_offset.y()); // y
			fsoss.push(current_storage_offset.x()); // x
		}
		_ => unimplemented!(),
	}
	
	// Push new toss onto the stack stack
	thread.stack_stack.push_stack(new_toss);
	
	// Set new storage offset
	let mut new_storage_offset: InstructionPointer = thread.ip;
	new_storage_offset.add_delta_wrapping(&thread.delta);
	thread.stroage_offset = new_storage_offset;
}

