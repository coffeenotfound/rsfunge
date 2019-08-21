use std::collections::HashMap;
use crate::interpreter::{FungeAddress, FungeDimension, FungeSpaceAccessor, FungeValue};
use std::marker::PhantomData;

pub struct FungeSpace<'s, N, V, A> where N: FungeDimension, V: FungeValue, A: FungeSpaceAccessor<N, V> {
	page_map: HashMap<FungeAddress, FungeSpacePage<'s, N, V, A>>,
//	size: Vector3<u32>,
	_unused: PhantomData<(&'s u8, N)>,
}
impl<'s, N, V, A> FungeSpace<'s, N, V, A> where N: FungeDimension, V: FungeValue, A: FungeSpaceAccessor<N, V> {
	#[inline(always)]
	fn get_page(&mut self, page_address: FungeAddress) -> &mut FungeSpacePage<'s, N, V, A> {
		let map = &mut self.page_map;
		
		let page = map.entry(page_address).or_insert_with(|| {
			let page = FungeSpacePage::<N, V, A>::new();
			return page;
		});
		return page;
	}
}

pub struct FungeSpacePage<'s, N, V, A> where N: FungeDimension, V: FungeValue, A: FungeSpaceAccessor<N, V> {
	pub data: Vec<V>,
	_unused: PhantomData<(&'s u8, N, A)>,
}
impl<'s, N, V, A> FungeSpacePage<'s, N, V, A> where N: FungeDimension, V: FungeValue, A: FungeSpaceAccessor<N, V> {
	fn new() -> Self {
		let initial_value = unsafe {
			std::mem::transmute::<i32, V>(32) // Space character (32)
		};
		let page_cap = A::get_page_capacity();
		
		FungeSpacePage {
			data: vec![initial_value; (page_cap as usize)],
			_unused: PhantomData,
		}
	}
}

//pub trait SpacePageAccess<N, V> where N: FungeDimension, V: FungeValue {
//	fn get_page_capacity() -> u32;
//}
//
//struct DefaultSpacePageAccess<N, V> where N: FungeDimension, V: FungeValue;
//impl<N, V> SpacePageAccess<N, V> for DefaultSpacePageAccess<typenum::U3, V> {
//	fn get_page_capacity() -> u32 {
//		3
//	}
//}

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



//fn _static_assert() {
//	unsafe {
//		// Asset usize is u32
//		std::mem::transmute::<usize, [u8; 4]>(0);
//	}
//}

