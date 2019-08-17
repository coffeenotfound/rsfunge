use typenum;

pub trait FungeDimension: typenum::Unsigned {}
impl<T> FungeDimension for T where T: typenum::Unsigned {}
