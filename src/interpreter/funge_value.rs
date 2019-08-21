
/// A value that can be stored in a funge cell.
pub trait FungeValue: Copy + Clone + Default {}

impl<T> FungeValue for T where T: Copy + Clone + Default {}
