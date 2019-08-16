
/// Formats the elements in the given list slice into
/// a humanized list and returns it as a String. The elements
/// are converted to string via the ToString trait.
/// 
/// Example: `[1, 2, 3, 4]` will be formatted as `"1, 2, 3 and 4"`.
pub fn format_humaized_list <S: ToString>(list: &[S]) -> String {
	let mut result = String::new();
	let last_index = list.len() - 1;
	
	for (i, elem) in list.iter().enumerate() {
		result += match i {
			0 => "",
			x if x == last_index => " and ",
			_ => ", ",
		};
		result += elem.to_string().as_str();
	}
	return result;
}
