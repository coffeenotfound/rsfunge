use crate::interpreter::{FungeDimension, FungeValue, FungeDim2, FungeDim3, FungeAddress, FungePageAddress};
use std::marker::PhantomData;

pub trait FungeSpaceAccessor<N, V> where N: FungeDimension, V: FungeValue {
	fn get_page_width() -> u32;
	
	fn get_page_capacity() -> u32;
	
	fn localize_address(address: &FungeAddress) -> FungeAddress;
	
	fn address_to_page_linear_index(local_address: &FungeAddress) -> usize;
	
	fn make_page_address(address: &FungeAddress) -> FungePageAddress;
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
	
	fn localize_address(address: &FungeAddress) -> FungeAddress {
		let page_width = Self::get_page_width() as i32;
		
		let mut local = FungeAddress::new();
		local.set_x(address.x().rem_euclid(page_width));
		local.set_y(address.y().rem_euclid(page_width));
		return local;
	}
	
	fn address_to_page_linear_index(local_address: &FungeAddress) -> usize {
		let page_width = Self::get_page_width() as usize;
		
		return (local_address.x() as usize)
			+ (local_address.y() as usize * page_width);
	}
	
	fn make_page_address(address: &FungeAddress) -> FungePageAddress {
		let page_width = Self::get_page_width() as i32;
		
		let mut page_address = FungeAddress::new();
		page_address.set_x(address.x().div_euclid(page_width));
		page_address.set_y(address.y().div_euclid(page_width));
		return page_address;
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
	
	fn localize_address(address: &FungeAddress) -> FungeAddress {
		let page_width = Self::get_page_width() as i32;
		
		let mut local = FungeAddress::new();
		local.set_x(address.x().rem_euclid(page_width));
		local.set_y(address.y().rem_euclid(page_width));
		local.set_y(address.z().rem_euclid(page_width));
		return local;
	}
	
	fn address_to_page_linear_index(local_address: &FungeAddress) -> usize {
		let page_width = Self::get_page_width() as usize;
		return (local_address.x() as usize)
			+ (local_address.y() as usize * page_width)
			+ (local_address.z() as usize * page_width * page_width);
	}
	
	fn make_page_address(address: &FungeAddress) -> FungePageAddress {
		let page_width = Self::get_page_width() as i32;
		
		let mut page_address = FungeAddress::new();
		page_address.set_x(address.x().div_euclid(page_width));
		page_address.set_y(address.y().div_euclid(page_width));
		page_address.set_z(address.z().div_euclid(page_width));
		return page_address;
	}
}
