
#[macro_export]
macro_rules! ternary {
	($condition:expr => $positive:expr, $negative:expr) => {
		if $condition { $positive } else { $negative }
	}
}
