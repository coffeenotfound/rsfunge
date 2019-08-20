use typenum;

pub trait FungeDimension {
	type Dimension: typenum::Unsigned;
}

// One dimensional
pub struct FungeDim1;
impl FungeDimension for FungeDim1 {
	type Dimension = typenum::U1;
}

// Two dimensional
pub struct FungeDim2;
impl FungeDimension for FungeDim2 {
	type Dimension = typenum::U1;
}

// Three dimensional
pub struct FungeDim3;
impl FungeDimension for FungeDim3 {
	type Dimension = typenum::U1;
}
