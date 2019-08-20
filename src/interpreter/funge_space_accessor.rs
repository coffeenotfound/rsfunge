use crate::interpreter::{FungeDimension, FungeValue, FungeDim2, FungeDim3};
use std::marker::PhantomData;

pub trait FungeSpaceAccessor<N, V> where N: FungeDimension, V: FungeValue {
	fn get_page_width() -> u32;
	
	fn get_page_capacity() -> u32;
	
	fn initial_value() -> V;
}

// Two dimensional
const DEFAULT_PAGE_WIDTH_DIM2: u32 = 32;

pub struct SpaceAccessorDim2<V> {
	_phantom: PhantomData<(V)>,
}
impl<V> FungeSpaceAccessor<FungeDim2, V> for SpaceAccessorDim2<V> where V: FungeValue {
	#[inline(always)]
	fn get_page_width() -> u32 {
		DEFAULT_PAGE_WIDTH_DIM2
	}
	
	#[inline(always)]
	fn get_page_capacity() -> u32 {
		DEFAULT_PAGE_WIDTH_DIM2*DEFAULT_PAGE_WIDTH_DIM2
	}
	
	#[inline(always)]
	fn initial_value() -> V {
		V::default()
	}
}

// Three dimensional
const DEFAULT_PAGE_WIDTH_DIM3: u32 = 16;

pub struct SpaceAccessorDim3<V> {
	_phantom: PhantomData<(V)>,
}
impl<V> FungeSpaceAccessor<FungeDim3, V> for SpaceAccessorDim3<V> where V: FungeValue {
	#[inline(always)]
	fn get_page_width() -> u32 {
		DEFAULT_PAGE_WIDTH_DIM3
	}
	
	#[inline(always)]
	fn get_page_capacity() -> u32 {
//		let w = Self::get_page_width();
//		return w*w*w;
		DEFAULT_PAGE_WIDTH_DIM3*DEFAULT_PAGE_WIDTH_DIM3*DEFAULT_PAGE_WIDTH_DIM3
	}
	
	#[inline(always)]
	fn initial_value() -> V {
		V::default()
	}
}
