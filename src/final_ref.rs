use std::ops::Deref;

pub struct Final<T> {
	value: T,
}

impl<T> Deref for Final<T> {
	type Target = T;
	
	fn deref(&self) -> &Self::Target {
		&self.value
	}
}
