use crate::vector::Vector3;
use std::collections::HashMap;
use crate::interpreter::{FungeAddress, FungeDimension};
use std::marker::PhantomData;

pub struct FungeSpace<N, V = i32> where N: FungeDimension, V: FungeValue {
	page_map: HashMap<FungeAddress, FungeSpacePage<N, V>>,
	size: Vector3<u32>,
	_phantom: PhantomData<[N]>,
}
impl<N, V> FungeSpace<N, V> where N: FungeDimension, V: FungeValue {
	#[inline(always)]
	fn get_page(&mut self, page_address: FungeAddress) -> &mut FungeSpacePage<N, V> {
		let map = &mut self.page_map;
		
		let page = map.entry(page_address).or_insert_with(|| {
			let page = FungeSpacePage::<N, V>::new();
			return page;
		});
		return page;
	}
}

const FUNGE_PAGE_WIDTH: u32 = 128;

pub struct FungeSpacePage<N, V, A = DefaultSpacePageAccess<N, V>> where N: FungeDimension, V: FungeValue, A: SpacePageAccess<N, V> {
	pub data: Vec<V>,
	_phantom: PhantomData<[N]>,
}
impl<N, V, A> FungeSpacePage<N, V, A> where N: FungeDimension, V: FungeValue, A: SpacePageAccess<N, V> {
	fn new() -> Self {
		let zero_value = V::default();
		let page_cap = Self::get_page_capacity();
//		let page_cap = PageCellAddressCalc::<N>::get_page_capacity();
//		let page_cap = 0;
		
		FungeSpacePage {
			data: vec![zero_value; (page_cap as usize)],
			_phantom: PhantomData,
		}
	}
}

pub trait SpacePageAccess<N, V> where N: FungeDimension, V: FungeValue {
	fn get_page_capacity() -> u32;
}

struct DefaultSpacePageAccess<N, V> where N: FungeDimension, V: FungeValue;
impl<N, V> SpacePageAccess<N, V> for DefaultSpacePageAccess<typenum::U3, V> {
	fn get_page_capacity() -> u32 {
		3
	}
}

//pub trait PageCellAddressCalc<N: FungeDimension> {
//	fn calc_linear_address(address: FungeAddress) -> u32;
//	
//	fn get_page_capacity() -> u32;
//}
//
//impl<N, V> PageCellAddressCalc<N> for FungeSpacePage<typenum::U1, V> where N: FungeDimension, V: FungeValue {
//	fn calc_linear_address(address: Vector3<u32>) -> u32 {
//		address.x()
//	}
//	
//	fn get_page_capacity() -> u32 {
//		FUNGE_PAGE_WIDTH
//	}
//}

//impl<N, V> PageCellAddressCalc<N> for FungeSpacePage<N, V> where N: FungeDimension, V: FungeValue {
//	fn calc_linear_address(address: Vector3<u32>) -> u32 {
//		address.x() + (address.y() * FUNGE_PAGE_WIDTH)
//	}
//	
//	fn get_page_capacity() -> u32 {
//		FUNGE_PAGE_WIDTH * FUNGE_PAGE_WIDTH
//	}
//}
//impl<N, V> PageCellAddressCalc<N> for FungeSpacePage<N, V> where N: FungeDimension, V: FungeValue {
//	fn calc_linear_address(address: Vector3<u32>) -> u32 {
//		address.x() + (address.y() * FUNGE_PAGE_WIDTH) + (address.z() * FUNGE_PAGE_WIDTH * FUNGE_PAGE_WIDTH)
//	}
//	
//	fn get_page_capacity() -> u32 {
//		FUNGE_PAGE_WIDTH * FUNGE_PAGE_WIDTH * FUNGE_PAGE_WIDTH
//	}
//}
//
//impl<N, V> PageCellAddressCalc<N> for FungeSpacePage<N, V> where N: FungeDimension, V: FungeValue {
//	fn calc_linear_address(address: Vector3<u32>) -> u32 {
////		assert!(false, "PageCellAddressCalc is not impl'ed for ")
//		unimplemented!()
//	}
//	
//	fn get_page_capacity() -> u32 {
//		unimplemented!()
//	}
//}

//impl<T, E> PageCellAddressCalc for T where E: Eq<T, typenum::U1> {
//	fn calc_linear_address(address: Vector3<u32>) -> u32 {
//		address.x()
//	}
//}
//impl<T, E> PageCellAddressCalc for T where E: Eq<T, typenum::U2> {
//	fn calc_linear_address(address: Vector3<u32>) -> u32 {
//
//	}
//}


//pub struct FungeSpacePageKey<V: FungeValue> {
//
//}

/// A value that can be stored in a funge cell.
pub trait FungeValue: Copy + Clone + Default {}
impl<T> FungeValue for T where T: Copy + Clone + Default {}

//fn _static_assert() {
//	unsafe {
//		// Asset usize is u32
//		std::mem::transmute::<usize, [u8; 4]>(0);
//	}
//}

